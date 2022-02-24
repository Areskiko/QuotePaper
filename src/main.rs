extern crate cairo;
use cairo::{ ImageSurface, Format, Context };
use log::{info, error};
use quote_paper::{get_settings, paint_background, paint_text, setup_logger, save_to_file};



fn main() {
    match setup_logger() {
        Ok(_) => info!("Logger setup"),
        Err(e) => panic!("Couldn't setup logger: {}", e),
    }

    let settings = get_settings();

    let surface_res = ImageSurface::create(Format::ARgb32, settings.width, settings.height);
    if let Ok(surface) = surface_res {
        let context = Context::new(&surface);

        paint_background(&context, &settings);
        paint_text(&context, &settings);

        save_to_file(&settings, &surface);
    } else {
        error!("Couldn't create surface");
    }
}
