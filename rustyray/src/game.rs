//const BLACK: Color = Color::RGB(0, 0, 0);
//const BLUE: Color = Color::RGB(1, 2, 233);
const TEXTURE_W: u32 = 16;
const TEXTURE_H: u32 = 12;

use crate::{load_scene_name, Scene, ViewZone};
use rayon::ThreadPool;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use sdl2::video::Window;
use sdl2::Sdl;

pub const PARALLEL_OFF: u8 = 0;
pub const PARALLEL_ON: u8 = 1;

pub struct Sdl_Game<'r> {
    sdl_context: Sdl,
    window: Window,
    pool: ThreadPool,

    canvas: WindowCanvas,

    depth: u64,
    split: u8,
    views: Vec<ViewZone>,
    one_texture: Texture<'r>,
    scene: Scene,
    parallel_mode: u8,
}
impl Sdl_Game<'static> {
    pub fn init(&mut self) {
        self.sdl_context = sdl2::init().unwrap();
        self.window = self
            .sdl_context
            .video()
            .unwrap()
            .window("f", 800, 600)
            .opengl()
            .build()
            .unwrap();
    }

    pub fn new<'r>(screen_w: u32, screen_h: u32, scene_name: String) -> Sdl_Game<'r> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();

        let sdl_context = sdl2::init().unwrap();
        let window = sdl_context
            .video()
            .unwrap()
            .window("f", 800, 600)
            .opengl()
            .build()
            .unwrap();
        //let mouse = sdl_context.mouse();

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        //canvas.clear();

        const texture_w: u32 = 16;
        const texture_h: u32 = 12;

        let depth = 3;
        let split = 1;
        let views = ViewZone::fullratio().split_n_ratio(split, split);

        //:
        //let mut textures = make_textures(texture_w, texture_h, &views, &texture_creator);

        //let texture_creator = canvas.texture_creator();
        let one_texture = canvas
            .texture_creator()
            .create_texture_streaming(PixelFormatEnum::RGB24, texture_w, texture_h)
            .unwrap();

        // let mut texture = texture_creator
        //     .create_texture_streaming(PixelFormatEnum::RGB24, texture_w, texture_h)
        //     .map_err(|e| e.to_string())?;
        // Create a red-green gradient
        //texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        //    for y in 0..texture_h {
        //        for x in 0..texture_w {
        //            let offset = y * pitch + x * 3;
        //            buffer[offset] = x as u8;
        //            buffer[offset + 1] = y as u8;
        //            buffer[offset + 2] = 0;
        //        }
        //    }
        //})?;

        let scene = load_scene_name(scene_name);
        let parallel_mode = PARALLEL_OFF;

        Sdl_Game {
            sdl_context: sdl_context,
            window: window,
            pool: pool,
            canvas: canvas,
            depth: 2,
            split: 1,
            views: views,
            one_texture: one_texture,
            scene: scene,
            parallel_mode: parallel_mode,
        }
    }
    pub fn update(&mut self, pmoves: &Player_moves, loop_dur: f64) {
        // update
        if pmoves.moving_forward {
            //scene.camera.origin = &scene.camera.origin + &scene.camera.dir * 10.0 * loop_dur;
            self.scene.camera.move_speed(10.0 * loop_dur);
        }
        if pmoves.moving_back {
            self.scene.camera.move_speed(-10.0 * loop_dur);
            //scene.camera.origin = &scene.camera.origin - &scene.camera.dir * 10.0 * loop_dur;
        }

        if pmoves.mousex != 0 {
            self.scene
                .camera
                .rot_angl((1.0 * loop_dur as f64) * pmoves.mousex as f64);
        }

        if pmoves.mousey != 0 {
            let vert_vel = 0.8 * loop_dur as f64;
            self.scene.camera.rot_ud(pmoves.mousey as f64 * vert_vel);
        }
    }
    pub fn render(&mut self) {}
}

pub struct Player_moves {
    moving_forward: bool,
    moving_back: bool,
    mousex: i32,
    mousey: i32,
}
