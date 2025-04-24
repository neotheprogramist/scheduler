use bincode;

use crate::scheduler::Scheduler;

/// Helper functions for encoding and decoding data on the scheduler
pub mod stack {
    use super::*;
    
    /// Decode a value from the scheduler's data stack and truncate the stack
    pub fn decode<T>(scheduler: &mut Scheduler) -> T 
    where 
        T: bincode::Decode<()>,
    {
        let reversed_data = scheduler.get_reversed_data();
        let (value, len): (T, usize) = bincode::decode_from_slice(
            &reversed_data, 
            bincode::config::standard()
        ).unwrap();
        scheduler.truncate_stack(len);
        value
    }
    
    /// Encode a value and push it to the scheduler's data stack
    pub fn encode<T: bincode::Encode>(scheduler: &mut Scheduler, value: T) {
        let encoded = bincode::encode_to_vec(value, bincode::config::standard()).unwrap();
        scheduler.extend_data(&encoded);
    }
    
    /// Push multiple encoded values to the stack
    pub fn push_encoded(scheduler: &mut Scheduler, encoded_values: Vec<Vec<u8>>) {
        scheduler.push_multiple_data(encoded_values);
    }
} 