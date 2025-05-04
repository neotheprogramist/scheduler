use std::num::TryFromIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StackError {
    #[error("Not enough space in BidirectionalStack")]
    InsufficientCapacity,

    #[error("Data size exceeds maximum allowed length")]
    DataTooLarge,

    #[error("Stack underflow - attempted to read from empty stack")]
    Underflow,

    #[error("Conversion")]
    Conversion(#[from] TryFromIntError),
}

#[derive(Clone, Debug)]
pub struct BidirectionalStack<const CAPACITY: usize> {
    front_index: usize,
    back_index: usize,
    buffer: [u8; CAPACITY],
}

impl<const CAPACITY: usize> BidirectionalStack<CAPACITY> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn available_capacity(&self) -> usize {
        if self.back_index >= self.front_index {
            self.back_index - self.front_index
        } else {
            0
        }
    }

    fn has_capacity_for(&self, data_size: usize) -> bool {
        // Need space for data + 1 byte to store length
        self.available_capacity() > data_size
    }

    pub fn push_front(&mut self, data: &[u8]) -> Result<(), StackError> {
        let data_length = data.len();

        if data_length > u8::MAX as usize {
            return Err(StackError::DataTooLarge);
        }

        if !self.has_capacity_for(data_length) {
            return Err(StackError::InsufficientCapacity);
        }

        for byte in data {
            self.buffer[self.front_index] = *byte;
            self.front_index = self.front_index.saturating_add(1);
        }

        self.buffer[self.front_index] = data_length.try_into()?;
        self.front_index = self.front_index.saturating_add(1);

        Ok(())
    }

    pub fn pop_front(&mut self) -> Result<Vec<u8>, StackError> {
        if self.is_empty_front() {
            return Err(StackError::Underflow);
        }

        self.front_index = self.front_index.saturating_sub(1);
        let data_length = self.buffer[self.front_index].into();

        let mut result = Vec::with_capacity(data_length);
        for _ in 0..data_length {
            self.front_index = self.front_index.saturating_sub(1);
            result.push(self.buffer[self.front_index]);
        }
        result.reverse();

        Ok(result)
    }

    pub fn push_back(&mut self, data: &[u8]) -> Result<(), StackError> {
        let data_length = data.len();

        if data_length > u8::MAX.into() {
            return Err(StackError::DataTooLarge);
        }

        if !self.has_capacity_for(data_length) {
            return Err(StackError::InsufficientCapacity);
        }

        for byte in data {
            self.back_index = self.back_index.saturating_sub(1);
            self.buffer[self.back_index] = *byte;
        }

        self.back_index = self.back_index.saturating_sub(1);
        self.buffer[self.back_index] = data_length.try_into()?;

        Ok(())
    }

    pub fn pop_back(&mut self) -> Result<Vec<u8>, StackError> {
        if self.is_empty_back() {
            return Err(StackError::Underflow);
        }

        let data_length = self.buffer[self.back_index].into();
        self.back_index = self.back_index.saturating_add(1);

        let mut result = Vec::with_capacity(data_length);
        for _ in 0..data_length {
            result.push(self.buffer[self.back_index]);
            self.back_index = self.back_index.saturating_add(1);
        }
        result.reverse();

        Ok(result)
    }

    pub fn is_empty_front(&self) -> bool {
        self.front_index == 0
    }

    pub fn is_empty_back(&self) -> bool {
        self.back_index == CAPACITY
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty_front() && self.is_empty_back()
    }

    pub fn clear(&mut self) {
        self.front_index = 0;
        self.back_index = CAPACITY;
    }
}

impl<const CAPACITY: usize> Default for BidirectionalStack<CAPACITY> {
    fn default() -> Self {
        BidirectionalStack {
            buffer: [0; CAPACITY],
            front_index: 0,
            back_index: CAPACITY,
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
        println!("{:?}", stack.buffer);

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
