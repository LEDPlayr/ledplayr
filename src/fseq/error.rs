use thiserror::Error;

#[derive(Debug, Error)]
pub enum FSeqError {
    #[error("Bad magic number")]
    BadMagic,
    #[error("Bad variable block")]
    BadVariableBlock,
    #[error("Unhandled compression scheme")]
    UnhandledCompression,
    #[error("Unknown compression scheme: {0}")]
    UnknownCompression(u8),
    #[error("Frame not found")]
    FrameNotFound,
}
