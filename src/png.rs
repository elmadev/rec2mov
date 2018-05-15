use elma::lgr::*;
use pcx;
use std::{iter, collections::HashMap, path::Path};

pub(crate) fn convert_pcx<P: AsRef<Path>>(file: P) -> HashMap<String, (Vec<u8>, u32, u32)> {
    // let's start by converting pcx to png
    let lgr = LGR::load(file).unwrap();

    // get palette
    // TODO: add as method in lgr mod, avoid .find() etc.
    let palette_picture = lgr.picture_data
        .iter()
        .find(|el| el.name.to_lowercase() == "q1body.pcx")
        .unwrap();

    let reader = pcx::Reader::new(palette_picture.data.as_slice()).unwrap();
    let mut palette: Vec<u8> = iter::repeat(0).take(256 * 3).collect();
    reader.read_palette(&mut palette).unwrap();

    lgr.picture_data
        .iter()
        .map(|picture| {
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

            // get transparency info
            // TODO: use hashmaps in lgr mod for easier search, no need to keep un-ordered list (?)
            let found_pic = lgr.picture_list.iter().find(|el| {
                el.name.to_lowercase() == picture.name.replace(".pcx", "").to_lowercase()
            });
            let mut is_texture = false;
            let mut transparency = Transparency::TopLeft;
            if let Some(pic) = found_pic {
                transparency = pic.transparency;
                if pic.picture_type == PictureType::Texture {
                    is_texture = true;
                }
            };

            // special texture files handled by elma we need to check for
            match picture.name.to_lowercase().as_str() {
                "qframe.pcx" | "qgrass.pcx" => is_texture = true,
                _ => {}
            }

            let transparent_index = match transparency {
                Transparency::TopLeft => pcx_pixels[0],
                Transparency::TopRight => pcx_pixels[width - 1],
                Transparency::BottomLeft => pcx_pixels[((width - 1) * height) - 1],
                Transparency::BottomRight => pcx_pixels[width * height - 1],
                _ => 0,
            };

            let rgba_pixels: Vec<_> = pcx_pixels
                .iter()
                .flat_map(|b| {
                    // only rgb part
                    let mut rgba: Vec<_> = palette[*b as usize * 3..*b as usize * 3 + 3].to_vec();
                    // add alpha based on picture properties
                    if is_texture {
                        rgba.push(255);
                    } else if transparency == Transparency::Palette && *b == palette[0] {
                        rgba.push(0);
                    } else if transparency != Transparency::Solid && *b == transparent_index {
                        rgba.push(0);
                    } else {
                        rgba.push(255);
                    }
                    rgba
                })
                .collect();

            // let png_file = vec![];
            // let png_encoder = png::PNGEncoder::new(png_file);
            // png_encoder
            //     .encode(&png_pixels, width as u32, width as u32, ColorType::RGBA(8))
            //     .unwrap();

            (
                picture.name.to_lowercase().clone(),
                (rgba_pixels, width as u32, height as u32),
            )
        })
        .collect()
}
