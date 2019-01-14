extern crate conrod_winit;

use std;
use glium;

pub struct GliumDisplayWinitWrapper(pub glium::Display);

impl conrod_winit::WinitWindow for GliumDisplayWinitWrapper {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        self.0.gl_window().get_inner_size().map(Into::into)
    }

    fn hidpi_factor(&self) -> f32 {
        self.0.gl_window().get_hidpi_factor() as _
    }
}

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
    frame_delay: std::time::Duration,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
            frame_delay: std::time::Duration::from_millis(16),
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) -> Vec<glium::glutin::Event> {
        let last_update = self.last_update;
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < self.frame_delay {
            std::thread::sleep(self.frame_delay - duration_since_last_update);
        }

        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}
