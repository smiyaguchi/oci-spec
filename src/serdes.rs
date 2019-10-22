use serde;
use serde_json;

use std::fmt;
use std::io;
use std::error::Error;
use std::fs::File;

#[derive(Debug)]
pub enum OciSpecError {
    Io(io::Error),
    Json(serde_json::Error),    
}

impl fmt::Display for OciSpecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OciSpecError::Io(ref err) => err.fmt(f),
            OciSpecError::Json(ref err) => err.fmt(f),    
        }    
    }    
}

impl Error for OciSpecError {
    fn description(&self) -> &str {
        match *self {
            OciSpecError::Io(ref err) => err.description(),
            OciSpecError::Json(ref err) => err.description(),    
        }    
    }
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            OciSpecError::Io(ref err) => Some(err),
            OciSpecError::Json(ref err) => Some(err),    
        }    
    } 
}

impl From<io::Error> for OciSpecError {
    fn from(err: io::Error) -> OciSpecError {
        OciSpecError::Io(err)    
    } 
}

impl From<serde_json::Error> for OciSpecError {
    fn from(err: serde_json::Error) -> OciSpecError {
        OciSpecError::Json(err) 
    }    
}

pub fn serialize<T: serde::Serialize>(obj: &T, path: &str) -> Result<(), OciSpecError> {
    let mut file = File::create(path)?;
    Ok(serde_json::to_writer(&mut file, &obj)?)
}

pub fn deserialize<T>(path: &str) -> Result<T, OciSpecError> 
    where for<'de> T: serde::Deserialize<'de>
{
    let file = File::open(path)?;
    Ok(serde_json::from_reader(&file)?)    
}

pub fn to_writer<W: io::Write, T: serde::Serialize>(obj: &T, mut writer: W) -> Result<(), OciSpecError> {
    Ok(serde_json::to_writer(&mut writer, &obj)?)    
}

pub fn to_string<T: serde::Serialize>(obj: &T) -> Result<String, OciSpecError> {
    Ok(serde_json::to_string(&obj)?)    
}
