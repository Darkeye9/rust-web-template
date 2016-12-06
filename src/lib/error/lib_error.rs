use std::{io, fmt, error};
use rustc_serialize::json::{DecoderError, EncoderError};

#[derive(Debug)]
#[allow(dead_code)]
pub enum LibError {
    Io(io::Error),
    Parse(DecoderError),
    Encode(EncoderError),
    PoisonedLock,
    LongLockedLock
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibError::Io(ref err) => err.fmt(f),
            LibError::Parse(ref err) => err.fmt(f),
            LibError::Encode(ref err) => err.fmt(f),
            LibError::PoisonedLock => write!(f, "The lock is poisoned"),
            LibError::LongLockedLock => write!(f, "The lock locked for too much time")
        }
    }
}

impl error::Error for LibError {
    fn description(&self) -> &str {
        match *self {
            LibError::Io(ref err) => err.description(),
            LibError::Parse(ref err) => err.description(),
            LibError::Encode(ref err) => err.description(),
            LibError::PoisonedLock => "lock is poisoned",
            LibError::LongLockedLock => "lock locked very long",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LibError::Io(ref err) => Some(err),
            LibError::Parse(ref err) => Some(err),
            LibError::Encode(ref err) => Some(err),
            LibError::PoisonedLock => None,
            LibError::LongLockedLock => None
        }
    }
}

//For composed Errors, we need to define a translation
impl From<io::Error> for LibError {
    fn from(err: io::Error) -> LibError {
        LibError::Io(err)
    }
}

impl From<DecoderError> for LibError {
    fn from(err: DecoderError) -> LibError {
        LibError::Parse(err)
    }
}

impl From<EncoderError> for LibError {
    fn from(err: EncoderError) -> LibError {
        LibError::Encode(err)
    }
}
