extern crate sdl2;

use self::sdl2::Sdl;
use self::sdl2::VideoSubsystem;
use self::sdl2::video::Window;
use sdl2::EventPump;

use ::input::{InputState, PressedState};
use ::gfx::screen::Screen;

pub struct System {
    pub sdl: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub window: Window
}

impl System {
    pub fn new(title: &str) -> Result<System, String> {
        let sdl = try!(sdl2::init());
        let video = try!(sdl.video());
        let mut window_builder = video.window(title, 160, 144);
        let window = try!(window_builder.position_centered().resizable().build());

        Ok(System {
            sdl: sdl,
            video_subsystem: video,
            window: window
        })
    }
}

pub struct Game {
    pub system: System,
    pub input_state: InputState,
    pub running: bool,
    pub screen: Screen
}

impl Game {
    pub fn new() -> Result<Game, String> {
        Ok(Game {
            system: try!(System::new("gbjam4")),
            input_state: InputState::new(),
            running: true,
            screen: Screen::new()
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        // loop over events
        let mut event_pump: EventPump = match self.system.sdl.event_pump() {
            Ok(e) => e,
            Err(s) => return Err(s)
        };

        while self.running {
            self.input_state.update();

            self.handle_events(&mut event_pump);
            // draw
        }

        Ok(())
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) -> () {
        for e in event_pump.poll_iter() {
            use sdl2::event::Event::*;
            match e {
                Quit { .. } => { self.running = false; }
                KeyUp { scancode, .. } => {
                    scancode.map(|s| { self.handle_key_up(s) });
                },
                KeyDown { scancode, .. } => {
                    scancode.map(|s| { self.handle_key_down(s) });
                },
                _ => (),
            };
        };
    }

    fn handle_key_up(&mut self, scancode: sdl2::keyboard::Scancode) -> () {
        use self::sdl2::keyboard::Scancode;

        match scancode {
            Scancode::Left => { self.input_state.left = PressedState::Up },
            Scancode::Right => { self.input_state.right = PressedState::Up },
            Scancode::Up => {self.input_state.up = PressedState::Up},
            Scancode::Down => {self.input_state.down = PressedState::Up},
            Scancode::Z => {self.input_state.a = PressedState::Up},
            Scancode::X => {self.input_state.b = PressedState::Up},
            Scancode::Return => {self.input_state.start = PressedState::Up},
            Scancode::RShift => {self.input_state.select = PressedState::Up},
            _ => return ()
        };
    }

    fn handle_key_down(&mut self, scancode: sdl2::keyboard::Scancode) -> () {
        use self::sdl2::keyboard::Scancode;

        match scancode {
            Scancode::Left => { self.input_state.left = PressedState::Pressed },
            Scancode::Right => { self.input_state.right = PressedState::Pressed },
            Scancode::Up => {self.input_state.up = PressedState::Pressed},
            Scancode::Down => {self.input_state.down = PressedState::Pressed},
            Scancode::Z => {self.input_state.a = PressedState::Pressed},
            Scancode::X => {self.input_state.b = PressedState::Pressed},
            Scancode::Return => {self.input_state.start = PressedState::Pressed},
            Scancode::RShift => {self.input_state.select = PressedState::Pressed},
            _ => return ()
        };
    }
}
