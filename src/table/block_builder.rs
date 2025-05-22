use std::error::Error;

use crate::common::{Options, Slice};

struct BlockBuilder {
    options: Options,
    buffer: String,
    restarts: Vec<u32>,
    counter: u32,
    finished: bool,
}


impl BlockBuilder {
    pub fn new(options: Options) -> Self {
        BlockBuilder {
            options: options,
            buffer: String::new(),
            restarts: Vec::new(),
            counter: 0,
            finished: false,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.restarts.clear();
        self.counter = 0;
    }

    pub fn add(&mut self, key: &Slice, value: &Slice) -> Result<(), Box<dyn Error>> {
        if self.finished {
            return Err("BlockBuilder is finished".into());
        }

        if self.restarts.is_empty() {
            self.restarts.push(0);
        }
        
        

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn current_size_estimate(&self) -> usize {
        self.buffer.len()
    }
}
