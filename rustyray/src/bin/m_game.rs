use rustyray::game::{SdlGame, TEXTURE_H, TEXTURE_W};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use std::time::{Duration, Instant};
extern crate blas_src;

fn main() -> Result<(), String> {
    let mut g = SdlGame::new(800, 600, "scene1.json".to_string());

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let fontloc = "/usr/share/fonts/dejavu-sans-mono-fonts/DejaVuSansMono.ttf";

    let mut font = ttf_context.load_font(fontloc, 14)?;
    // TextureCreator<Texture>
    let mut t_creator = g.canvas.texture_creator();
    let mut tex = t_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, TEXTURE_W, TEXTURE_H)
        .unwrap();

    let mut strt_loop_time;
    let mut loop_dur: f64 = 1.0;
    loop {
        strt_loop_time = Instant::now();
        if let Err(String) = g.events() {
            break;
        }

        //unwrap();
        g.update(loop_dur);
        g.render(&mut tex);
        g.canvas.present();

        // blit debug info
        //let loop_durstr = format!("{:?}", loop_dur);
        let surface = font
            .render(
                format!(
                    "{:.1} fps \norigin: {:}\ndir:{:}\nsplit:{:}\n{:?}",
                    1.0 / loop_dur,
                    g.scene.camera.origin,
                    g.scene.camera.dir,
                    (g.rendering_options.split as u32),
                    g.rendering_options,
                )
                .as_str(),
            )
            .blended_wrapped(Color::RGB(255, 0, 0), 400)
            .map_err(|e| e.to_string())?;
        let texture = t_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        g.canvas
            .copy(&texture, None, Rect::new(0, 0, width, height))?;
        g.canvas.present();

        if g.rendering_options.limit_fps {
            let durnano = (Instant::now() - strt_loop_time).as_nanos();

            let target_dur_nano = (1_000_000_000u128 / 30);
            if durnano < target_dur_nano {
                ::std::thread::sleep(Duration::new(0, (target_dur_nano - durnano) as u32));
            }
            loop_dur = (Instant::now() - strt_loop_time).as_secs_f64();
        } else {
            loop_dur = (Instant::now() - strt_loop_time).as_secs_f64();
        }

        //let t = format!("{:?}", 1.0 / loop_dur).as_str();
    }
    Ok(())
}
