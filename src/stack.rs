use thiserror::Error;

#[derive(Error, Debug)]
pub enum StackError {
    #[error("Not enough space in BidirectionalStack")]
    InsufficientCapacity,
}

/// A bidirectional stack that allows pushing and popping from both ends
/// within a fixed-size buffer.
pub struct BidirectionalStack<const CAPACITY: usize> {
    /// Buffer that stores data for both stacks
    buffer: [u8; CAPACITY],
    /// Front index (points to next free slot)
    front_index: usize,
    /// Back index (points to next free slot)
    back_index: usize,
}

impl<const CAPACITY: usize> BidirectionalStack<CAPACITY> {
    /// Creates a new empty bidirectional stack with fixed capacity.
    pub fn new() -> Self {
        BidirectionalStack {
            buffer: [0; CAPACITY],
            front_index: 0,
            back_index: CAPACITY - 1,
        }
    }

    /// Returns the amount of free space available in the stack.
    pub fn available_capacity(&self) -> usize {
        if self.back_index >= self.front_index {
            self.back_index - self.front_index + 1
        } else {
            0 // Stacks have overlapped - no free space
        }
    }

    /// Pushes data to the front of the stack.
    ///
    /// Returns Ok(()) if successful, or StackError if there's not enough space.
    pub fn push_front(&mut self, data: &[u8]) -> Result<(), StackError> {
        let data_length = data.len();
        // Need space for data + 1 byte to store length
        if self.available_capacity() < data_length + 1 {
            return Err(StackError::InsufficientCapacity);
        }

        // Store the data
        for byte in data {
            self.buffer[self.front_index] = *byte;
            self.front_index += 1;
        }

        // Store the length after the data
        self.buffer[self.front_index] = data_length as u8;
        self.front_index += 1;

        Ok(())
    }

    /// Pops data from the front of the stack.
    ///
    /// Returns the data if successful, or None if the stack is empty.
    pub fn pop_front(&mut self) -> Option<Vec<u8>> {
        if self.front_index == 0 {
            return None; // Stack is empty
        }

        // Read the length byte
        self.front_index -= 1;
        let data_length = self.buffer[self.front_index] as usize;

        if self.front_index < data_length {
            // This indicates data corruption - restore front pointer and return None
            self.front_index += 1;
            return None;
        }

        // Read the data bytes
        let mut result = Vec::with_capacity(data_length);
        for _ in 0..data_length {
            self.front_index -= 1;
            result.push(self.buffer[self.front_index]);
        }

        // Reverse to get correct order (we read data in reverse)
        result.reverse();
        Some(result)
    }

    /// Pushes data to the back of the stack.
    ///
    /// Returns Ok(()) if successful, or StackError if there's not enough space.
    pub fn push_back(&mut self, data: &[u8]) -> Result<(), StackError> {
        let data_length = data.len();
        // Need space for data + 1 byte to store length
        if self.available_capacity() < data_length + 1 {
            return Err(StackError::InsufficientCapacity);
        }

        // Store the data
        for byte in data {
            self.buffer[self.back_index] = *byte;
            self.back_index -= 1;
        }

        // Store the length after the data
        self.buffer[self.back_index] = data_length as u8;
        self.back_index -= 1;

        Ok(())
    }

    /// Pops data from the back of the stack.
    ///
    /// Returns the data if successful, or None if the stack is empty.
    pub fn pop_back(&mut self) -> Option<Vec<u8>> {
        if self.back_index == CAPACITY - 1 {
            return None; // Stack is empty
        }

        // Read the length byte
        self.back_index += 1;
        let data_length = self.buffer[self.back_index] as usize;

        if self.back_index + data_length >= CAPACITY {
            // This indicates data corruption - restore back pointer and return None
            self.back_index -= 1;
            return None;
        }

        // Read the data bytes
        let mut result = Vec::with_capacity(data_length);
        for _ in 0..data_length {
            self.back_index += 1;
            result.push(self.buffer[self.back_index]);
        }

        // Reverse to get correct order (we read data in reverse)
        result.reverse();
        Some(result)
    }
}
