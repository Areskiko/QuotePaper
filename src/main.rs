extern crate cairo;
use cairo::{ ImageSurface, Format, Context };
use quote::{get_settings, paint_background, paint_text};
use std::fs::File;


fn main() {

    let settings = get_settings();

    let surface = ImageSurface::create(Format::ARgb32, settings.width, settings.height)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);

    paint_background(&context, &settings);
    paint_text(&context, &settings);


    let mut file = File::create("output.png")
        .expect("Couldn't create file"); 
    surface.write_to_png(&mut file)
        .expect("Couldn't write to png");
}