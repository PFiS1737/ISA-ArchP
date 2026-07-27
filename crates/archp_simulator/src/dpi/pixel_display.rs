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

use crate::dpi::memory::MEMORY;

pub struct PixelDisplay {
    scale: usize,

    canvas: Option<Canvas<Window>>,
    texture: Option<Texture>,

    event_pump: Option<EventPump>,

    scancode: Scancode,
}

impl PixelDisplay {
    pub fn new() -> Self {
        Self {
            scale: 0,
            canvas: None,
            texture: None,
            event_pump: None,
            scancode: Scancode::Unknown,
        }
    }

    pub fn init(&mut self, (fb_width, fb_height): (usize, usize), scale: usize) -> Result<()> {
        self.scale = scale;

        let sdl = sdl3::init()?;
        let video = sdl.video()?;

        let window = video
            .window(
                "PixelDisplay",
                (fb_width * scale) as u32,
                (fb_height * scale) as u32,
            )
            .position_centered()
            .build()?;

        self.canvas = Some(window.into_canvas());
        self.texture = Some(
            self.canvas
                .as_ref()
                .unwrap()
                .texture_creator()
                .create_texture_target(
                    Some(PixelFormat::RGBA8888),
                    fb_width as u32,
                    fb_height as u32,
                )?,
        );
        self.texture
            .as_mut()
            .unwrap()
            .set_scale_mode(ScaleMode::Nearest);

        self.event_pump = Some(sdl.event_pump()?);

        Ok(())
    }

    pub fn commit(&mut self, fb_data: &[u8], fb_width: usize) -> Result<()> {
        let canvas = self.canvas.as_mut().unwrap();
        let texture = self.texture.as_mut().unwrap();

        texture.update(None, fb_data, fb_width * size_of::<u32>())?;

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
extern "C" fn pixel_display_set(x: u32, y: u32, color: u32) {
    PIXEL_DISPLAY.with(|pd| {
        let mut memory = MEMORY.get().unwrap().lock().unwrap();

        let fb = memory.get_fb();
        fb.set_pixel(x as usize, y as usize, color);

        let mut pd = pd.borrow_mut();
        pd.commit(&fb.data, fb.width).unwrap();
    });
}

#[unsafe(no_mangle)]
extern "C" fn keyboard_get() -> u32 {
    PIXEL_DISPLAY.with(|pd| pd.borrow_mut().scancode as i32 as u32)
}
