extern crate sdl2;

pub mod entity;

use self::sdl2::Sdl;
use self::sdl2::VideoSubsystem;
use sdl2::EventPump;
use sdl2::render::Renderer;

use ::input::{InputState, PressedState};
use ::gfx::screen::Screen;
use ::math::rect::Rect;

pub struct System<'a> {
    pub sdl: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub renderer: Renderer<'a>
}

impl<'a> System<'a> {
    pub fn new(title: &str) -> Result<System<'a>, String> {
        let sdl = try!(sdl2::init());
        let video = try!(sdl.video());
        let mut window_builder = video.window(title, 640, 576);
        let window = try!(window_builder.position_centered().resizable().build());
        let renderer = try!(window.renderer().build());

        Ok(System {
            sdl: sdl,
            video_subsystem: video,
            renderer: renderer
        })
    }
}

pub struct Game<'a> {
    pub system: System<'a>,
    pub input_state: InputState,
    pub running: bool,
    pub screen: Screen
}

impl<'a> Game<'a> {
    pub fn new() -> Result<Game<'a>, String> {
        Ok(Game {
            system: try!(System::new("gbjam4")),
            input_state: InputState::new(),
            running: true,
            screen: Screen::new()
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        use sdl2::pixels::PixelFormatEnum;
        use sdl2::render::Texture;
        use ::gfx::image::Image;

        // loop over events
        let mut event_pump: EventPump = match self.system.sdl.event_pump() {
            Ok(e) => e,
            Err(s) => return Err(s)
        };

        let mut im = Image::new((16, 16), 1u8);

        // Set screen colors
        self.screen.colors[0] = [0, 0, 0, 255];
        self.screen.colors[1] = [64, 64, 64, 255];
        self.screen.colors[2] = [170, 170, 170, 255];
        self.screen.colors[3] = [255, 255, 255, 255];

        //self.system.renderer.set_clip_rect()

        let mut render_texture: Texture = try!(self.system.renderer.create_texture_streaming(PixelFormatEnum::RGB888, (160, 144)));

        try!(self.system.renderer.set_logical_size(160u32, 144u32));

        while self.running {
            self.input_state.update();

            self.handle_events(&mut event_pump);
            // draw

            im.blit_to(None::<Rect>, &mut self.screen.image, None);

            // copy custom screen buffer to render texture, mapping colors
            render_texture.with_lock(None, |buf, size| {
                for (i, x) in self.screen.image.buffer.iter().enumerate() {
                    let color = &self.screen.colors[*x as usize];
                    buf[(i * 4) + 0] = color[0];
                    buf[(i * 4) + 1] = color[1];
                    buf[(i * 4) + 2] = color[2];
                }
                ()
            }).unwrap();

            self.system.renderer.clear();
            self.system.renderer.copy(&render_texture, None, None);
            self.system.renderer.present();
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
