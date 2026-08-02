use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

use anyhow::Result;
use evdev::{Device, EventSummary, KeyCode};

pub struct Keyboard {
    //  low 32 bits: keycode (16 bits actually, 0 == none)
    // high 32 bits: value   (unused)
    //
    // TODO: event based
    pub data: Arc<AtomicU64>,
    pub size: usize,
}

impl Keyboard {
    pub fn new(tx: mpsc::Sender<bool>, grab_keyboard: bool) -> Self {
        let data = Arc::new(AtomicU64::new(0));

        let mut devices = evdev::enumerate()
            .filter_map(|(_, mut dev)| {
                if dev_is_keyboard(&dev) {
                    if grab_keyboard {
                        dev.grab().ok()?;
                    }
                    Some(dev)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        {
            let data = data.clone();
            thread::spawn(move || -> Result<()> {
                let mut ctrl = false;

                loop {
                    let mut has_event = false;

                    for dev in &mut devices {
                        for event in dev.fetch_events()? {
                            has_event = true;

                            if let EventSummary::Key(_, code, value) = event.into() {
                                match value {
                                    // Pressed
                                    1 => {
                                        if matches!(
                                            code,
                                            KeyCode::KEY_LEFTCTRL | KeyCode::KEY_RIGHTCTRL
                                        ) {
                                            ctrl = true;
                                        }

                                        if ctrl && code == KeyCode::KEY_C {
                                            let _ = tx.send(true);
                                        }

                                        data.store(code.0 as u64, Ordering::Relaxed);
                                    },
                                    // Released
                                    0 => {
                                        if matches!(
                                            code,
                                            KeyCode::KEY_LEFTCTRL | KeyCode::KEY_RIGHTCTRL
                                        ) {
                                            ctrl = false;
                                        }

                                        data.store(0, Ordering::Relaxed);
                                    },
                                    // Repeated
                                    3 => {},
                                    _ => {},
                                }
                            }
                        }
                    }

                    if !has_event {
                        thread::sleep(Duration::from_millis(1));
                    }
                }
            });
        }

        Self { data, size: 8 }
    }
}

fn dev_is_keyboard(dev: &Device) -> bool {
    dev.supported_keys().is_some_and(|keys| {
        keys.contains(KeyCode::KEY_A)
            && keys.contains(KeyCode::KEY_Z)
            && keys.contains(KeyCode::KEY_ENTER)
    })
}
