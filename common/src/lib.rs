pub trait App {
    fn run<E: EventHandler>(self, handler: E);
}

pub trait EventHandler {
    fn on_exit(&mut self);
    fn on_key_pressed(&mut self, key_code: u32);
}
