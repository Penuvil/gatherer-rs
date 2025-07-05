use common::{App, EventHandler};
use platform_window::PlatformWindow;

#[derive(Default)]
struct Gatherer {}

impl EventHandler for Gatherer {
    fn on_exit(&mut self) {
        println!("Shutting down The Gatherer");
    }

    fn on_key_pressed(&mut self, key_code: u32) {
        println!("Key Pressed with KeyCode: {}", key_code);
    }
}

fn main() {
    let window = PlatformWindow {};
    window.run(Gatherer {});
}
