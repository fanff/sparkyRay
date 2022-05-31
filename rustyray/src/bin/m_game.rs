use rustyray::game::{SdlGame, TEXTURE_H, TEXTURE_W};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use std::time::Instant;

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

        loop_dur = (Instant::now() - strt_loop_time).as_secs_f64();

        // blit debug info
        //let loop_durstr = format!("{:?}", loop_dur);
        let surface = font
            .render(
                format!(
                    "{:.1} fps \nthreading: {:}\ndepth:{:}\nsplit:{:}\n{:?}",
                    1.0 / loop_dur,
                    g.rendering_options.parallel_mode,
                    g.rendering_options.depth,
                    (g.rendering_options.split as u32).pow(2),
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

        //let t = format!("{:?}", 1.0 / loop_dur).as_str();
    }
    Ok(())
}
