use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::ptr::NonNull;
use std::time::Duration;
use std::fs;
use std::thread;
mod Game;
struct BackBone{
    sdl_context:sdl2::Sdl,
    video_subsystem:sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas,
    game: Game::Game,
}
impl BackBone{
    pub fn new() -> Self {
        let sdl_context=sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let  _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let mut  window = video_subsystem.window("Chess",1900,1080)
            .position_centered()
            .build()
            .expect("could not intitialize video subsystem");
        let mut canvas=window.into_canvas().build().unwrap();
        //let canvas = &window.into_canvas().build()
         //   .expect("could not make a canvas");
        //let texture_creator = canvas.texture_creator();
        Self{
            sdl_context:sdl_context,
            video_subsystem:video_subsystem,
            canvas:canvas,
            game:Game::Game::new(),
        }
        
       }
    fn render(&mut self,textures:&[Texture;12]){
         self.canvas.set_draw_color(Color::RGB(0,0,0));
         self.canvas.clear();
            
         self.game.renderComponents(&mut self.canvas,textures);

         self.canvas.present();
    }
    fn gameLoop(&mut self){
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let texture_creator = self.canvas.texture_creator();
        let mut pieces_textures:[Texture;12]=[
            texture_creator.load_texture("resources/bpawn.png").unwrap(),
            texture_creator.load_texture("resources/brook.png").unwrap(),
            texture_creator.load_texture("resources/bhorse.png").unwrap(),
            texture_creator.load_texture("resources/bbishop.png").unwrap(),
            texture_creator.load_texture("resources/bqueen.png").unwrap(),
            texture_creator.load_texture("resources/bking.png").unwrap(),
            texture_creator.load_texture("resources/wpawn.png").unwrap(),
            texture_creator.load_texture("resources/wrook.png").unwrap(),
            texture_creator.load_texture("resources/whorse.png").unwrap(),
            texture_creator.load_texture("resources/wbishop.png").unwrap(),
            texture_creator.load_texture("resources/wqueen.png").unwrap(),
            texture_creator.load_texture("resources/wking.png").unwrap(),
        ];
        let mut turn=1;
        self.game.putPieces();
        'running: loop {
            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    Event::KeyDown{ keycode: Some(Keycode::R), .. }=>{
                        self.game.putPieces();
                    }
                    Event::MouseButtonDown { x, y, .. } => {
                    if self.game.selected_piece.selected{
                        turn+=self.game.movePiece(x,y);
                    }else{

                        let piece=self.game.get_piece_by_clicking(x,y,turn);
                        if !piece.is_none() {
                            piece.unwrap().print();
                        }
                    }
                }
                    _ => {}
                }
            }

        self.render(&pieces_textures);

    }
    }
}

fn main() -> Result<(),String> {
    let mut backbone=BackBone::new();
    backbone.gameLoop();
    Ok(())
}
