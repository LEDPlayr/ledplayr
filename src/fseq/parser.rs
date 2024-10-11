use std::{
    fs::File,
    io::{Read, Seek},
    mem,
};

use anyhow::{bail, Context, Result};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::fseq::error::FSeqError;

pub struct FSeq {
    f: File,
    pub filename: String,

    channel_data_offset: u16,
    pub major_version: u8,
    pub minor_version: u8,
    variable_data_offset: u16,
    pub channel_count: u32,
    pub frame_count: u32,
    pub step_time_ms: u8,
    flags: u8,
    pub compression_type: CompressionType,
    compression_block_count: u16,
    sparse_range_count: u8,
    reserved: u8,
    pub uuid: u64,

    compressed_blocks: Vec<CompressedBlock>,
    sparse_ranges: Vec<SparseRange>,
    pub variables: Vec<Variable>,

    cached_first_frame: u32,
    cached_last_frame: u32,
    cached: Vec<u8>,
}

impl std::fmt::Display for FSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FSeq")?;

        writeln!(f, "ChannelDataOffset: {}", self.channel_data_offset)?;
        writeln!(f, "Version: {}.{}", self.major_version, self.minor_version)?;
        writeln!(f, "VariableDataOffset: {}", self.variable_data_offset)?;
        writeln!(f, "ChannelCount: {}", self.channel_count)?;
        writeln!(f, "FrameCount: {}", self.frame_count)?;
        writeln!(f, "StepTimeMS: {}", self.step_time_ms)?;
        writeln!(f, "Flags: {}", self.flags)?;
        writeln!(f, "CompressionType: {:?}", self.compression_type)?;
        writeln!(f, "CompressionBlockCount: {}", self.compression_block_count)?;
        writeln!(f, "SparseRangeCount: {}", self.sparse_range_count)?;
        writeln!(f, "Reserved: {}", self.reserved)?;
        writeln!(f, "UUID: {}", self.uuid)?;

        for fc in &self.compressed_blocks {
            write!(f, "{}", fc)?;
        }

        for sr in &self.sparse_ranges {
            write!(f, "{}", sr)?;
        }

        for v in &self.variables {
            write!(f, "{}", v)?;
        }

        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum CompressionType {
    None = 0,
    Zstd = 1,
    Zlib = 2,
    Unknown(u8) = 255,
}

impl From<u8> for CompressionType {
    fn from(value: u8) -> Self {
        match value {
            0 => CompressionType::None,
            1 => CompressionType::Zstd,
            2 => CompressionType::Zlib,
            x => CompressionType::Unknown(x),
        }
    }
}

impl From<CompressionType> for u8 {
    fn from(value: CompressionType) -> Self {
        match value {
            CompressionType::None => 0,
            CompressionType::Zstd => 1,
            CompressionType::Zlib => 2,
            CompressionType::Unknown(_) => 255,
        }
    }
}

impl std::fmt::Display for CompressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Zstd => f.write_str("zstd"),
            Self::Zlib => f.write_str("Zlib"),
            Self::Unknown(_) => f.write_str("Unknown"),
        }
    }
}

pub struct CompressedBlock {
    first_frame_number: u32,
    last_frame_number: u32,
    size: u32,
}

impl std::fmt::Display for CompressedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CompressedBlock")?;
        writeln!(f, "  FirstFrameNumber: {}", self.first_frame_number)?;
        writeln!(f, "  LastFrameNumber: {}", self.last_frame_number)?;
        writeln!(f, "  Size: {}", self.size)?;
        Ok(())
    }
}

pub struct SparseRange {
    start_channel: u32,
    end_channel_offset: u32,
}

impl std::fmt::Display for SparseRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SparseRange")?;
        writeln!(f, "  StartChannel: {}", self.start_channel)?;
        writeln!(f, "  EndChannelOffset: {}", self.end_channel_offset)?;
        Ok(())
    }
}

pub struct Variable {
    pub code: String,
    pub data: String,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Variable")?;
        writeln!(f, "  Code: {}", self.code)?;
        writeln!(f, "  Data: {}", self.data)?;
        Ok(())
    }
}

pub fn parse(fname: &str) -> Result<Box<FSeq>> {
    let mut f = File::open(fname).context("Couldn't open file")?;

    let magic = f.read_u32::<BigEndian>()?;
    if magic != 0x50534551 {
        bail!(FSeqError::BadMagic)
    }

    let channel_data_offset = f.read_u16::<LittleEndian>()?;
    let minor_version = f.read_u8()?;
    let major_version = f.read_u8()?;
    let variable_data_offset = f.read_u16::<LittleEndian>()?;
    let channel_count = f.read_u32::<LittleEndian>()?;
    let frame_count = f.read_u32::<LittleEndian>()?;
    let step_time_ms = f.read_u8()?;
    let flags = f.read_u8()?;

    let mut compression_block_count = 0u16;

    let tmp = f.read_u8()?;
    compression_block_count += ((tmp & 0xf0) as u16) << 8;
    let ct = CompressionType::from(tmp & 0xf);

    compression_block_count += f.read_u8()? as u16;
    let sparse_range_count = f.read_u8()?;
    let reserved = f.read_u8()?;
    let uuid = f.read_u64::<LittleEndian>()?;

    let mut compressed_blocks = Vec::<CompressedBlock>::new();
    for _ in 0..compression_block_count {
        let first_frame_number = f.read_u32::<LittleEndian>()?;
        let size = f.read_u32::<LittleEndian>()?;

        if size == 0 {
            continue;
        }

        if let Some(cb) = compressed_blocks.last_mut() {
            cb.last_frame_number = first_frame_number - 1;
        }

        compressed_blocks.push(CompressedBlock {
            first_frame_number,
            last_frame_number: 0,
            size,
        });
    }

    if let Some(cb) = compressed_blocks.last_mut() {
        cb.last_frame_number = frame_count;
    }

    let mut sparse_ranges = Vec::new();
    for _ in 0..sparse_range_count {
        let mut start_channel_tmp = [0u8; 4];
        start_channel_tmp[1] = f.read_u8()?;
        start_channel_tmp[2] = f.read_u8()?;
        start_channel_tmp[3] = f.read_u8()?;
        let start_channel;
        unsafe {
            start_channel = mem::transmute::<[u8; 4], u32>(start_channel_tmp);
        }

        let mut end_channel_offset_tmp = [0u8; 4];
        end_channel_offset_tmp[1] = f.read_u8()?;
        end_channel_offset_tmp[2] = f.read_u8()?;
        end_channel_offset_tmp[3] = f.read_u8()?;
        let end_channel_offset;
        unsafe {
            end_channel_offset = mem::transmute::<[u8; 4], u32>(end_channel_offset_tmp);
        }

        sparse_ranges.push(SparseRange {
            start_channel,
            end_channel_offset,
        })
    }

    let mut variables = Vec::new();
    let mut variable_count = channel_data_offset - variable_data_offset;
    while variable_count >= 4 {
        let size = f.read_u16::<LittleEndian>()?;

        let mut code = [0u8; 2];
        code[0] = f.read_u8()?;
        code[1] = f.read_u8()?;
        let code = std::str::from_utf8(&code)?;

        let data_len = size - 4;
        let mut data = Vec::new();
        for _ in 0..data_len {
            data.push(f.read_u8()?);
        }
        data.pop(); // Drop the final null
        let data = std::str::from_utf8(&data)?;

        variables.push(Variable {
            code: code.to_string(),
            data: data.to_string(),
        });

        variable_count -= size;
    }

    while variable_count > 0 {
        _ = f.read_u8()?;
        variable_count -= 1
    }

    if f.stream_position()? != channel_data_offset as u64 {
        bail!(FSeqError::BadVariableBlock);
    }

    Ok(Box::new(FSeq {
        f,
        filename: fname.to_string(),
        channel_data_offset,
        major_version,
        minor_version,
        variable_data_offset,
        channel_count,
        frame_count,
        step_time_ms,
        flags,
        compression_type: ct,
        compression_block_count,
        sparse_range_count,
        reserved,
        uuid,
        compressed_blocks,
        sparse_ranges,
        variables,
        cached_first_frame: 0,
        cached_last_frame: 0,
        cached: Vec::new(),
    }))
}

impl FSeq {
    pub fn get_frame(&mut self, f: u32) -> Result<Option<Vec<u8>>> {
        match self.compression_type {
            CompressionType::None => bail!(FSeqError::UnhandledCompression),
            CompressionType::Zstd => self.get_frame_zstd(f),
            CompressionType::Zlib => bail!(FSeqError::UnhandledCompression),
            CompressionType::Unknown(x) => bail!(FSeqError::UnknownCompression(x)),
        }
    }

    fn get_frame_zstd(&mut self, f: u32) -> Result<Option<Vec<u8>>> {
        if f > self.frame_count - 1 {
            bail!(FSeqError::FrameNotFound)
        }

        // Refill cache if required
        if self.cached_first_frame > f || f > self.cached_last_frame || self.cached.is_empty() {
            // Find the compressed data block
            let mut seek = self.channel_data_offset as u32;
            let mut to_read = 0;

            for cb in self.compressed_blocks.as_slice() {
                if cb.first_frame_number <= f && f <= cb.last_frame_number {
                    self.cached_first_frame = cb.first_frame_number;
                    self.cached_last_frame = cb.last_frame_number;
                    to_read = cb.size;
                    break;
                }
                seek += cb.size;
            }

            // Seek to the compressed block
            self.f.seek(std::io::SeekFrom::Start(seek as u64))?;

            // Read into buffer and decompress
            let mut buf = vec![0u8; to_read as usize];
            self.f.read_exact(&mut buf)?;
            self.cached = zstd::decode_all(buf.as_slice())?;
        }

        // Read just the specific frame back
        let offset = f - self.cached_first_frame;
        let start = (offset * self.channel_count) as usize;
        let end = start + self.channel_count as usize;
        Ok(Some(
            self.cached
                .iter()
                .enumerate()
                .filter(|(i, _x)| start <= *i && *i < end)
                .map(|(_i, x)| *x)
                .collect(),
        ))
    }
}
