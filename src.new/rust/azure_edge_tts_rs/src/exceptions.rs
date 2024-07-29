/*Exceptions for the Edge TTS project.*/

use std::error::Error;

#[derive(Debug)]
struct UnknownResponse{
    message: String,
}

impl std::fmt::Display for UnknownResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: UnkownResponse: {}", self.message)
    }
}

impl Error for UnknownResponse{}

