use std::fmt::{self, Display};
use std::io;

#[derive(Debug)]
pub enum ExecutionError {
    IoError(io::Error),
    ParseError(ParseError),
    CpuError(CpuError),
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "{}", err),
            Self::ParseError(err) => write!(f, "{}", err),
            Self::CpuError(err) => write!(f, "{}", err),
        }
    }
}

impl From<io::Error> for ExecutionError {
    fn from(err: io::Error) -> Self {
        ExecutionError::IoError(err)
    }
}

impl From<ParseError> for ExecutionError {
    fn from(err: ParseError) -> Self {
        ExecutionError::ParseError(err)
    }
}

impl From<CpuError> for ExecutionError {
    fn from(err: CpuError) -> Self {
        ExecutionError::CpuError(err)
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    BitCountParseError,
    CommandParseError { cmd: String, line: usize },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BitCountParseError => write!(f, "invalid bit count specification"),
            Self::CommandParseError { cmd, line } => {
                write!(f, "invalid command '{}' at line {}", cmd, line)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CpuError {
    NegativeAddr,
    OutOfMemory,
}

impl Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuError::NegativeAddr => write!(f, "attempted to move below address 0"),
            CpuError::OutOfMemory => write!(f, "attempted to move beyond memory end"),
        }
    }
}
