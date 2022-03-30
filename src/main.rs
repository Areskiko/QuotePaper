extern crate cairo;
use cairo::{ ImageSurface, Format, Context };
use log::{error};
use quote_paper::{
    get_settings, save_to_file,
    painting::{paint_background, paint_text},
    source::{use_source, Source}
};



fn main() {

    let settings = get_settings();

    let surface_res = ImageSurface::create(Format::ARgb32, settings.width as i32, settings.height as i32);
    if let Ok(surface) = surface_res {
        let context = Context::new(&surface);
        let src = use_source(Source::RR);

        paint_background(&context, &settings);
        paint_text(&context, &settings, src);

        save_to_file(&settings, &surface);
    } else {
        error!("Couldn't create surface");
    }
}
