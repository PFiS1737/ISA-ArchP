pub struct FrameBuffer {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl FrameBuffer {
    pub fn new(w: usize, h: usize) -> Self {
        let mut data = vec![0u8; w * h * 4];

        for px in data.as_chunks_mut::<4>().0 {
            px.copy_from_slice(&0x404040FF_u32.to_le_bytes());
        }

        Self {
            data,
            width: w,
            height: h,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            panic!(
                "Pixel coordinates out of bounds: ({}, {}) for framebuffer size {}x{}",
                x, y, self.width, self.height
            );
        }

        let offset = (y * self.width + x) * 4;
        self.data[offset..offset + 4].copy_from_slice(&color.to_le_bytes());
    }
}
