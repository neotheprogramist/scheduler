use crate::scheduler::Scheduler;
use serde::{Serialize, de::DeserializeOwned};
use std::io::Cursor;
use thiserror::Error;

/// Errors that can occur during encoding/decoding operations
#[derive(Debug, Error)]
pub enum CodecError {
    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("deserialization error: {0}")]
    DeserializationError(String),

    #[error("empty stack")]
    EmptyStack,
}

/// Helper functions for encoding and decoding data on the scheduler's stack
pub mod stack {
    use super::*;

    /// Decode a value from the scheduler's data stack and truncate the stack
    ///
    /// This function will:
    /// 1. Read the data stack in reverse (to get the correct order)
    /// 2. Deserialize the value at the top of the stack
    /// 3. Truncate the stack to remove the consumed data
    pub fn decode<T>(scheduler: &mut Scheduler) -> T
    where
        T: DeserializeOwned,
    {
        let reversed_data = scheduler.get_reversed_data();
        let mut cursor = Cursor::new(&reversed_data);
        let value: T = ciborium::de::from_reader(&mut cursor)
            .expect("Failed to deserialize value from data stack");
        let pos = cursor.position() as usize;
        scheduler.truncate_stack(pos);
        value
    }

    /// Try to decode a value, returning a Result
    ///
    /// This is a safer version of decode that returns an error instead of panicking
    pub fn try_decode<T>(scheduler: &mut Scheduler) -> Result<T, CodecError>
    where
        T: DeserializeOwned,
    {
        let reversed_data = scheduler.get_reversed_data();
        if reversed_data.is_empty() {
            return Err(CodecError::EmptyStack);
        }

        let mut cursor = Cursor::new(&reversed_data);
        let value: T = ciborium::de::from_reader(&mut cursor)
            .map_err(|e| CodecError::DeserializationError(e.to_string()))?;
        let pos = cursor.position() as usize;
        scheduler.truncate_stack(pos);
        Ok(value)
    }

    /// Encode a value and push it to the scheduler's data stack
    ///
    /// This function will serialize the value and add it to the data stack
    pub fn encode<T: Serialize>(scheduler: &mut Scheduler, value: T) {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&value, &mut buffer)
            .expect("Failed to serialize value to data stack");
        scheduler.extend_data(&buffer);
    }

    /// Try to encode a value, returning a Result
    ///
    /// This is a safer version of encode that returns an error instead of panicking
    pub fn try_encode<T: Serialize>(scheduler: &mut Scheduler, value: T) -> Result<(), CodecError> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&value, &mut buffer)
            .map_err(|e| CodecError::SerializationError(e.to_string()))?;
        scheduler.extend_data(&buffer);
        Ok(())
    }

    /// Push multiple encoded values to the stack
    ///
    /// This is useful when pushing multiple values at once
    pub fn push_encoded(scheduler: &mut Scheduler, encoded_values: Vec<Vec<u8>>) {
        scheduler.push_multiple_data(encoded_values);
    }

    /// Peek at the top value on the stack without removing it
    ///
    /// This is useful for inspecting the stack without modifying it
    pub fn peek<T>(scheduler: &Scheduler) -> Result<T, CodecError>
    where
        T: DeserializeOwned,
    {
        let reversed_data = scheduler.get_reversed_data();
        if reversed_data.is_empty() {
            return Err(CodecError::EmptyStack);
        }

        let mut cursor = Cursor::new(&reversed_data);
        let value: T = ciborium::de::from_reader(&mut cursor)
            .map_err(|e| CodecError::DeserializationError(e.to_string()))?;
        Ok(value)
    }
}

/// Convert a value to bytes using CBOR serialization
pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, CodecError> {
    let mut buffer = Vec::new();
    ciborium::ser::into_writer(value, &mut buffer)
        .map_err(|e| CodecError::SerializationError(e.to_string()))?;
    Ok(buffer)
}

/// Convert bytes to a value using CBOR deserialization
pub fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, CodecError> {
    let mut cursor = Cursor::new(bytes);
    let value: T = ciborium::de::from_reader(&mut cursor)
        .map_err(|e| CodecError::DeserializationError(e.to_string()))?;
    Ok(value)
}
