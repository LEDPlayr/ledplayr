use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use anyhow::{anyhow, bail, Context, Result};
use rustix::path::Arg;

use crate::{config::Config, fseq, models::Channels};

pub enum StorageType {
    Sequences,
    Media,
    Other,
}

impl Display for StorageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sequences => f.write_str("sequences"),
            Self::Media => f.write_str("media"),
            Self::Other => f.write_str("other"),
        }
    }
}

pub fn init(cfg: &Config) -> Result<()> {
    let path = Path::new(&cfg.storage);

    tracing::debug!("Creating 'sequences' directory");
    let sequence = path.join(StorageType::Sequences.to_string());
    std::fs::create_dir_all(sequence).context("Couldn't create 'sequences' directory")?;

    tracing::debug!("Creating 'media' directory");
    let media = path.join(StorageType::Media.to_string());
    std::fs::create_dir_all(media).context("Couldn't create 'media' directory")?;

    tracing::debug!("Creating 'other' directory");
    let other = path.join(StorageType::Other.to_string());
    std::fs::create_dir_all(other).context("Couldn't create 'other' directory")?;

    Ok(())
}

pub fn get_dir(filename: &String) -> Option<StorageType> {
    let filepath = Path::new(&filename);
    let mime = mime_guess::from_path(filepath).first();

    match mime {
        Some(mime) => {
            if mime.type_() == mime_guess::mime::AUDIO {
                Some(StorageType::Media)
            } else {
                tracing::debug!("unknown mime: {mime}");
                None
            }
        }
        None => match filepath.extension() {
            Some(ext) => {
                if ext == "fseq" {
                    Some(StorageType::Sequences)
                } else {
                    tracing::debug!("unknown extension: {ext:?}");
                    None
                }
            }
            None => {
                tracing::debug!("missing file extension");
                None
            }
        },
    }
}

pub fn read_file(cfg: &Config, filename: &String, st: StorageType) -> Result<Option<Vec<u8>>> {
    let path = Path::new(&cfg.storage).join(st.to_string()).join(filename);

    if path.exists() {
        match File::open(path) {
            Ok(mut f) => match f.metadata() {
                Ok(meta) => {
                    let mut buf = vec![0; meta.len() as usize];
                    match f.read_exact(&mut buf) {
                        Ok(_) => Ok(Some(buf)),
                        Err(e) => bail!("Could not read file: {e}"),
                    }
                }
                Err(e) => bail!("Could not read file metadata: {e}"),
            },
            Err(e) => bail!("Could not open file: {e}"),
        }
    } else {
        Ok(None)
    }
}

pub fn upload_file(cfg: &Config, filename: &String, st: StorageType, data: Vec<u8>) -> Result<()> {
    let path = Path::new(&cfg.storage).join(st.to_string()).join(filename);

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    f.write_all(data.as_slice())?;

    Ok(())
}

pub fn read_sequence_meta(cfg: &Config, filename: &String) -> Result<Option<fseq::parser::FSeq>> {
    let path = Path::new(&cfg.storage)
        .join(StorageType::Sequences.to_string())
        .join(filename);

    if !path.exists() {
        return Ok(None);
    }

    let path = path.as_str()?;
    Ok(Some(*fseq::parser::parse(path)?))
}

pub fn del_file(cfg: &Config, filename: &String, st: StorageType) -> Result<()> {
    let path = Path::new(&cfg.storage).join(st.to_string()).join(filename);

    if !path.exists() {
        return Ok(());
    }

    std::fs::remove_file(path).map_err(|e| anyhow!(e))
}

pub fn read_outputs(cfg: &Config) -> Result<Channels> {
    let filename = Path::new(&cfg.storage)
        .join(StorageType::Other.to_string())
        .join("outputs.json");

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(filename)
        .context("Could not open file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Could not read file")?;

    serde_json::from_str::<Channels>(&contents).map_err(|e| anyhow!(e))
}

pub fn output_exists(cfg: &Config) -> bool {
    Path::new(&cfg.storage)
        .join(StorageType::Other.to_string())
        .join("outputs.json")
        .exists()
}
