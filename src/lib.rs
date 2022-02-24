use std::fs::File;

use cairo::Context;
use serde::{Deserialize, Serialize};
use log::{error, info};

const CONF_FILE: &str = "quote";

#[derive(Serialize, Deserialize, Debug)]
enum Slant {
    Normal,
    Italic,
    Oblique,
}

#[derive(Serialize, Deserialize, Debug)]
enum Weight {
    Normal,
    Bold,
}

#[derive(Serialize, Deserialize, Debug)]
struct Font {
    family: String,
    slant: Slant,
    weight: Weight,
}

impl Font {
    fn to_cairo(&self) -> cairo::FontFace {
        let slant = match self.slant {
            Slant::Normal => cairo::FontSlant::Normal,
            Slant::Italic => cairo::FontSlant::Italic,
            Slant::Oblique => cairo::FontSlant::Oblique,
        };
        let weight = match self.weight {
            Weight::Normal => cairo::FontWeight::Normal,
            Weight::Bold => cairo::FontWeight::Bold,
        };
        cairo::FontFace::toy_create(&self.family, slant, weight)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub width: i32,
    pub height: i32,
    filename: String,
    font_size: f64,
    text: String,
    font_color: (f64, f64, f64),
    background_color: (f64, f64, f64),
    font_face: Font,
}

impl ::std::default::Default for Settings {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            filename: "output.png".to_string(),
            font_size: 60.0,
            font_face: Font {
                family: "Courier".to_string(),
                slant: Slant::Normal,
                weight: Weight::Normal,
            },
            font_color: (1.0, 1.0, 1.0),
            background_color: (35.0 / 255.0, 39.0 / 255.0, 46.0 / 255.0),
            text: "Scientia Invicta".to_string(),
        }
    }
}

pub fn get_settings() -> Settings {
    let load = confy::load(CONF_FILE);
    
    let settings;
    if let Ok(load) = load {
        settings = load;
        info!("Loaded settings from conf file");
    } else {
        settings = Settings::default();
        let res = confy::store(CONF_FILE, &settings);
        match res {
            Ok(_) => info!("Created default conf file"),
            Err(e) => error!("Couldn't create conf file: {}", e),
        }
    }
    settings
}

pub fn paint_background(context: &Context, settings: &Settings) {
    context.set_source_rgb(settings.background_color.0, settings.background_color.1, settings.background_color.2);
    context.paint();
}

pub fn paint_text(context: &Context, settings: &Settings) {
    context.set_font_face(settings.font_face.to_cairo());
    context.set_font_size(settings.font_size);
    context.set_source_rgb(settings.font_color.0, settings.font_color.1, settings.font_color.2);
    let ext = context.text_extents(&settings.text);
    context.move_to(settings.width / 2.0 - ext.width / 2.0, settings.height / 2.0 - ext.height / 2.0);
    context.show_text(&settings.text);
}

pub fn save_to_file(settings: &Settings, surface: &cairo::ImageSurface) {
    let file_res = File::create(&settings.filename);
    match file_res {
        Ok(mut file) => {
            let res = surface.write_to_png(&mut file);
            match res {
                Ok(_) => info!("Saved image to file"),
                Err(e) => error!("Couldn't save image to file: {}", e),
            }
        }
        Err(e) => error!("Couldn't create file: {}", e),
    }
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
