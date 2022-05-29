extern crate sdl2;

use ndarray::arr1;
use rustyray::{load_scene_name, normalize, rotation_matrix};
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum::RGB888;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use std::borrow::BorrowMut;
use std::time::{Duration, Instant};

const BLACK: Color = Color::RGB(0, 0, 0);
const BLUE: Color = Color::RGB(1, 2, 233);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let window = sdl_context
        .video()
        .unwrap()
        .window("f", 800, 600)
        .opengl()
        .build()
        .unwrap();
    let mouse = sdl_context.mouse();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    let mut texture_w: u32 = 8;
    let mut texture_h: u32 = 6;

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, texture_w, texture_h)
        .map_err(|e| e.to_string())?;
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

    let mut scene = load_scene_name("scene1.json".to_string());

    let mut event_pump = sdl_context.event_pump()?;
    // let mut mng = FPSManager::new();
    // &mng.set_framerate(30)?;

    let mut moving_forward = false;
    let mut moving_back = false;
    let mut strt_loop_time: Instant;
    let mut loop_dur: f64 = 1.0;

    let mut mousex = 0;
    let mut mousey = 0;

    'running: loop {
        strt_loop_time = Instant::now();

        mousex = 0;
        mousey = 0;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { xrel, yrel, .. } => {
                    // println!("{:?}", event)
                    mousex += xrel;
                    mousey += yrel;
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    mouse.capture(true);
                    mouse.show_cursor(false);
                    mouse.set_relative_mouse_mode(true);
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    texture_h *= 2;
                    texture_w *= 2;

                    texture = texture_creator
                        .create_texture_streaming(PixelFormatEnum::RGB24, texture_w, texture_h)
                        .map_err(|e| e.to_string())?;
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    moving_forward = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    moving_forward = false;
                }

                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    moving_back = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    moving_back = false;
                }

                _ => {}
            }
        }

        // update
        if moving_forward {
            //scene.camera.origin = &scene.camera.origin + &scene.camera.dir * 10.0 * loop_dur;
            scene.camera.move_speed(10.0 * loop_dur);
        }
        if moving_back {
            scene.camera.move_speed(-10.0 * loop_dur);
            //scene.camera.origin = &scene.camera.origin - &scene.camera.dir * 10.0 * loop_dur;
        }

        if mousex != 0 {
            scene.camera.rot_angl(1.0 * loop_dur as f64 * mousex as f64);
        }

        if mousey != 0 {
            let vert_vel = 0.8 * loop_dur as f64;
            scene.camera.rot_ud(mousey as f64 * vert_vel);
        }

        // render render();
        scene.render_to_texture(
            texture_w as usize,
            texture_h as usize,
            5,
            texture.borrow_mut(),
        );
        //let img = scene.render(
        //    texture_w as usize,
        //    texture_h as usize,
        //    3,
        //    -1.0,
        //    1.1,
        //    -1.0,
        //    1.1,
        //);
        //
        //let array = img.as_raw().as_slice();
        //
        //texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        //    buffer.copy_from_slice(array);
        //})?;

        //canvas.set_draw_color(BLACK);
        //canvas.clear();
        //canvas.set_draw_color(BLUE);
        //canvas.fill_rect(Rect::new(10, 20, 5, 50));

        canvas.copy(&texture, None, Rect::new(0, 0, 800, 600))?;

        canvas.present();

        //let wstr = (format!("{}  fps", &mng.get_frame_count()));
        //println!("{}", &wstr);

        //&mng.delay();
        loop_dur = (Instant::now() - strt_loop_time).as_secs_f64();
        let t = format!("{:?}", 1.0 / loop_dur).as_str();
        println!("{:?}", 1.0 / loop_dur);
        // &window.set_title("t");
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
