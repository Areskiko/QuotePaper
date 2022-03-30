use std::fs::File;

pub mod input;
pub mod source;
pub mod painting;


use input::{structs::SourceType};
use serde::{Deserialize, Serialize};
use log::{error, info, LevelFilter};



const CONF_FILE: &str = "quote";
//const SCALE: u8 = 255;

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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

fn get_level(level: Level) -> LevelFilter {
    match level {
        Level::Debug => LevelFilter::Debug,
        Level::Info => LevelFilter::Info,
        Level::Warn => LevelFilter::Warn,
        Level::Error => LevelFilter::Error,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub width: f64,
    pub height: f64,
    padding: f64,
    spacing: f64,
    font_size: f64,
    filename: String,
    location: String,
    log_level: Level,
    source_type: SourceType,
    source_format: source::Source,
    font_color: (f64, f64, f64),
    background_color: (f64, f64, f64),
    font_face: Font,
}

impl Settings {
    pub fn location(&self) -> &String {
        &self.location
    }
    pub fn source_type(&self) -> &SourceType {
        &self.source_type
    }
    pub fn source_format(&self) -> &source::Source {
        &self.source_format
    }
}

impl ::std::default::Default for Settings {
    fn default() -> Self {
        Self {
            width: 1920.0,
            height: 1080.0,
            padding: 10.0,
            spacing: 10.0,
            log_level: Level::Error,
            filename: "output.png".to_string(),
            font_size: 60.0,
            font_face: Font {
                family: "Courier".to_string(),
                slant: Slant::Normal,
                weight: Weight::Normal,
            },
            font_color: (1.0, 1.0, 1.0),
            background_color: (35.0 / 255.0, 39.0 / 255.0, 46.0 / 255.0),
            location: "https://www.redrisingquotes.com/api/v1/random/".to_string(),
            source_type: SourceType::URL,
            source_format: source::Source::Basic,
        }
    }
}

pub fn get_settings() -> Settings {
    let load = confy::load(CONF_FILE);
    
    let settings;
    
    // This is a bad idea...
    if let Ok(load) = load {
        settings = load;
        match setup_logger(&settings) {
            Ok(_) => info!("Logger setup"),
            Err(e) => panic!("Couldn't setup logger: {}", e),
        }
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

pub fn setup_logger(settings: &Settings) -> Result<(), fern::InitError> {
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
        .level(get_level(settings.log_level))
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
