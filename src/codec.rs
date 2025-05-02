use std::io::Cursor;
use crate::scheduler::Scheduler;

/// Helper functions for encoding and decoding data on the scheduler
pub mod stack {
    use super::*;

    /// Decode a value from the scheduler's data stack and truncate the stack
    pub fn decode<T>(scheduler: &mut Scheduler) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        let reversed_data = scheduler.get_reversed_data();
        let mut cursor = Cursor::new(&reversed_data);
        let value: T = ciborium::de::from_reader(&mut cursor)
            .expect("Failed to deserialize value from data stack");
        let pos = cursor.position() as usize;
        scheduler.truncate_stack(pos);
        value
    }

    /// Encode a value and push it to the scheduler's data stack
    pub fn encode<T: serde::Serialize>(scheduler: &mut Scheduler, value: T) {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&value, &mut buffer)
            .expect("Failed to serialize value to data stack");
        scheduler.extend_data(&buffer);
    }

    /// Push multiple encoded values to the stack
    pub fn push_encoded(scheduler: &mut Scheduler, encoded_values: Vec<Vec<u8>>) {
        scheduler.push_multiple_data(encoded_values);
    }
}
