#[derive(Debug, Copy, Clone)]
pub struct Dot3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Dot3D {
    pub fn new(x: f64, y: f64, z: f64) -> Dot3D {
        Dot3D {
            x,
            y,
            z,
        }
    }

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn get_position(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn distance_to(&self, other: &Dot3D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    pub fn midpoint(&self, other: &Dot3D) -> Dot3D {
        Dot3D::new(
            (self.x + other.x) / 2.0,
            (self.y + other.y) / 2.0,
            (self.z + other.z) / 2.0,
        )
    }
}

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow};

trait EventHandler {
    fn on_init(&mut self) {}
    fn on_window_event(&mut self, event: &WindowEvent) {}
    fn on_event_loop_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {}
}

impl EventHandler for Dot3D {
    fn on_init(&mut self) {
        println!("Dot Init");
    }

    fn on_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => println!("Window closed"),
            _ => (),
        }
    }

    fn on_event_loop_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::LoopDestroyed => println!("Event loop destroyed"),
            _ => (),
        }

        *control_flow = ControlFlow::Exit;
    }
}