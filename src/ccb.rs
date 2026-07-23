use core::mem::MaybeUninit;

/// Circular Buffer
pub struct CircularBuffer<T, const N: usize>
{
    buffer: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const N: usize> CircularBuffer<T, N>
{
    pub const fn new() -> Self
    {
        assert!(N > 0);
        Self {buffer: unsafe { MaybeUninit::uninit().assume_init() }, head: 0, tail: 0, len: 0}
    }

    #[inline]
    pub const fn capacity(&self) -> usize
    {
        N
    }

    #[inline]
    pub const fn len(&self) -> usize
    {
        self.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool
    {
        self.len == 0
    }

    #[inline]
    pub const fn is_full(&self) -> bool
    {
        self.len == N
    }

    #[inline]
    pub const fn remaining(&self) -> usize
    {
        N - self.len
    }

    #[inline(always)]
    fn increment(index: usize) -> usize
    {
        if index + 1 == N
        {
            0
        }
        else
        {
            index + 1
        }
    }

    /* 
     * Insert new element
     * Return Err(value) if is full
     */
    pub fn push(&mut self, value: T) -> Result<(), T>
    {
        if self.is_full()
        {
            return Err(value);
        }

        self.buffer[self.tail].write(value);

        self.tail = Self::increment(self.tail);
        self.len += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T>
    {
        if self.is_empty()
        {
            return None;
        }

        let value = unsafe
        {
            self.buffer[self.head].assume_init_read()
        };

        self.head = Self::increment(self.head);
        self.len -= 1;

        Some(value)
    }

    pub fn peek(&self) -> Option<&T>
    {
        if self.is_empty()
        {
            return None;
        }

        unsafe
        {
            Some(self.buffer[self.head].assume_init_ref())
        }
    }

    /*
     * Get peek mutable
     */
    pub fn peek_mut(&mut self) -> Option<&mut T>
    {
        if self.is_empty() {
            return None;
        }

        unsafe {
            Some(self.buffer[self.head].assume_init_mut())
        }
    }
    /*
     * Remove all elements.
     */
    pub fn clear(&mut self)
    {
        while self.pop().is_some() {}
    }

}

impl<T, const N: usize> Default for CircularBuffer<T, N>
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl<T, const N: usize> Drop for CircularBuffer<T, N>
{
    fn drop(&mut self)
    {
        while let Some(_) = self.pop() {}
    }
}