use std::cell::RefCell;

use anyhow::Result;
use sdl3::{
    EventPump,
    event::{Event, WindowEvent},
    keyboard::Scancode,
    pixels::PixelFormat,
    render::{Canvas, ScaleMode, Texture},
    video::Window,
};

pub struct PixelDisplay {
    w: usize,
    h: usize,
    scale: usize,

    canvas: Option<Canvas<Window>>,
    texture: Option<Texture>,
    framebuffer: Vec<u32>,

    event_pump: Option<EventPump>,

    pub scancode: Scancode,
}

impl PixelDisplay {
    pub fn new() -> Self {
        Self {
            w: 0,
            h: 0,
            scale: 0,
            canvas: None,
            texture: None,
            framebuffer: Vec::new(),
            event_pump: None,
            scancode: Scancode::Unknown,
        }
    }

    pub fn init(&mut self, w: usize, h: usize, scale: usize) -> Result<()> {
        self.w = w;
        self.h = h;
        self.scale = scale;

        let sdl = sdl3::init()?;
        let video = sdl.video()?;

        let window = video
            .window("PixelDisplay", (w * scale) as u32, (h * scale) as u32)
            .position_centered()
            .build()?;

        self.canvas = Some(window.into_canvas());
        self.texture = Some(
            self.canvas
                .as_ref()
                .unwrap()
                .texture_creator()
                .create_texture_target(Some(PixelFormat::RGBA8888), w as u32, h as u32)?,
        );
        self.texture
            .as_mut()
            .unwrap()
            .set_scale_mode(ScaleMode::Nearest);

        self.reset();

        self.event_pump = Some(sdl.event_pump()?);

        Ok(())
    }

    pub fn reset(&mut self) {
        self.framebuffer = vec![0x404040FF; self.w * self.h];
    }

    pub fn set(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.w || y >= self.h {
            panic!(
                "Coordinates out of bounds: x={} y={} w={} h={}",
                x, y, self.w, self.h
            );
        }

        self.framebuffer[y * self.w + x] = color;
    }

    pub fn commit(&mut self) -> Result<()> {
        let canvas = self.canvas.as_mut().unwrap();
        let texture = self.texture.as_mut().unwrap();

        texture.update(
            None,
            bytemuck::cast_slice(&self.framebuffer),
            self.w * size_of::<u32>(),
        )?;

        canvas.clear();
        canvas.copy(texture, None, None)?;
        canvas.present();

        Ok(())
    }

    pub fn handle_event(&mut self) -> bool {
        let event_pump = self.event_pump.as_mut().unwrap();

        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => return false,
                Event::Window {
                    win_event: WindowEvent::CloseRequested,
                    ..
                } => {
                    return false;
                },
                Event::KeyDown {
                    scancode: Some(sc), ..
                } => {
                    if sc == Scancode::Q {
                        return false;
                    }
                    self.scancode = sc;
                },
                Event::KeyUp { .. } => {
                    self.scancode = Scancode::Unknown;
                },
                _ => {},
            }
        }
        true
    }
}

thread_local! {
    pub static PIXEL_DISPLAY: RefCell<PixelDisplay> = RefCell::new(PixelDisplay::new());
}

#[unsafe(no_mangle)]
extern "C" fn pixel_display_reset() {
    PIXEL_DISPLAY.with(|pd| pd.borrow_mut().reset());
}

#[unsafe(no_mangle)]
extern "C" fn pixel_display_set(x: u32, y: u32, color: u32) {
    PIXEL_DISPLAY.with(|pd| {
        pd.borrow_mut().set(x as usize, y as usize, color);
        pd.borrow_mut().commit().unwrap();
    });
}

#[unsafe(no_mangle)]
extern "C" fn keyboard_get() -> u32 {
    PIXEL_DISPLAY.with(|pd| pd.borrow_mut().scancode as i32 as u32)
}
