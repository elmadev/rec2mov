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
// use image::ImageBuffer;
// use image::png::PNGEncoder;
use std::iter;

// use std::collections::HashMap;

fn main() {
    // let's start by converting pcx to png
    let lgr = LGR::load("Default.lgr").unwrap();
    // let mut pictures: HashMap<String, ?> = HashMap::new();

    for picture in lgr.picture_data.iter().take(1) {
        println!("{:?}", picture.name);
        let mut reader = pcx::Reader::new(picture.data.as_slice()).unwrap();
        // dumb thing requires buffer that is img width, so fill capacity up to length
        let mut img_buffer = Vec::with_capacity(reader.width() as usize);
        img_buffer.extend(iter::repeat(0).take(reader.width() as usize));

        // iterate through rows in pcx
        for _y in 0..reader.height() {
            reader.next_row_paletted(&mut img_buffer).unwrap();
            for px in img_buffer.iter() {
                print!("{}", if *px > 100 { "*" } else { "#" });
            }
            print!("\n");
        }
        // let image = ImageBuffer::from_raw(width: u32, height: u32, buf: Container);
        // let png_image: Vec<u8> = vec![];
        // let pngenc = PNGEncoder::new(png_image);
        // pngenc.encode(data: &[u8], width: u32, height: u32, color: ColorType);
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
