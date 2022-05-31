//const BLACK: Color = Color::RGB(0, 0, 0);
//const BLUE: Color = Color::RGB(1, 2, 233);
pub const TEXTURE_W: u32 = 16;
pub const TEXTURE_H: u32 = 12;

use crate::{load_scene_name, Scene, ViewZone};
use rayon::ThreadPool;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};

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
        let window = sdl_context
            .video()
            .unwrap()
            .window("f", screen_w, screen_h)
            .opengl()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let views =
            ViewZone::fullratio().split_n_ratio(rendering_options.split, rendering_options.split);

        //let texture_creator = canvas.texture_creator();
        //let one_texture = texture_creator
        //    .create_texture_streaming(PixelFormatEnum::RGB24, TEXTURE_W, TEXTURE_H)
        //    .unwrap();

        let scene = load_scene_name(scene_name);

        SdlGame {
            sdl_context,

            //window: window,
            pool,
            canvas,

            views,
            //one_texture: &one_texture,
            scene,
            rendering_options,
            player_moves: PlayerMoves::default(),
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
                        self.views = ViewZone::fullratio().split_n_ratio(
                            self.rendering_options.split,
                            self.rendering_options.split,
                        );
                    } else {
                        self.rendering_options.split += 1;
                        self.views = ViewZone::fullratio().split_n_ratio(
                            self.rendering_options.split,
                            self.rendering_options.split,
                        );
                    }
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

                    if keymod.contains(Mod::LSHIFTMOD) && self.rendering_options.depth > 1 {
                        self.rendering_options.depth -= 1
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

        if self.rendering_options.parallel_mode == PARALLEL_OFF {
            self.views
                .iter()
                .map(|vp| {
                    self.scene.render_zone_to_texture(
                        TEXTURE_W as usize,
                        TEXTURE_H as usize,
                        vp,
                        self.rendering_options.depth as u64,
                        texture,
                    );
                    self.canvas
                        .copy(&texture, None, vp.to_sceen_rect(800, 600))
                        .unwrap();
                })
                .for_each(drop);
        }
        if self.rendering_options.parallel_mode == PARALLEL_ON {
            let (tx, rx) = std::sync::mpsc::channel();

            self.views
                .iter()
                .map(|vp| {
                    //scene.render_zone_to_texture(texture_w as usize, texture_h as usize, vp, 5, t);
                    //canvas.copy(&t, None, vp.to_sceen_rect(800, 600)).unwrap();

                    let tx = tx.clone();

                    let sc_copy = self.scene.clone();
                    let vp_copy = vp.clone();
                    let depth = self.rendering_options.depth;
                    self.pool.spawn(move || {
                        let mut buff = [0u8; 3 * TEXTURE_W as usize * TEXTURE_H as usize];
                        //&sc_copy;
                        sc_copy.render_zone_to_buff(
                            TEXTURE_W as usize,
                            TEXTURE_H as usize,
                            &vp_copy,
                            depth as u64,
                            &mut buff,
                            TEXTURE_W as usize,
                        );

                        let r = vp_copy.to_sceen_rect(800, 600);
                        tx.send((buff, r)).unwrap();
                    });
                })
                .for_each(drop);

            drop(tx); // need to close all senders, otherwise...

            //let hashes: Vec<([u8; 3 * texture_h as usize * texture_w as usize], Rect)> =
            //    rx.into_iter().collect(); // ... this would block

            rx.iter()
                .map(|(buff, r)| {
                    texture
                        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                            buffer.copy_from_slice(&buff);
                        })
                        .unwrap();

                    self.canvas.copy(&texture, None, r).unwrap();
                })
                .for_each(drop);
        }

        //self.canvas.present();
    }
}

pub struct PlayerMoves {
    moving_forward: bool,
    moving_back: bool,
    mousex: i32,
    mousey: i32,
}
impl PlayerMoves {
    pub fn default() -> PlayerMoves {
        PlayerMoves {
            moving_forward: false,
            moving_back: false,
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
}
impl RenderingOpts {
    pub fn default() -> RenderingOpts {
        RenderingOpts {
            split: 1,
            depth: 2,
            enable_bp: true,
            parallel_mode: false,
        }
    }
}
