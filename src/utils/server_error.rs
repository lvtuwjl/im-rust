// use crate::constant::errors::StatusCode;
use std::{error::Error as StdError, fmt::Display};
#[derive(Debug, Clone)]
pub struct ServerError {
    status: i16,
    // kind: String,
    message: String,
}

impl ServerError {
    pub fn new(status: i16, message: String) -> Self {
        Self {
            status,
            // kind,
            message,
        }
    }

    // pub fn status(&self) -> i16 {
    //     self.status
    // }

    // pub fn kind(&self) -> String {
    //     self.kind.clone()
    // }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl StdError for ServerError {}
