extern crate sdl2;

pub mod world;
pub mod entitybuilder;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

use sdl2::EventPump;
use sdl2::render::Renderer;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

use ::input::{InputState, PressedState};
use ::gfx::screen::Screen;
use ::math::rect::Rect;
use ::game::world::World;
use ::gfx::blit::Blit;

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
    pub screen: Rc<RefCell<Screen>>,
    pub world: Option<Rc<RefCell<World>>>
}

impl<'a> Game<'a> {
    pub fn new() -> Result<Game<'a>, String> {
        Ok(Game {
            system: try!(System::new("gbjam4")),
            input_state: InputState::new(),
            running: true,
            screen: Rc::new(RefCell::new(Screen::new())),
            world: None
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

        let mut path_buf = PathBuf::new();
        path_buf.push("assets");
        path_buf.push("test-img.png");
        let im = try!(::assets::load_image(path_buf));

        // Set screen colors
        self.screen.borrow_mut().colors = ::gfx::palettes::default_colors();

        //self.system.renderer.set_clip_rect()

        let mut render_texture: Texture = try!(self.system.renderer.create_texture_streaming(PixelFormatEnum::RGB888, (160, 144)));

        try!(self.system.renderer.set_logical_size(160u32, 144u32));

        // Create world
        self.world = Some(Rc::new(RefCell::new(World::new())));

        // Make an entity
        if let Some(ref mut w) = self.world {
            use std::ops::DerefMut;
            use ::game::entitybuilder::EntityBuilder;
            use ::math::Vector;
            use ::gfx::image::ImageDelegate::*;

            let mut world = w.borrow_mut();
            let world_ref: &mut World = world.deref_mut();
            let ent = EntityBuilder::new(world_ref)
                .position(Vector::new(0.0, 0.0))
                .sprite(ImageBuf(Rc::new(im)))
                .finish();
        }

        // Play. The. Game.
        while self.running {
            use std::thread;
            self.input_state.update();

            self.handle_events(&mut event_pump);

            // think and draw entities
            if let Some(ref mut w) = self.world {
                let entities_clone = w.borrow().clone_entities();
                for i in entities_clone.into_iter() {
                    if let Some(thinker) = w.borrow().thinker(i) {
                        thinker(w.clone(), i, self.input_state);
                    }
                }
                let entities_clone = w.borrow().clone_entities();

                for i in entities_clone.into_iter() {
                    if let Some(drawer) = w.borrow().drawer(i) {
                        drawer(w.clone(), self.screen.clone(), i);
                    } else {
                        // default drawer implementation
                        if let Some(sprite) = w.borrow().sprite(i) {
                            sprite.blit_to(None, &mut self.screen.borrow_mut().image, None);
                        }
                    }
                }
            }

            // copy custom screen buffer to render texture, mapping colors
            render_texture.with_lock(None, |buf, size| {
                let screen_b = self.screen.borrow_mut();
                for (i, x) in screen_b.image.buffer.iter().enumerate() {
                    let color = &screen_b.colors[*x as usize];
                    // It's BGR for some reason?
                    buf[(i * 4) + 0] = color[2];
                    buf[(i * 4) + 1] = color[1];
                    buf[(i * 4) + 2] = color[0];
                }
                ()
            }).unwrap();

            self.system.renderer.clear();
            self.system.renderer.copy(&render_texture, None, None);
            self.system.renderer.present();

            thread::sleep_ms((1000 / 60) as u32);
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
