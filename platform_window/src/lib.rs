use common::{App, EventHandler};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

#[derive(Default)]
struct Runtime<E: EventHandler> {
    window: Option<Window>,
    event_handler: E,
}

impl<E: EventHandler> ApplicationHandler for Runtime<E> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.event_handler.on_exit();
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        repeat: false,
                        physical_key,
                        ..
                    },
                ..
            } => {
                self.event_handler.on_key_pressed(key_to_u32(physical_key));
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Released,
                        repeat: false,
                        physical_key,
                        ..
                    },
                ..
            } => {}
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub struct PlatformWindow {}

impl App for PlatformWindow {
    fn run<E: EventHandler>(self, handler: E) {
        let event_loop = match EventLoop::new() {
            Ok(val) => val,
            Err(e) => panic!("Failed to create event loop with error: {:?}", e),
        };
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut runtime = Runtime {
            event_handler: handler,
            window: None,
        };
        event_loop.run_app(&mut runtime);
    }
}

fn key_to_u32(key: PhysicalKey) -> u32 {
    match key {
        PhysicalKey::Code(KeyCode::Backquote) => KeyCode::Backquote as u32,
        PhysicalKey::Code(KeyCode::Backslash) => KeyCode::Backslash as u32,
        PhysicalKey::Code(KeyCode::BracketLeft) => KeyCode::BracketLeft as u32,
        PhysicalKey::Code(KeyCode::BracketRight) => KeyCode::BracketRight as u32,
        PhysicalKey::Code(KeyCode::Comma) => KeyCode::Comma as u32,
        PhysicalKey::Code(KeyCode::Digit0) => KeyCode::Digit0 as u32,
        PhysicalKey::Code(KeyCode::Digit1) => KeyCode::Digit1 as u32,
        PhysicalKey::Code(KeyCode::Digit2) => KeyCode::Digit2 as u32,
        PhysicalKey::Code(KeyCode::Digit3) => KeyCode::Digit3 as u32,
        PhysicalKey::Code(KeyCode::Digit4) => KeyCode::Digit4 as u32,
        PhysicalKey::Code(KeyCode::Digit5) => KeyCode::Digit5 as u32,
        PhysicalKey::Code(KeyCode::Digit6) => KeyCode::Digit6 as u32,
        PhysicalKey::Code(KeyCode::Digit7) => KeyCode::Digit7 as u32,
        PhysicalKey::Code(KeyCode::Digit8) => KeyCode::Digit8 as u32,
        PhysicalKey::Code(KeyCode::Digit9) => KeyCode::Digit9 as u32,
        PhysicalKey::Code(KeyCode::Equal) => KeyCode::Equal as u32,
        PhysicalKey::Code(KeyCode::IntlBackslash) => KeyCode::IntlBackslash as u32,
        PhysicalKey::Code(KeyCode::IntlRo) => KeyCode::IntlRo as u32,
        PhysicalKey::Code(KeyCode::IntlYen) => KeyCode::IntlYen as u32,
        PhysicalKey::Code(KeyCode::KeyA) => KeyCode::KeyA as u32,
        PhysicalKey::Code(KeyCode::KeyB) => KeyCode::KeyB as u32,
        PhysicalKey::Code(KeyCode::KeyC) => KeyCode::KeyC as u32,
        PhysicalKey::Code(KeyCode::KeyD) => KeyCode::KeyD as u32,
        PhysicalKey::Code(KeyCode::KeyE) => KeyCode::KeyE as u32,
        PhysicalKey::Code(KeyCode::KeyF) => KeyCode::KeyF as u32,
        PhysicalKey::Code(KeyCode::KeyG) => KeyCode::KeyG as u32,
        PhysicalKey::Code(KeyCode::KeyH) => KeyCode::KeyH as u32,
        PhysicalKey::Code(KeyCode::KeyI) => KeyCode::KeyI as u32,
        PhysicalKey::Code(KeyCode::KeyJ) => KeyCode::KeyJ as u32,
        PhysicalKey::Code(KeyCode::KeyK) => KeyCode::KeyK as u32,
        PhysicalKey::Code(KeyCode::KeyL) => KeyCode::KeyL as u32,
        PhysicalKey::Code(KeyCode::KeyM) => KeyCode::KeyM as u32,
        PhysicalKey::Code(KeyCode::KeyN) => KeyCode::KeyN as u32,
        PhysicalKey::Code(KeyCode::KeyO) => KeyCode::KeyO as u32,
        PhysicalKey::Code(KeyCode::KeyP) => KeyCode::KeyP as u32,
        PhysicalKey::Code(KeyCode::KeyQ) => KeyCode::KeyQ as u32,
        PhysicalKey::Code(KeyCode::KeyR) => KeyCode::KeyR as u32,
        PhysicalKey::Code(KeyCode::KeyS) => KeyCode::KeyS as u32,
        PhysicalKey::Code(KeyCode::KeyT) => KeyCode::KeyT as u32,
        PhysicalKey::Code(KeyCode::KeyU) => KeyCode::KeyU as u32,
        PhysicalKey::Code(KeyCode::KeyV) => KeyCode::KeyV as u32,
        PhysicalKey::Code(KeyCode::KeyW) => KeyCode::KeyW as u32,
        PhysicalKey::Code(KeyCode::KeyX) => KeyCode::KeyX as u32,
        PhysicalKey::Code(KeyCode::KeyY) => KeyCode::KeyY as u32,
        PhysicalKey::Code(KeyCode::KeyZ) => KeyCode::KeyZ as u32,
        PhysicalKey::Code(KeyCode::Minus) => KeyCode::Minus as u32,
        PhysicalKey::Code(KeyCode::Period) => KeyCode::Period as u32,
        PhysicalKey::Code(KeyCode::Quote) => KeyCode::Quote as u32,
        PhysicalKey::Code(KeyCode::Semicolon) => KeyCode::Semicolon as u32,
        PhysicalKey::Code(KeyCode::Slash) => KeyCode::Slash as u32,
        _ => 0,
    }
}
