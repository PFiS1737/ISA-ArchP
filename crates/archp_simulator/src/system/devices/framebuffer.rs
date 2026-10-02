use std::{fs::OpenOptions, path::PathBuf, sync::Mutex};

use anyhow::{Context, Result, anyhow};
use drm::{
    buffer::DrmFourcc,
    control::{
        Device, connector, crtc,
        dumbbuffer::{DumbBuffer, DumbMapping},
        framebuffer,
    },
};

pub struct FrameBuffer<'a> {
    card: Card,
    db: Box<DumbBuffer>,
    fb: framebuffer::Handle,

    pub width: usize,
    pub height: usize,
    pub data: Mutex<DumbMapping<'a>>,
}

impl<'a> FrameBuffer<'a> {
    pub fn new(dev: &PathBuf, w: usize, h: usize) -> Result<Self> {
        // TODO: other backends?
        let card = Card::open(dev)?;

        let (connector, crtc) = card.get_res()?;

        // TODO: show valid modes while erroring
        let &mode = connector
            .modes()
            .iter()
            .find(|mode| mode.size() == (w as u16, h as u16))
            .with_context(|| anyhow!("No modes found for connector"))?;

        let (width, height) = mode.size();

        let mut db = Box::new(
            card.create_dumb_buffer((width.into(), height.into()), DrmFourcc::Xrgb8888, 32)
                .with_context(|| anyhow!("Could not create dumb buffer"))?,
        );

        let fb = card
            .add_framebuffer(db.as_ref(), 24, 32)
            .with_context(|| anyhow!("Could not create framebuffer"))?;

        let mut data: DumbMapping<'a> =
            unsafe { std::mem::transmute(card.map_dumb_buffer(&mut db)?) };

        for px in data.as_mut().as_chunks_mut::<4>().0 {
            px.copy_from_slice(&0x404040_u32.to_le_bytes());
        }

        card.set_crtc(
            crtc.handle(),
            Some(fb),
            (0, 0),
            &[connector.handle()],
            Some(mode),
        )
        .with_context(|| anyhow!("Could not set CRTC"))?;

        Ok(Self {
            card,
            db,
            fb,
            width: width as usize,
            height: height as usize,
            data: Mutex::new(data),
        })
    }

    pub fn size(&self) -> usize {
        self.width * self.height * 4
    }

    pub fn set_pixel(&self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            panic!(
                "Pixel coordinates out of bounds: ({}, {}) for framebuffer size {}x{}",
                x, y, self.width, self.height
            );
        }

        let offset = (y * self.width + x) * 4;
        let mut data = self.data.lock().unwrap();
        data[offset..offset + 4].copy_from_slice(&color.to_le_bytes());
    }
}

impl Drop for FrameBuffer<'_> {
    fn drop(&mut self) {
        self.card.destroy_framebuffer(self.fb).unwrap();
        self.card.destroy_dumb_buffer(*self.db).unwrap();
    }
}

struct Card(std::fs::File);

impl std::os::unix::io::AsFd for Card {
    fn as_fd(&self) -> std::os::unix::io::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl drm::Device for Card {}
impl Device for Card {}

impl Card {
    fn open(path: &PathBuf) -> Result<Self> {
        Ok(Card(OpenOptions::new().read(true).write(true).open(path)?))
    }

    fn get_res(&self) -> Result<(connector::Info, crtc::Info)> {
        let res = self.resource_handles()?;

        let connector = res
            .connectors()
            .iter()
            .flat_map(|con| self.get_connector(*con, true))
            .find(|i| i.state() == connector::State::Connected)
            .with_context(|| anyhow!("No connected connectors"))?;

        let crtc = res
            .crtcs()
            .iter()
            .flat_map(|crtc| self.get_crtc(*crtc))
            .next()
            .with_context(|| anyhow!("No crtcs found"))?;

        Ok((connector, crtc))
    }
}
