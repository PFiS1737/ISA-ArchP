#[derive(Debug)]
pub struct Ram {
    pub data: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn reset(&mut self) {
        self.data.fill(0);
    }
}
