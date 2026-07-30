use std::fs::OpenOptions;

use anyhow::{Result, anyhow};
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
    pub data: DumbMapping<'a>,
}

impl<'a> FrameBuffer<'a> {
    pub fn new(dev: &str, w: usize, h: usize) -> Result<Self> {
        // TODO: other backends?
        let card = Card::open(dev)?;

        let (connector, crtc) = card.get_res()?;

        // TODO: show valid modes while erroring
        let &mode = connector
            .modes()
            .iter()
            .find(|mode| mode.size() == (w as u16, h as u16))
            .ok_or(anyhow!("No modes found for connector"))?;

        let (width, height) = mode.size();

        let mut db = Box::new(
            card.create_dumb_buffer((width.into(), height.into()), DrmFourcc::Xrgb8888, 32)
                .map_err(|err| anyhow!("Could not create dumb buffer: {err}"))?,
        );

        let fb = card
            .add_framebuffer(db.as_ref(), 24, 32)
            .map_err(|err| anyhow!("Could not create framebuffer: {err}"))?;

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
        .map_err(|err| anyhow!("Could not set CRTC: {err}"))?;

        Ok(Self {
            card,
            width: width as usize,
            height: height as usize,
            db,
            fb,
            data,
        })
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
    fn open(path: &str) -> Result<Self> {
        Ok(Card(OpenOptions::new().read(true).write(true).open(path)?))
    }

    fn get_res(&self) -> Result<(connector::Info, crtc::Info)> {
        let res = self.resource_handles()?;

        let connector = res
            .connectors()
            .iter()
            .flat_map(|con| self.get_connector(*con, true))
            .find(|i| i.state() == connector::State::Connected)
            .ok_or(anyhow!("No connected connectors"))?;

        let crtc = res
            .crtcs()
            .iter()
            .flat_map(|crtc| self.get_crtc(*crtc))
            .next()
            .ok_or(anyhow!("No crtcs found"))?;

        Ok((connector, crtc))
    }
}
