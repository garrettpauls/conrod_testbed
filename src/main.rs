#[macro_use]
extern crate conrod_core;
#[macro_use]
extern crate conrod_derive;
extern crate conrod_glium;

mod components;
mod support;
mod systems;
mod window;

fn main() {
    window::run();
}
