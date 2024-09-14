use crate::server::Frame;

use bytes::Bytes;
use core::fmt;
use std::vec;

#[derive(Debug)]
pub(crate) struct Parser {
    /// Array frame iterator
    tokens: vec::IntoIter<Frame>,
}

#[derive(Debug)]
pub(crate) enum ParserError {
    /// No more frame to be consumed
    NoMoreFrame,

    /// Other errors
    Other(crate::Error),
}

impl Parser {
    pub(crate) fn new(frame: Frame) -> Result<Parser, ParserError> {
        let array = match frame {
            Frame::Array(array) => array,
            frame => return Err(format!("protocol error; expected array, got {:?}", frame).into()),
        };

        Ok(Parser {
            tokens: array.into_iter(),
        })
    }

    fn next(&mut self) -> Result<Frame, ParserError> {
        self.tokens.next().ok_or(ParserError::NoMoreFrame)
    }

    pub(crate) fn next_string(&mut self) -> Result<String, ParserError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(data) => std::str::from_utf8(&data[..])
                .map(|s| s.to_string())
                .map_err(|_| "protocol error; invalid string".into()),
            frame => Err(format!(
                "protocol error; expected simple frame or bulk frame, got {:?}",
                frame
            )
            .into()),
        }
    }

    pub(crate) fn next_bytes(&mut self) -> Result<Bytes, ParserError> {
        match self.next()? {
            Frame::Simple(s) => Ok(Bytes::from(s.into_bytes())),
            Frame::Bulk(data) => Ok(data),
            frame => Err(format!(
                "protocol error; expected simple frame or bulk frame, got {:?}",
                frame
            )
            .into()),
        }
    }

    pub(crate) fn next_int(&mut self) -> Result<u64, ParserError> {
        match self.next()? {
            Frame::Integer(int) => Ok(int),
            Frame::Simple(data) => atoi::atoi::<u64>(data.as_bytes())
                .ok_or_else(|| "protocol error; invalid number".into()),
            Frame::Bulk(data) => {
                atoi::atoi::<u64>(&data).ok_or_else(|| "protocol error; invalid number".into())
            }
            frame => Err(format!("protocol error; expected int frame but got {:?}", frame).into()),
        }
    }

    pub(crate) fn finish(&mut self) -> Result<(), ParserError> {
        if self.tokens.next().is_none() {
            Ok(())
        } else {
            Err("protocol error; expected end of frame, but there was more".into())
        }
    }
}

impl From<String> for ParserError {
    fn from(src: String) -> ParserError {
        ParserError::Other(src.into())
    }
}

impl From<&str> for ParserError {
    fn from(src: &str) -> ParserError {
        src.to_string().into()
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::NoMoreFrame => "protocol error; unexpected end of stream".fmt(f),
            ParserError::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ParserError {}

#[cfg(test)]
#[path = "test/parser_test.rs"]
mod parser_test;