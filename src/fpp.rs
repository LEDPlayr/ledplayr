use std::{
    ffi::CStr,
    io::Cursor,
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use socket2::{Domain, Protocol, Socket, Type};

use anyhow::{bail, Result};
use tokio::net::UdpSocket;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

/*
* https://github.com/FalconChristmas/fpp/blob/master/docs/ControlProtocol.txt
*/

pub async fn listen(cancel: CancellationToken) {
    let mut ips = Vec::new();
    if let Ok(network_interfaces) = local_ip_address::list_afinet_netifas() {
        network_interfaces.iter().for_each(|(_name, ip)| {
            if ip.is_ipv4() && !ip.is_loopback() {
                ips.push(ip.to_string());
            }
        });
    }

    if ips.is_empty() {
        tracing::error!("No network interfaces found");
        cancel.cancel();
    }

    let tracker = TaskTracker::new();
    for ip in ips.into_iter() {
        let cancel = cancel.clone();

        tracker.spawn(async move {
            match Ipv4Addr::from_str(&ip) {
                Ok(addr) => match listen_thread(addr, cancel.clone()).await {
                    Ok(_) => {}
                    Err(e) => {
                        cancel.cancel();
                        tracing::error!("Error in UDP listener: {e}");
                    }
                },
                Err(e) => {
                    cancel.cancel();
                    tracing::error!("Error parsing IPv4Addr: {e}")
                }
            }
        });
    }
    tracker.close();

    tracker.wait().await;
}

async fn listen_thread(bind: Ipv4Addr, cancel: CancellationToken) -> Result<()> {
    let multi_addr = SocketAddrV4::new(Ipv4Addr::new(239, 70, 80, 80), 32320);
    let addr = SocketAddrV4::new(bind, 32320);
    let all = Ipv4Addr::new(0, 0, 0, 0);
    let all_sock = SocketAddrV4::new(all, 32320);

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

    socket.set_reuse_address(true)?;
    socket.bind(&socket2::SockAddr::from(all_sock))?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(multi_addr.ip(), &all)?;
    socket.set_nonblocking(true)?;

    tracing::info!("UDP Listening on {} from multicast {}", addr, multi_addr);

    let socket = UdpSocket::from_std(socket.into())?;

    let mut buf = [0; 1024];
    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                return Ok(())
            },
            res = socket.recv_from(&mut buf) => {
                let (_len, from_addr) = res?;
                let mut cur = Cursor::new(buf);

                if let Ok(fpp) = FPP::read(&mut cur) {
                    if fpp.packet_type != PacketType::Ping {
                        continue;
                    }

                    if let PacketBody::Ping(ping) = fpp.body {
                        if ping.sub_type != PingSubType::Discovery {
                            continue;
                        }
                    }

                    tracing::info!("Multicast Discover Received from {} on {}", from_addr, addr);

                    let fpp = FPP {
                        magic: 0x46505044,
                        packet_type: PacketType::Ping,
                        body: PacketBody::Ping(Ping {
                            data_len: 200,
                            ping_version: 2,
                            sub_type: PingSubType::Ping,
                            hardware_type: HardwareType::Fpp,
                            major_version: 1,
                            minor_version: 0,
                            operating_mode: OperatingMode {
                                bridge: false,
                                player: true,
                                multisync: false,
                                remote: false,
                            },
                            ip_address: bind,
                            hostname: "localhost".to_string(),
                            version: "RSP".to_string(),
                            hardware: "Fedora".to_string(),
                            channels: "".to_string(),
                        }),
                    };

                    let mut buf = Vec::new();
                    fpp.write(&mut buf)?;
                    match socket.send_to(buf.as_slice(), from_addr).await {
                        Ok(_) => {
                            tracing::info!("Replied to Multicast Discovery");
                        }
                        Err(e) => {
                            tracing::error!("Failed to reply to Multicast Discovery: {}", e);
                        }
                    }
                }
            },
        }
    }
}

trait Reader<R>
where
    R: std::io::Read,
{
    fn read(r: &mut R) -> Result<Self>
    where
        Self: Sized;
}

trait Writer<W>
where
    W: std::io::Write,
{
    fn write(self, w: &mut W) -> Result<()>;
}

#[derive(Debug, PartialEq)]
pub struct FPP {
    magic: u32,
    packet_type: PacketType,
    body: PacketBody,
}

impl<R> Reader<R> for FPP
where
    R: std::io::Read,
{
    fn read(r: &mut R) -> Result<Self> {
        let magic = r.read_u32::<BigEndian>()?;
        if magic != 0x46505044 {
            bail!("Oh no")
        }

        let packet_type = PacketType::read(r)?;
        let body = match packet_type {
            PacketType::Legacy => PacketBody::Legacy,
            PacketType::Multisync => PacketBody::Multisync,
            PacketType::Event => PacketBody::Event,
            PacketType::Blanking => PacketBody::Blanking,
            PacketType::Ping => PacketBody::Ping(Ping::read(r)?),
            PacketType::Plugin => PacketBody::Plugin,
            PacketType::Command => PacketBody::Command,
            PacketType::Unknown => bail!("Unknown packet type"),
        };

        Ok(FPP {
            magic,
            packet_type,
            body,
        })
    }
}

impl<W> Writer<W> for FPP
where
    W: std::io::Write,
{
    fn write(self, w: &mut W) -> Result<()> {
        w.write_u32::<BigEndian>(self.magic)?;
        self.packet_type.write(w)?;
        if let PacketBody::Ping(ping) = self.body {
            ping.write(w)?
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum PacketBody {
    Legacy,
    Multisync,
    Event,
    Blanking,
    Ping(Ping),
    Plugin,
    Command,
}

#[derive(Debug, PartialEq)]
pub struct Ping {
    data_len: u16,
    ping_version: u8,
    sub_type: PingSubType,
    hardware_type: HardwareType,
    major_version: u16,
    minor_version: u16,
    operating_mode: OperatingMode,
    ip_address: Ipv4Addr,
    hostname: String,
    version: String,
    hardware: String,
    channels: String,
}

impl<R> Reader<R> for Ping
where
    R: std::io::Read,
{
    fn read(r: &mut R) -> Result<Self> {
        let mut hostname = [0u8; 65];
        let mut version = [0u8; 41];
        let mut hardware = [0u8; 41];
        let mut channels2 = [0u8; 41];
        let mut channels3 = [0u8; 121];

        let data_len = r.read_u16::<LittleEndian>()?;
        let ping_version = r.read_u8()?;
        let sub_type = PingSubType::read(r)?;
        let hardware_type = HardwareType::read(r)?;
        let major_version = r.read_u16::<BigEndian>()?;
        let minor_version = r.read_u16::<BigEndian>()?;
        let operating_mode = OperatingMode::read(r)?;
        let ip_address = r.read_u32::<BigEndian>()?;
        r.read_exact(&mut hostname)?;
        r.read_exact(&mut version)?;
        if ping_version >= 2 {
            r.read_exact(&mut hardware)?;
        }
        if ping_version == 2 {
            r.read_exact(&mut channels2)?;
        } else if ping_version == 3 {
            r.read_exact(&mut channels3)?;
        }

        let ip_address = Ipv4Addr::from(ip_address);
        let hostname = CStr::from_bytes_until_nul(&hostname)?
            .to_string_lossy()
            .clone()
            .to_string();
        let version = CStr::from_bytes_until_nul(&version)?
            .to_string_lossy()
            .clone()
            .to_string();
        let hardware = CStr::from_bytes_until_nul(&hardware)?
            .to_string_lossy()
            .clone()
            .to_string();

        let mut channels = String::new();
        if ping_version == 2 {
            channels = CStr::from_bytes_until_nul(&channels2)?
                .to_string_lossy()
                .clone()
                .to_string();
        } else if ping_version == 3 {
            channels = CStr::from_bytes_until_nul(&channels3)?
                .to_string_lossy()
                .clone()
                .to_string();
        }

        Ok(Ping {
            data_len,
            ping_version,
            sub_type,
            hardware_type,
            major_version,
            minor_version,
            operating_mode,
            ip_address,
            hostname,
            version,
            hardware,
            channels,
        })
    }
}

impl<W> Writer<W> for Ping
where
    W: std::io::Write,
{
    fn write(self, w: &mut W) -> Result<()> {
        let ip_address: u32 = self.ip_address.into();
        let mut hostname = [0u8; 65];
        let mut version = [0u8; 41];
        let mut hardware = [0u8; 41];
        let mut channels = [0u8; 121];

        hostname[..self.hostname.len()].copy_from_slice(self.hostname.as_bytes());
        version[..self.version.len()].clone_from_slice(self.version.as_bytes());
        hardware[..self.hardware.len()].clone_from_slice(self.hardware.as_bytes());
        channels[..self.channels.len()].clone_from_slice(self.channels.as_bytes());

        w.write_u16::<LittleEndian>(self.data_len)?;
        w.write_u8(self.ping_version)?;
        self.sub_type.write(w)?;
        self.hardware_type.write(w)?;
        w.write_u16::<BigEndian>(self.major_version)?;
        w.write_u16::<BigEndian>(self.minor_version)?;
        self.operating_mode.write(w)?;
        w.write_u32::<BigEndian>(ip_address)?;
        w.write_all(&hostname)?;
        w.write_all(&version)?;
        if self.ping_version >= 2 {
            w.write_all(&hardware)?;
        }
        if self.ping_version == 2 {
            w.write_all(&channels[..41])?;
        } else if self.ping_version == 3 {
            w.write_all(&channels)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct OperatingMode {
    bridge: bool,
    player: bool,
    multisync: bool,
    remote: bool,
}

impl From<u8> for OperatingMode {
    fn from(value: u8) -> Self {
        OperatingMode {
            bridge: value & 1 == 1,
            player: value & 2 == 2,
            multisync: value & 4 == 4,
            remote: value & 8 == 8,
        }
    }
}

impl<R> Reader<R> for OperatingMode
where
    R: std::io::Read,
{
    fn read(r: &mut R) -> Result<Self> {
        let value = r.read_u8()?;
        Ok(OperatingMode::from(value))
    }
}

impl<W> Writer<W> for OperatingMode
where
    W: std::io::Write,
{
    fn write(self, w: &mut W) -> Result<()> {
        let mut value = 0;
        if self.bridge {
            value += 1
        }
        if self.player {
            value += 2
        }
        if self.multisync {
            value += 4
        }
        if self.remote {
            value += 8
        }
        w.write_u8(value)?;
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum HardwareType {
    Unknown,
    Fpp,
    PiA,
    PiB,
    PiAPlus,
    PiBPlus,
    Pi2b,
    Pi2bNew,
    Pi3b,
    Pi3bPlus,
    PiZero,
    PiZerow,
    Pi3aPlus,
    Pi4,
    BeagleboneBlackRevB,
    BeagleboneBlackRevC,
    BeagleboneBlackWireless,
    BeagleboneGreen,
    BeagleboneGreenWireless,
    Pocketbeagle,
    SancloudBeagleboneEnhanced,
    Armbian,
    Macos,
    UnknownFalcon,
    F16v2B,
    F4v2_64,
    F16v2Red,
    F4v2Red,
    F16v3,
    F4v3,
    F48,
    F16v4,
    F48v4,
    F16v5,
    F32v5,
    F48v5,
    GeniusPixel16,
    GeniusPixel8,
    GeniusLongRange,
    Other,
    Xdchedule,
    EspixelstickEsp8266,
    EspixelstickEsp32,
    Wled,
    Diyledexpress,
    Hinkspix,
    Alphapix,
    Sandevices,
}

impl From<u8> for HardwareType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::Fpp,
            0x02 => Self::PiA,
            0x03 => Self::PiB,
            0x04 => Self::PiAPlus,
            0x05 => Self::PiBPlus,
            0x06 => Self::Pi2b,
            0x07 => Self::Pi2bNew,
            0x08 => Self::Pi3b,
            0x09 => Self::Pi3bPlus,
            0x10 => Self::PiZero,
            0x11 => Self::PiZerow,
            0x12 => Self::Pi3aPlus,
            0x13 => Self::Pi4,
            0x40 => Self::BeagleboneBlackRevB,
            0x41 => Self::BeagleboneBlackRevC,
            0x42 => Self::BeagleboneBlackWireless,
            0x43 => Self::BeagleboneGreen,
            0x44 => Self::BeagleboneGreenWireless,
            0x45 => Self::Pocketbeagle,
            0x46 => Self::SancloudBeagleboneEnhanced,
            0x60 => Self::Armbian,
            0x70 => Self::Macos,
            0x80 => Self::UnknownFalcon,
            0x81 => Self::F16v2B,
            0x82 => Self::F4v2_64,
            0x83 => Self::F16v2Red,
            0x84 => Self::F4v2Red,
            0x85 => Self::F16v3,
            0x86 => Self::F4v3,
            0x87 => Self::F48,
            0x88 => Self::F16v4,
            0x89 => Self::F48v4,
            0x8A => Self::F16v5,
            0x8B => Self::F32v5,
            0x8C => Self::F48v5,
            0xA0 => Self::GeniusPixel16,
            0xA1 => Self::GeniusPixel8,
            0xA2 => Self::GeniusLongRange,
            0xC0 => Self::Other,
            0xC1 => Self::Xdchedule,
            0xC2 => Self::EspixelstickEsp8266,
            0xC3 => Self::EspixelstickEsp32,
            0xFB => Self::Wled,
            0xFC => Self::Diyledexpress,
            0xFD => Self::Hinkspix,
            0xFE => Self::Alphapix,
            0xFF => Self::Sandevices,
            _ => Self::Unknown,
        }
    }
}

impl<R> Reader<R> for HardwareType
where
    R: std::io::Read,
{
    fn read(r: &mut R) -> Result<Self> {
        let value = r.read_u8()?;
        Ok(HardwareType::from(value))
    }
}

impl<W> Writer<W> for HardwareType
where
    W: std::io::Write,
{
    fn write(self, w: &mut W) -> Result<()> {
        w.write_u8(self as u8)?;
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum PacketType {
    Legacy = 0,
    Multisync = 1,
    Event = 2,
    Blanking = 3,
    Ping = 4,
    Plugin = 5,
    Command = 6,
    Unknown = 255,
}

impl From<u8> for PacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Legacy,
            1 => Self::Multisync,
            2 => Self::Event,
            3 => Self::Blanking,
            4 => Self::Ping,
            5 => Self::Plugin,
            6 => Self::Command,
            _ => Self::Unknown,
        }
    }
}

impl<R> Reader<R> for PacketType
where
    R: std::io::Read,
{
    fn read(r: &mut R) -> Result<Self> {
        let value = r.read_u8()?;
        Ok(PacketType::from(value))
    }
}

impl<W> Writer<W> for PacketType
where
    W: std::io::Write,
{
    fn write(self, w: &mut W) -> Result<()> {
        w.write_u8(self as u8)?;
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum SyncAction {
    Start = 0,
    Stop = 1,
    Sync = 2,
    Open = 3,
    Unknown = 255,
}

impl From<u8> for SyncAction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Start,
            1 => Self::Stop,
            2 => Self::Sync,
            3 => Self::Open,
            _ => Self::Unknown,
        }
    }
}

impl<R> Reader<R> for SyncAction
where
    R: std::io::Read,
{
    fn read(r: &mut R) -> Result<Self> {
        let value = r.read_u8()?;
        Ok(SyncAction::from(value))
    }
}

impl<W> Writer<W> for SyncAction
where
    W: std::io::Write,
{
    fn write(self, w: &mut W) -> Result<()> {
        w.write_u8(self as u8)?;
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum SyncType {
    Fseq = 0,
    Media = 1,
    Unknown = 255,
}

impl From<u8> for SyncType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Fseq,
            1 => Self::Media,
            _ => Self::Unknown,
        }
    }
}

impl<R> Reader<R> for SyncType
where
    R: std::io::Read,
{
    fn read(r: &mut R) -> Result<Self> {
        let value = r.read_u8()?;
        Ok(SyncType::from(value))
    }
}

impl<W> Writer<W> for SyncType
where
    W: std::io::Write,
{
    fn write(self, w: &mut W) -> Result<()> {
        w.write_u8(self as u8)?;
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum PingSubType {
    Ping = 0,
    Discovery = 1,
    Unknown = 255,
}

impl From<u8> for PingSubType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Ping,
            1 => Self::Discovery,
            _ => Self::Unknown,
        }
    }
}

impl<R> Reader<R> for PingSubType
where
    R: std::io::Read,
{
    fn read(r: &mut R) -> Result<Self> {
        let value = r.read_u8()?;
        Ok(PingSubType::from(value))
    }
}

impl<W> Writer<W> for PingSubType
where
    W: std::io::Write,
{
    fn write(self, w: &mut W) -> Result<()> {
        w.write_u8(self as u8)?;
        Ok(())
    }
}
