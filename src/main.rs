extern crate cairo;
use cairo::{ ImageSurface, Format, Context };
use log::{info, error};
use quote_paper::{get_settings, setup_logger, save_to_file, painting::{paint_background, paint_text}, source::rr::RRQuote, input::structs::QuoteSource};




fn main() {

    match setup_logger() {
        Ok(_) => info!("Logger setup"),
        Err(e) => panic!("Couldn't setup logger: {}", e),
    }

    let settings = get_settings();

    let surface_res = ImageSurface::create(Format::ARgb32, settings.width as i32, settings.height as i32);
    if let Ok(surface) = surface_res {
        let context = Context::new(&surface);

        paint_background(&context, &settings);
        paint_text(&context, &settings, &mut RRQuote::new());

        save_to_file(&settings, &surface);
    } else {
        error!("Couldn't create surface");
    }
}
