use crate::http::request;

use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter, Debug};
use std::str;

pub struct Request {
    path: String,
    query_str: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    
    // Only supports HTTP 1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }
        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            // i + 1 because first value of slicing is inclusive
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write message with the Formatter
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write message with the Formatter
        write!(f, "{}", self.message())
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {
    
}

