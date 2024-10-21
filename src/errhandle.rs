use std::error::Error;
use std::fmt::{Display, Formatter, Debug};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ParseErrorKind {
    InvalidParameter,
    NotSupportUsage,
    RandError,
    UnpaddingNotMatch,
    InvalidPublicKey,
    InvalidPrivateKey,
    VerificationFailed,
    OuterErr,
    InnerErr,
}

impl Debug for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::InvalidParameter => write!(f, "{}", "InvalidParameter"),
            ParseErrorKind::NotSupportUsage => write!(f, "{}", "NotSupportUsage"),
            ParseErrorKind::RandError => write!(f, "{}", "RandError"),
            ParseErrorKind::UnpaddingNotMatch => write!(f, "{}", "UnpaddingNotMatch"),
            ParseErrorKind::InvalidPublicKey => write!(f, "{}", "InvalidPublicKey"),
            ParseErrorKind::InvalidPrivateKey => write!(f, "{}", "InvalidPrivateKey"),
            ParseErrorKind::VerificationFailed => write!(f, "{}", "VerificationFailed"),
            ParseErrorKind::OuterErr => write!(f, "{}", "OuterErr: ErrorsCausedByExternalModule"),
            ParseErrorKind::InnerErr => write!(f, "{}", "InnerError"),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    err: Box<dyn std::error::Error + Sync + Send>,
}

impl ParseError {
    pub fn new<E>(kind: ParseErrorKind, err: E) -> ParseError 
        where E: Into<Box<dyn Error + Sync + Send>>{
        ParseError {
            kind,
            err: err.into(),
        }
    }
    
    pub fn kind(&self) -> ParseErrorKind {
        self.kind
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.kind, self.err)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.err.source()
    }
}
