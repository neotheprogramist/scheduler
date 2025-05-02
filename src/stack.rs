//! Bidirectional stack implementation for the scheduler.
//!
//! This module provides a fixed-size bidirectional stack that allows pushing and
//! popping from both ends. It's used by the scheduler to maintain the call stack
//! and data stack.

use thiserror::Error;

/// Errors that can occur during stack operations.
#[derive(Error, Debug)]
pub enum StackError {
    /// The stack doesn't have enough capacity for the requested operation
    #[error("Not enough space in BidirectionalStack")]
    InsufficientCapacity,
}

/// A bidirectional stack that allows pushing and popping from both ends
/// within a fixed-size buffer.
///
/// This stack maintains two logical stacks within a single buffer:
/// - Front stack: Data is pushed and popped from the beginning of the buffer (lower indices)
/// - Back stack: Data is pushed and popped from the end of the buffer (higher indices)
///
/// Both stacks grow towards the middle of the buffer. When they meet, the buffer is full.
#[derive(Clone, Debug)]
pub struct BidirectionalStack<const CAPACITY: usize> {
    /// Buffer that stores data for both stacks
    buffer: [u8; CAPACITY],
    /// Front index (points to next free slot)
    front_index: usize,
    /// Back index (points to next free slot)
    back_index: usize,
}

impl<const CAPACITY: usize> BidirectionalStack<CAPACITY> {
    /// Creates a new, empty bidirectional stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::stack::BidirectionalStack;
    ///
    /// let stack = BidirectionalStack::<1024>::new();
    /// assert!(stack.is_empty_front());
    /// assert!(stack.is_empty_back());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the amount of free space available in the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::stack::BidirectionalStack;
    ///
    /// let stack = BidirectionalStack::<10>::new();
    /// assert_eq!(stack.available_capacity(), 10);
    /// ```
    pub fn available_capacity(&self) -> usize {
        if self.back_index >= self.front_index {
            self.back_index - self.front_index + 1
        } else {
            0 // Stacks have overlapped - no free space
        }
    }

    /// Checks if the stack has enough capacity for the given data size.
    ///
    /// Takes into account the extra byte needed to store the length.
    fn has_capacity_for(&self, data_size: usize) -> bool {
        // Need space for data + 1 byte to store length
        self.available_capacity() > data_size
    }

    /// Pushes data to the front of the stack.
    ///
    /// Returns Ok(()) if successful, or StackError if there's not enough space.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to push to the front of the stack
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::stack::BidirectionalStack;
    ///
    /// let mut stack = BidirectionalStack::<10>::new();
    /// stack.push_front(&[1, 2, 3]).unwrap();
    /// ```
    pub fn push_front(&mut self, data: &[u8]) -> Result<(), StackError> {
        let data_length = data.len();

        // Check if we have enough space
        if !self.has_capacity_for(data_length) {
            return Err(StackError::InsufficientCapacity);
        }

        // Store the data
        for byte in data {
            self.buffer[self.front_index] = *byte;
            self.front_index += 1;
        }

        // Store the length after the data
        self.buffer[self.front_index] = data_length
            .try_into()
            .map_err(|_| StackError::InsufficientCapacity)?;
        self.front_index += 1;

        Ok(())
    }

    /// Pops data from the front of the stack.
    ///
    /// Returns the data if successful, or None if the stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::stack::BidirectionalStack;
    ///
    /// let mut stack = BidirectionalStack::<10>::new();
    /// stack.push_front(&[1, 2, 3]).unwrap();
    /// assert_eq!(stack.pop_front().unwrap(), vec![1, 2, 3]);
    /// ```
    pub fn pop_front(&mut self) -> Option<Vec<u8>> {
        if self.is_empty_front() {
            return None; // Stack is empty
        }

        // Read the length byte
        self.front_index = self.front_index.saturating_sub(1);
        let data_length = self.buffer[self.front_index] as usize;

        if self.front_index < data_length {
            // This indicates data corruption - restore front pointer and return None
            self.front_index += 1;
            return None;
        }

        // Read the data bytes
        let mut result = Vec::with_capacity(data_length);
        for _ in 0..data_length {
            self.front_index = self.front_index.saturating_sub(1);
            result.push(self.buffer[self.front_index]);
        }

        // Reverse to get correct order (we read data in reverse)
        result.reverse();
        Some(result)
    }

    /// Pushes data to the back of the stack.
    ///
    /// Returns Ok(()) if successful, or StackError if there's not enough space.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to push to the back of the stack
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::stack::BidirectionalStack;
    ///
    /// let mut stack = BidirectionalStack::<10>::new();
    /// stack.push_back(&[1, 2, 3]).unwrap();
    /// ```
    pub fn push_back(&mut self, data: &[u8]) -> Result<(), StackError> {
        let data_length = data.len();

        // Check if we have enough space
        if !self.has_capacity_for(data_length) {
            return Err(StackError::InsufficientCapacity);
        }

        // Store the data
        for byte in data {
            self.buffer[self.back_index] = *byte;
            self.back_index = self.back_index.saturating_sub(1);
        }

        // Store the length after the data
        self.buffer[self.back_index] = data_length
            .try_into()
            .map_err(|_| StackError::InsufficientCapacity)?;
        self.back_index = self.back_index.saturating_sub(1);

        Ok(())
    }

    /// Pops data from the back of the stack.
    ///
    /// Returns the data if successful, or None if the stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::stack::BidirectionalStack;
    ///
    /// let mut stack = BidirectionalStack::<10>::new();
    /// stack.push_back(&[1, 2, 3]).unwrap();
    /// assert_eq!(stack.pop_back().unwrap(), vec![1, 2, 3]);
    /// ```
    pub fn pop_back(&mut self) -> Option<Vec<u8>> {
        if self.is_empty_back() {
            return None; // Stack is empty
        }

        // Read the length byte
        self.back_index = self.back_index.saturating_add(1);
        let data_length = self.buffer[self.back_index] as usize;

        if self.back_index + data_length >= CAPACITY {
            // This indicates data corruption - restore back pointer and return None
            self.back_index = self.back_index.saturating_sub(1);
            return None;
        }

        // Read the data bytes
        let mut result = Vec::with_capacity(data_length);
        for _ in 0..data_length {
            self.back_index = self.back_index.saturating_add(1);
            result.push(self.buffer[self.back_index]);
        }

        // Reverse to get correct order (we read data in reverse)
        result.reverse();
        Some(result)
    }

    /// Returns true if the front stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::stack::BidirectionalStack;
    ///
    /// let mut stack = BidirectionalStack::<10>::new();
    /// assert!(stack.is_empty_front());
    /// stack.push_front(&[1, 2, 3]).unwrap();
    /// assert!(!stack.is_empty_front());
    /// ```
    pub fn is_empty_front(&self) -> bool {
        self.front_index == 0
    }

    /// Returns true if the back stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::stack::BidirectionalStack;
    ///
    /// let mut stack = BidirectionalStack::<10>::new();
    /// assert!(stack.is_empty_back());
    /// stack.push_back(&[1, 2, 3]).unwrap();
    /// assert!(!stack.is_empty_back());
    /// ```
    pub fn is_empty_back(&self) -> bool {
        self.back_index == CAPACITY - 1
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

        // Push some data
        stack.push_front(&[1, 2, 3]).unwrap();
        assert!(!stack.is_empty_front());

        // Pop the data
        let data = stack.pop_front().unwrap();
        assert_eq!(data, vec![1, 2, 3]);
        assert!(stack.is_empty_front());
    }

    #[test]
    fn test_push_pop_back() {
        let mut stack = BidirectionalStack::<10>::new();
        assert!(stack.is_empty_back());

        // Push some data
        stack.push_back(&[1, 2, 3]).unwrap();
        assert!(!stack.is_empty_back());

        // Pop the data
        let data = stack.pop_back().unwrap();
        assert_eq!(data, vec![1, 2, 3]);
        assert!(stack.is_empty_back());
    }

    #[test]
    fn test_capacity() {
        let mut stack = BidirectionalStack::<5>::new();

        // Push 3 bytes + 1 byte for length = 4 bytes
        stack.push_front(&[1, 2, 3]).unwrap();

        // Only 1 byte left - not enough for another push
        assert_eq!(stack.available_capacity(), 1);
        assert!(stack.push_front(&[4]).is_err());
    }

    #[test]
    fn test_bidirectional() {
        let mut stack = BidirectionalStack::<10>::new();

        // Push from both ends
        stack.push_front(&[1, 2]).unwrap();
        stack.push_back(&[3, 4]).unwrap();

        // Pop from both ends
        let front_data = stack.pop_front().unwrap();
        let back_data = stack.pop_back().unwrap();

        assert_eq!(front_data, vec![1, 2]);
        assert_eq!(back_data, vec![3, 4]);
    }
}
