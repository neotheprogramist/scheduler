use thiserror::Error;

#[derive(Error, Debug)]
pub enum StackError {
    #[error("Not enough space in BidirectionalStack")]
    InsufficientCapacity,

    #[error("Data size exceeds maximum allowed length")]
    DataTooLarge,

    #[error("Stack underflow - attempted to read from empty stack")]
    Underflow,
}

/// A bidirectional stack that allows pushing and popping from both ends.
///
/// `CAPACITY` is the total number of bytes available for storage.
/// The stack grows from both ends toward the middle.
#[derive(Clone, Debug)]
pub struct BidirectionalStack<const CAPACITY: usize> {
    buffer: [u8; CAPACITY],
    front_index: usize, // Points to next free position from the front
    back_index: usize,  // Points to next free position from the back
}

impl<const CAPACITY: usize> BidirectionalStack<CAPACITY> {
    /// Creates a new empty stack with the specified capacity.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of bytes available in the stack.
    pub fn available_capacity(&self) -> usize {
        if self.back_index >= self.front_index {
            self.back_index - self.front_index
        } else {
            0
        }
    }

    /// Checks if there's enough capacity for the given data size plus one byte for length.
    fn has_capacity_for(&self, data_size: usize) -> bool {
        // Need space for data + 1 byte to store length
        self.available_capacity() > data_size
    }

    /// Pushes data to the front of the stack.
    ///
    /// Returns an error if there's not enough capacity.
    pub fn push_front(&mut self, data: &[u8]) -> Result<(), StackError> {
        let data_length = data.len();

        // Check if data_length can fit in a u8
        if data_length > u8::MAX as usize {
            return Err(StackError::DataTooLarge);
        }

        if !self.has_capacity_for(data_length) {
            return Err(StackError::InsufficientCapacity);
        }

        // Store length first
        self.buffer[self.front_index] = data_length as u8;
        self.front_index += 1;

        // Then store data
        for byte in data {
            self.buffer[self.front_index] = *byte;
            self.front_index += 1;
        }

        Ok(())
    }

    /// Pops data from the front of the stack.
    ///
    /// Returns None if the stack is empty from the front.
    pub fn pop_front(&mut self) -> Result<Vec<u8>, StackError> {
        if self.is_empty_front() {
            return Err(StackError::Underflow);
        }

        // Move back one position, adjusting for safety
        self.front_index = self.front_index.saturating_sub(1);

        // Move back data_length positions to find the start of the data
        let data_length = self.buffer[self.front_index].into();
        self.front_index = self.front_index.saturating_sub(data_length);

        // Extract the data
        let mut result = Vec::with_capacity(data_length);
        for i in 0..data_length {
            result.push(self.buffer[self.front_index + i]);
        }

        Ok(result)
    }

    /// Pushes data to the back of the stack.
    ///
    /// Returns an error if there's not enough capacity.
    pub fn push_back(&mut self, data: &[u8]) -> Result<(), StackError> {
        let data_length = data.len();

        // Check if data_length can fit in a u8
        if data_length > u8::MAX as usize {
            return Err(StackError::DataTooLarge);
        }

        if !self.has_capacity_for(data_length) {
            return Err(StackError::InsufficientCapacity);
        }

        // Store length first
        self.back_index = self.back_index.saturating_sub(1);
        self.buffer[self.back_index] = data_length as u8;

        // Then store data in reverse order
        for byte in data.iter().rev() {
            self.back_index = self.back_index.saturating_sub(1);
            self.buffer[self.back_index] = *byte;
        }

        Ok(())
    }

    /// Pops data from the back of the stack.
    ///
    /// Returns None if the stack is empty from the back.
    pub fn pop_back(&mut self) -> Result<Vec<u8>, StackError> {
        if self.is_empty_back() {
            return Err(StackError::Underflow);
        }

        // Read data length
        self.back_index = self.back_index.saturating_add(1);
        let data_length = self.buffer[self.back_index].into();

        // Extract the data
        let mut result = Vec::with_capacity(data_length);
        for _ in 0..data_length {
            self.back_index = self.back_index.saturating_add(1);
            result.push(self.buffer[self.back_index]);
        }

        Ok(result)
    }

    /// Returns true if the stack is empty from the front.
    pub fn is_empty_front(&self) -> bool {
        self.front_index == 0
    }

    /// Returns true if the stack is empty from the back.
    pub fn is_empty_back(&self) -> bool {
        self.back_index == CAPACITY - 1
    }

    /// Returns true if both the front and back of the stack are empty.
    pub fn is_empty(&self) -> bool {
        self.is_empty_front() && self.is_empty_back()
    }

    /// Clears the stack, removing all elements.
    pub fn clear(&mut self) {
        self.front_index = 0;
        self.back_index = CAPACITY - 1;
    }
}

impl<const CAPACITY: usize> Default for BidirectionalStack<CAPACITY> {
    fn default() -> Self {
        BidirectionalStack {
            buffer: [0; CAPACITY],
            front_index: 0,
            back_index: CAPACITY - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_front() {
        let mut stack = BidirectionalStack::<10>::new();
        assert!(stack.is_empty_front());

        stack.push_front(&[1, 2, 3]).unwrap();
        assert!(!stack.is_empty_front());

        let data = stack.pop_front().unwrap();
        assert_eq!(data, vec![1, 2, 3]);
        assert!(stack.is_empty_front());
    }

    #[test]
    fn test_push_pop_back() {
        let mut stack = BidirectionalStack::<10>::new();
        assert!(stack.is_empty_back());

        stack.push_back(&[1, 2, 3]).unwrap();
        assert!(!stack.is_empty_back());

        let data = stack.pop_back().unwrap();
        assert_eq!(data, vec![1, 2, 3]);
        assert!(stack.is_empty_back());
    }

    #[test]
    fn test_capacity() {
        let mut stack = BidirectionalStack::<5>::new();

        stack.push_front(&[1, 2]).unwrap();

        assert_eq!(stack.available_capacity(), 2);
        assert!(stack.push_front(&[3, 4]).is_err());
        assert!(stack.push_front(&[3]).is_ok());
    }

    #[test]
    fn test_bidirectional() {
        let mut stack = BidirectionalStack::<10>::new();

        stack.push_front(&[1, 2]).unwrap();
        stack.push_back(&[3, 4]).unwrap();

        let front_data = stack.pop_front().unwrap();
        let back_data = stack.pop_back().unwrap();

        assert_eq!(front_data, vec![1, 2]);
        assert_eq!(back_data, vec![3, 4]);
    }

    #[test]
    fn test_clear() {
        let mut stack = BidirectionalStack::<10>::new();

        stack.push_front(&[1, 2]).unwrap();
        stack.push_back(&[3, 4]).unwrap();

        assert!(!stack.is_empty());

        stack.clear();

        assert!(stack.is_empty());
        assert!(stack.is_empty_front());
        assert!(stack.is_empty_back());
    }
}
