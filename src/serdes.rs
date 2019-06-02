use serde;
use serde_json;

use std::fmt;
use std::io;
use std::error::Error;
use std::fs::File;

#[derive(Debug)]
pub enum SerDesError {
    Io(io::Error),
    Json(serde_json::Error),    
}

impl fmt::Display for SerDesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SerDesError::Io(ref err) => err.fmt(f),
            SerDesError::Json(ref err) => err.fmt(f),    
        }    
    }    
}

impl Error for SerDesError {
    fn description(&self) -> &str {
        match *self {
            SerDesError::Io(ref err) => err.description(),
            SerDesError::Json(ref err) => err.description(),    
        }    
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            SerDesError::Io(ref err) => Some(err),
            SerDesError::Json(ref err) => Some(err),    
        }    
    } 
}

impl From<io::Error> for SerDesError {
    fn from(err: io::Error) -> SerDesError {
        SerDesError::Io(err)    
    } 
}

impl From<serde_json::Error> for SerDesError {
    fn from(err: serde_json::Error) -> SerDesError {
        SerDesError::Json(err)    
    }    
}

pub fn serialize<T: serde::Serialize>(obj: &T, path: &str) -> Result<(), SerDesError> {
    let mut file = File::create(path)?;
    Ok(serde_json::to_writer(&mut file, &obj)?)
}

pub fn deserialize<T>(path: &str) -> Result<T, SerDesError> 
    where for<'de> T: serde::Deserialize<'de>
{
    let file = File::open(path)?;
    Ok(serde_json::from_reader(&file)?)    
}

pub fn to_writer<W: io::Write, T: serde::Serialize>(obj: &T, mut writer: W) -> Result<(), SerDesError> {
    Ok(serde_json::to_writer(&mut writer, &obj)?)    
}

pub fn to_string<T: serde::Serialize>(obj: &T) -> Result<String, SerDesError> {
    Ok(serde_json::to_string(&obj)?)    
}
