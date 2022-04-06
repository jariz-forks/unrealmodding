use std::{io, fmt::Display, error};

use num_enum::{TryFromPrimitiveError, TryFromPrimitive};

#[derive(Debug)]
pub enum KismetError {
    InvalidToken(Box<str>),
    UnknownExpression(Box<str>)
}

impl KismetError {
    pub fn token(msg: String) -> Self {
        KismetError::InvalidToken(msg.into_boxed_str()) // todo: maybe not allocate a string
    }

    pub fn expression(msg: String) -> Self {
        KismetError::UnknownExpression(msg.into_boxed_str())
    }
}

impl Display for KismetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            KismetError::InvalidToken(ref err) => f.write_str(err),
            KismetError::UnknownExpression(ref err) => f.write_str(err)
        }
    }
}


#[derive(Debug)]
pub enum ErrorCode {
    Io(io::Error),
    InvalidFile(Box<str>),
    InvalidPackageIndex(Box<str>),
    InvalidEnumValue(Box<str>),
    Unimplemented(Box<str>),
    Kismet(KismetError)
}

#[derive(Debug)]
pub struct Error {
    code: ErrorCode
}

impl Error {
    pub fn invalid_file(msg: String) -> Self {
        Error { code: ErrorCode::InvalidFile(msg.into_boxed_str()) }
    }

    pub fn invalid_package_index(msg: String) -> Self {
        Error { code: ErrorCode::InvalidPackageIndex(msg.into_boxed_str()) }
    }

    pub fn unimplemented(msg: String) -> Self {
        Error { code: ErrorCode::Unimplemented(msg.into_boxed_str()) }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error { code: ErrorCode::Io(e) }
    }
}

impl<T: TryFromPrimitive> From<TryFromPrimitiveError<T>> for Error {
    fn from(e: TryFromPrimitiveError<T>) -> Self {
        Error { code: ErrorCode::InvalidEnumValue(e.to_string().into_boxed_str()) }
    }
}

impl From<KismetError> for Error {
    fn from(e: KismetError) -> Self {
        Error { code: ErrorCode::Kismet(e) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.code, f)
    }
}

impl error::Error for Error {
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorCode::Io(ref err) => Display::fmt(err, f),
            ErrorCode::InvalidFile(ref err) => f.write_str(err),
            ErrorCode::InvalidPackageIndex(ref err) => f.write_str(err),
            ErrorCode::InvalidEnumValue(ref err) => f.write_str(err),
            ErrorCode::Unimplemented(ref err) => f.write_str(err),
            ErrorCode::Kismet(ref err) => Display::fmt(err, f)
        }
    }
}