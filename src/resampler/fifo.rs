use std::default::Default;

pub struct Fifo<T> {
    buffer: [T; BUFFER_SIZE],
    windex: Index,
    rindex: Index,
}

impl<T: Default + Copy> Fifo<T> {
    pub fn new() -> Fifo<T> {
        Fifo {
            buffer: [ Default::default(); BUFFER_SIZE ],
            windex: Index::new(),
            rindex: Index::new(),
        }
    }

    pub fn empty(&self) -> bool {
        // The fifo is empty if the indexes point to the same position
        // and have the same carry
        self.windex.get() == self.rindex.get() &&
            self.windex.carry() == self.rindex.carry()
    }

    pub fn full(&self) -> bool {
        // The fifo is full if the indexes point to the same position
        // and have different carries
        self.windex.get() == self.rindex.get() &&
            self.windex.carry() != self.rindex.carry()
    }

    pub fn push(&mut self, val: T) -> Result<(), ()> {
        if self.full() {
            Err(())
        } else {
            self.buffer[self.windex.get()] = val;

            self.windex.bump();

            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.empty() {
            None
        } else {
            let v = self.buffer[self.rindex.get()];

            self.rindex.bump();

            Some(v)
        }
    }

    pub fn len(&self) -> usize {
        self.windex - self.rindex
    }

    pub fn capacity(&self) -> usize {
        BUFFER_SIZE
    }
}

#[derive(Copy)]
struct Index(usize);

impl Index {
    fn new() -> Index {
        Index(0)
    }

    fn get(self) -> usize {
        let Index(i) = self;

        i % BUFFER_SIZE
    }

    fn carry(self) -> bool {
        let Index(i) = self;

        // Use the fact that BUFFER_SIZE is a power of two.
        i & BUFFER_SIZE != 0
    }

    fn bump(&mut self) {
        let Index(i) = *self;

        *self = Index(i + 1);
    }
}


impl ::std::ops::Sub for Index {
    type Output = usize;

    fn sub(self, other: Index) -> usize {
        let Index(a) = self;
        let Index(b) = other;

        // Thanks to two's complement magic (and the fact that the
        // buffer size is always a power of two) this will always
        // compute the accurate distance between `a` and `b` even in
        // case of index wrapping.
        a - b
    }
}

/// Logarithm in base 2 of the buffer size.
const BUFFER_SIZE_LN:   usize = 12;

const BUFFER_SIZE:      usize = 1 << BUFFER_SIZE_LN;
