#[macro_use]
extern crate conrod_core;
#[macro_use]
extern crate conrod_derive;
extern crate conrod_glium;

mod support;
mod components;
mod window;

fn main() {
    window::run();
}
