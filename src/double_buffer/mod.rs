use std::sync::Mutex;

#[derive(Copy, Clone)]
pub struct RawBuffer {
    buffer: [u8; 4096],
    len: u32,
}

impl Default for RawBuffer {
    fn default() -> Self {
        RawBuffer {
            buffer: [0; 4096],
            len: 0,
        }
    }
}

pub struct DoubleBuffer {
    buffer_array: [std::sync::Mutex<RawBuffer>; 2],
    write_buffer_idx: usize,
}

impl DoubleBuffer {
    pub fn new() -> Self {
        return DoubleBuffer {
            buffer_array: [
                Mutex::new(RawBuffer::default()),
                Mutex::new(RawBuffer::default()),
            ],
            write_buffer_idx: 0,
        };
    }

    fn getWriteIndex(&self) -> usize {
        return self.write_buffer_idx;
    }

    fn getReadIndex(&self) -> usize {
        if self.write_buffer_idx == 1 {
            return 0;
        } else {
            return 1;
        }
    }

    pub fn getForWrite(&mut self) -> std::sync::MutexGuard<RawBuffer> {
        let write_idx = self.getWriteIndex();
        return self.buffer_array[write_idx]
            .lock()
            .expect("write lock failed");
    }

    pub fn getForRead(&self) -> std::sync::MutexGuard<RawBuffer> {
        let read_idx = self.getReadIndex();
        return self.buffer_array[read_idx]
            .lock()
            .expect("read lock failed");
    }

    pub fn switch(&mut self) {
        let _readGuard = self.buffer_array[0].lock();
        let _writeGuard = self.buffer_array[1].lock();

        match self.write_buffer_idx {
            0 => {
                self.write_buffer_idx = 1;
            }

            1 => {
                self.write_buffer_idx = 0;
            }

            _ => {
                panic!("write_buffer_idx out of bounds");
            }
        }
    }
}
