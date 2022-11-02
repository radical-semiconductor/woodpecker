use std::io;
use std::fmt::{self, Display};

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
pub struct ParseError {
    pub cmd: String,
    pub line: usize,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid command '{}' at line {}", self.cmd, self.line)
    }
}

#[derive(Debug, Clone)]
pub enum CpuErrorKind {
    NegativeAddr,
    OutOfMemory,
}

#[derive(Debug, Clone)]
pub struct CpuError {
    pub kind: CpuErrorKind,
    pub step: usize,
}

impl Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NegativeAddr => write!(f, "attempted to move below address 0 at step {}", self.step),
            OutOfMemory => write!(f, "attempted to move to location higher than ADDR can store at step {}", self.step),
        }
    }
}