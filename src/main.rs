// extern crate glutin_window;
// extern crate graphics;
// extern crate opengl_graphics;
// extern crate piston;
//
// use glutin_window::GlutinWindow as Window;
// use graphics::rectangle::square;
// use graphics::{clear, default_draw_state, Image};
// use opengl_graphics::{GlGraphics, OpenGL, Texture};
// use piston::event::*;
// use piston::window::WindowSettings;
// use std::path::Path;
extern crate elma;
extern crate image;
extern crate pcx;

use elma::lgr::*;
use std::iter;

// use std::collections::HashMap;

fn main() {
    // let's start by converting pcx to png
    let lgr = LGR::load("Default.lgr").unwrap();
    // let mut pictures: HashMap<String, ?> = HashMap::new();

    for picture in lgr.picture_data.iter() {
        let mut reader = pcx::Reader::new(picture.data.as_slice()).unwrap();
        let width = reader.width() as usize;
        let height = reader.height() as usize;
        // dumb thing requires buffer that is img width, so fill capacity up to length
        let mut img_buffer: Vec<u8> = iter::repeat(0).take(width).collect();

        // iterate through rows in pcx
        let mut pcx_pixels = Vec::with_capacity(width * height);
        for _y in 0..height {
            reader.next_row_paletted(&mut img_buffer).unwrap();
            pcx_pixels.extend_from_slice(&img_buffer);
        }

        let mut palette: Vec<u8> = iter::repeat(0).take(256 * 3).collect();
        reader.read_palette(&mut palette).unwrap();

        let png_pixels: Vec<_> = pcx_pixels
            .iter()
            .flat_map(|b| &palette[*b as usize * 3..*b as usize * 3 + 3])
            .map(|x| *x)
            .collect();

        image::save_buffer(
            picture.name.replace(".pcx", ".png"),
            &png_pixels,
            width as u32,
            height as u32,
            image::RGB(8),
        ).unwrap();
    }
    // let opengl = OpenGL::_3_2;
    // let mut gl = GlGraphics::new(opengl);
    // let window = Window::new(
    //     opengl,
    //     WindowSettings::new("Example", [600, 400]).exit_on_esc(true),
    // );
    //
    // //Create the image object and attach a square Rectangle object inside.
    // let image = Image::new().rect(square(0.0, 0.0, 200.0));
    // //A texture to use with the image
    // let texture = Texture::from_path(Path::new("Example.png")).unwrap();
    //
    // //Main loop
    // for e in window.events() {
    //     if let Some(r) = e.render_args() {
    //         gl.draw(r.viewport(), |c, gl| {
    //             //Clear the screen
    //             clear([0.0, 0.0, 0.0, 1.0], gl);
    //             //Draw the image with the texture
    //             image.draw(&texture, default_draw_state(), c.transform, gl);
    //         });
    //     }
    // }
}
