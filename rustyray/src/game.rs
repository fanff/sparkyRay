//const BLACK: Color = Color::RGB(0, 0, 0);
//const BLUE: Color = Color::RGB(1, 2, 233);
pub const TEXTURE_W: u32 = 16;
pub const TEXTURE_H: u32 = 16;

pub const TEXTURE_BYTE_SIZE: usize = (TEXTURE_H * TEXTURE_W * 3) as usize;

pub type TextureType = [u8; 3 * TEXTURE_H as usize * TEXTURE_W as usize];

use crate::{load_scene_name, Scene, ViewZone};
use rayon::ThreadPool;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::rect::Rect;

use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};

use sdl2::video::FullscreenType;
use sdl2::Sdl;

pub const PARALLEL_OFF: bool = false;
pub const PARALLEL_ON: bool = true;

pub struct SdlGame {
    pub sdl_context: Sdl,
    pub pool: ThreadPool,
    pub canvas: WindowCanvas,
    pub rendering_options: RenderingOpts,
    pub views: Vec<ViewZone>,
    pub scene: Scene,

    pub player_moves: PlayerMoves,
}
impl SdlGame {
    pub fn new(screen_w: u32, screen_h: u32, scene_name: String) -> SdlGame {
        let mut rendering_options = RenderingOpts::default();
        let pool = rayon::ThreadPoolBuilder::new().build().unwrap();

        let sdl_context = sdl2::init().unwrap();
        let mut window = sdl_context
            .video()
            .unwrap()
            .window("f", screen_w, screen_h)
            .resizable()
            //.input_grabbed()
            .opengl()
            .build()
            .unwrap();
        //window.set_fullscreen(FullscreenType::True).unwrap();
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let views = ViewZone::fullratio().split_n_ratio(1, 1);

        //let texture_creator = canvas.texture_creator();
        //let one_texture = texture_creator
        //    .create_texture_streaming(PixelFormatEnum::RGB24, TEXTURE_W, TEXTURE_H)
        //    .unwrap();

        let mut scene = load_scene_name(scene_name);

        let mut s = SdlGame {
            sdl_context,

            //window: window,
            pool,
            canvas,

            views,
            //one_texture: &one_texture,
            scene,
            rendering_options,
            player_moves: PlayerMoves::default(),
        };
        s.scene.camera.rot_angl(0.0);
        s.rebuild_views();
        s
    }
    pub fn rebuild_views(&mut self) {
        if self.rendering_options.bordering_split {
            self.views = ViewZone::fullratio().split_border_n_ratio(
                self.rendering_options.split,
                TEXTURE_W,
                TEXTURE_H,
            )
        } else {
            self.views = ViewZone::fullratio()
                .split_n_ratio(self.rendering_options.split, self.rendering_options.split);
        }
    }
    pub fn events(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Err("break".to_string());
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    let mouse = self.sdl_context.mouse();
                    mouse.capture(true);
                    mouse.show_cursor(false);
                    mouse.set_relative_mouse_mode(true);
                }
                Event::KeyDown {
                    keymod,
                    repeat: false,
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    if keymod.contains(Mod::LSHIFTMOD) && self.rendering_options.split > 1 {
                        self.rendering_options.split -= 1;
                        self.rebuild_views();
                    } else {
                        self.rendering_options.split += 1;
                        self.rebuild_views();
                    }
                }
                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    self.rendering_options.bordering_split =
                        !self.rendering_options.bordering_split;
                    self.rebuild_views();
                }

                Event::KeyDown {
                    keymod,
                    repeat: false,
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    // texture_h *= 2;
                    // texture_w *= 2;
                    //self.split += 1;
                    //self.views = ViewZone::fullratio().split_n_ratio(self.split, self.split);
                    //textures = make_textures(texture_w, texture_h, &views, &texture_creator);

                    if keymod.contains(Mod::LSHIFTMOD) {
                        if self.rendering_options.depth > 1 {
                            self.rendering_options.depth -= 1
                        }
                    } else {
                        self.rendering_options.depth += 1
                    }
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::T),
                    ..
                } => {
                    // texture_h *= 2;
                    // texture_w *= 2;
                    self.rendering_options.parallel_mode = !self.rendering_options.parallel_mode;
                    //views = ViewZone::fullratio().split_n_ratio(split, split);
                    //textures = make_textures(texture_w, texture_h, &views, &texture_creator);
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    // texture_h *= 2;
                    // texture_w *= 2;
                    self.rendering_options.limit_fps = !self.rendering_options.limit_fps;
                    //views = ViewZone::fullratio().split_n_ratio(split, split);
                    //textures = make_textures(texture_w, texture_h, &views, &texture_creator);
                }

                Event::MouseMotion { xrel, yrel, .. } => {
                    // println!("{:?}", event)
                    self.player_moves.mousex += xrel;
                    self.player_moves.mousey += yrel;
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    self.player_moves.moving_forward = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    self.player_moves.moving_forward = false;
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    self.player_moves.moving_back = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    self.player_moves.moving_back = false;
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    self.player_moves.moving_left = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    self.player_moves.moving_left = false;
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.player_moves.moving_right = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.player_moves.moving_right = false;
                }

                _ => {}
            }
        }
        Ok(())
    }
    pub fn update(&mut self, loop_dur: f64) {
        // update
        if self.player_moves.moving_forward {
            //scene.camera.origin = &scene.camera.origin + &scene.camera.dir * 10.0 * loop_dur;
            self.scene.camera.move_speed(10.0 * loop_dur);
        }
        if self.player_moves.moving_back {
            self.scene.camera.move_speed(-10.0 * loop_dur);
            //scene.camera.origin = &scene.camera.origin - &scene.camera.dir * 10.0 * loop_dur;
        }
        if self.player_moves.moving_left {
            //scene.camera.origin = &scene.camera.origin + &scene.camera.dir * 10.0 * loop_dur;
            self.scene.camera.move_side(-10.0 * loop_dur);
        }
        if self.player_moves.moving_right {
            self.scene.camera.move_side(10.0 * loop_dur);
            //scene.camera.origin = &scene.camera.origin - &scene.camera.dir * 10.0 * loop_dur;
        }

        if self.player_moves.mousex != 0 {
            self.scene
                .camera
                .rot_angl((0.5 * loop_dur as f64) * self.player_moves.mousex as f64);
        }

        if self.player_moves.mousey != 0 {
            let vert_vel = 0.4 * loop_dur as f64;
            self.scene
                .camera
                .rot_ud(self.player_moves.mousey as f64 * vert_vel);
        }

        self.player_moves.mousex = 0;
        self.player_moves.mousey = 0;
    }
    //:TextureCreator
    pub fn render(&mut self, texture: &mut Texture) {
        //let mut t_creator = self.canvas.texture_creator();
        //let tex = t_creator
        //    .create_texture_streaming(PixelFormatEnum::RGB24, TEXTURE_W, TEXTURE_H)
        //    .unwrap();
        let (window_size_w, window_size_h) = self.canvas.window().size();
        if self.rendering_options.parallel_mode == PARALLEL_OFF {
            self.views
                .iter()
                .map(|vp| {
                    let mut buff = [0u8; 3 * TEXTURE_W as usize * TEXTURE_H as usize];
                    //&sc_copy;
                    self.scene.render_zone_to_buff(
                        TEXTURE_W as usize,
                        TEXTURE_H as usize,
                        vp,
                        self.rendering_options.depth as u64,
                        &mut buff,
                    );

                    let r = vp.to_sceen_rect(window_size_w, window_size_h);

                    texture
                        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                            buffer.copy_from_slice(&buff);
                        })
                        .unwrap();

                    //self.scene.render_zone_to_texture(
                    //    TEXTURE_W as usize,
                    //    TEXTURE_H as usize,
                    //    vp,
                    //    self.rendering_options.depth as u64,
                    //    texture,
                    //);
                    self.canvas.copy(&texture, None, r).unwrap();
                })
                .for_each(drop);
        }
        if self.rendering_options.parallel_mode == PARALLEL_ON {
            let (tx, rx) = std::sync::mpsc::channel();
            //println!("render! ");

            self.views
                .iter()
                .enumerate()
                .map(|(vpidx, vp)| {
                    //scene.render_zone_to_texture(texture_w as usize, texture_h as usize, vp, 5, t);
                    //canvas.copy(&t, None, vp.to_sceen_rect(800, 600)).unwrap();
                    //println!("{:?}", vp.border);
                    let tx = tx.clone();

                    let sc_copy = self.scene.clone();
                    let vp_copy = vp.clone();
                    let depth = self.rendering_options.depth;
                    self.pool.spawn(move || {
                        //let mut buff2: TextureType;
                        let mut buff: TextureType = [0u8; TEXTURE_BYTE_SIZE];
                        //let mut buff = [0u8; 3 * TEXTURE_W as usize * TEXTURE_H as usize];
                        //&sc_copy;
                        sc_copy.render_zone_to_buff(
                            TEXTURE_W as usize,
                            TEXTURE_H as usize,
                            &vp_copy,
                            depth as u64,
                            &mut buff,
                        );

                        let r = vp_copy.to_sceen_rect(window_size_w, window_size_h);
                        tx.send((buff, r, vpidx)).unwrap();
                    });
                })
                .for_each(drop);

            drop(tx); // need to close all senders, otherwise...

            let mut hashes: Vec<(TextureType, Rect, usize)> = rx.into_iter().collect(); // ... this would block

            hashes.sort_by_key(|(t, r, idx)| *idx);

            //hashes.sort_by(|t1, t2| t1[2] > t1[2]);
            hashes
                .iter()
                .map(|(buff, r, vpidx)| {
                    texture
                        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                            buffer.copy_from_slice(buff);
                        })
                        .unwrap();
                    self.canvas.copy(&texture, None, *r).unwrap();
                })
                .for_each(drop);

            // directUnload
            // rx.iter()
            //     .map(|(buff, r)| {
            //         texture
            //             .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            //                 buffer.copy_from_slice(&buff);
            //             })
            //             .unwrap();
            //         self.canvas.copy(&texture, None, r).unwrap();
            //     })
            //     .for_each(drop);
        }

        //self.canvas.present();
    }
}

pub struct PlayerMoves {
    moving_forward: bool,
    moving_back: bool,
    moving_left: bool,
    moving_right: bool,
    mousex: i32,
    mousey: i32,
}
impl PlayerMoves {
    pub fn default() -> PlayerMoves {
        PlayerMoves {
            moving_forward: false,
            moving_back: false,
            moving_left: false,
            moving_right: false,
            mousex: 0,
            mousey: 0,
        }
    }
}

#[derive(Debug)]
pub struct RenderingOpts {
    pub split: u8,
    pub depth: u8,
    pub enable_bp: bool,
    pub parallel_mode: bool,
    pub limit_fps: bool,
    pub bordering_split: bool,
}
impl RenderingOpts {
    pub fn default() -> RenderingOpts {
        RenderingOpts {
            split: 1,
            depth: 2,
            enable_bp: true,
            parallel_mode: true,
            limit_fps: false,
            bordering_split: false,
        }
    }
    pub fn to_str(&self) -> String {
        let lol = format!("{:?}", self);
        lol
    }
}
