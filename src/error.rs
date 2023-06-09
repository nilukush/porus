// error.rs

use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    ReqwestError(ReqwestError),
    JsonError(JsonError),
    StringError(String), // New variant to hold the error message as a String
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::ReqwestError(err) => write!(f, "ReqwestError: {}", err),
            CustomError::JsonError(err) => write!(f, "JsonError: {}", err),
            CustomError::StringError(err) => write!(f, "StringError: {}", err),
        }
    }
}

impl std::error::Error for CustomError {}

impl From<ReqwestError> for CustomError {
    fn from(err: ReqwestError) -> Self {
        CustomError::ReqwestError(err)
    }
}

impl From<JsonError> for CustomError {
    fn from(err: JsonError) -> Self {
        CustomError::JsonError(err)
    }
}

impl From<String> for CustomError {
    fn from(err: String) -> Self {
        CustomError::StringError(err)
    }
}

impl serde::Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            CustomError::ReqwestError(_) => {
                // Customize the serialization of the ReqwestError variant
                serializer.serialize_unit()
            }
            CustomError::JsonError(_) => {
                // Customize the serialization of the JsonError variant
                serializer.serialize_unit()
            }
            CustomError::StringError(_) => {
                // Customize the serialization of the StringError variant
                serializer.serialize_unit()
            }
        }
    }
}
