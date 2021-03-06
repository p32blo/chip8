extern crate sdl2;

use sdl2::render::{Renderer};
use sdl2::{Sdl};

pub struct Screen<'a> {
    pub renderer: Renderer<'a>,
    pub buffer: [u8; 64*32],
}

impl<'a> Screen<'a> { 
    pub fn new(ctx: &Sdl) -> Screen<'a> {
        let video   = ctx.video().unwrap();
        let window  = video.window("chip8", 64*10, 32*10)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        Screen {
            renderer: window.renderer().build().unwrap(),
            buffer: [0; 64*32]
        }
    }

    pub fn draw(&mut self) {
        self.renderer.present();
    }

    pub fn clear(&mut self) { 
        self.buffer = [0; 64*32];
    }
}
