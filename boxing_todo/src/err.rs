
use std::fmt;
use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]

pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErr::Empty => write!(f, "Fail to parse todo"),
            ParseErr::Malformed(_) => write!(f, "Fail to parse todo"),
        }
    }
}

#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to read todo file")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(_) => Some(self),
        }
    }
}
impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.child_err.as_ref())
    }
}