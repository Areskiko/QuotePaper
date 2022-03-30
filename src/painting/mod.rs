use cairo::Context;
use log::debug;

use crate::{input::{parsing::get, structs::QuoteSource}, Settings};


pub fn paint_background(context: &Context, settings: &Settings) {
    context.set_source_rgb(settings.background_color.0, settings.background_color.1, settings.background_color.2);
    context.paint();
}

pub fn paint_text(context: &Context, settings: &Settings, source: Box<dyn QuoteSource>) {
    context.set_font_face(settings.font_face.to_cairo());
    context.set_font_size(settings.font_size);
    context.set_source_rgb(settings.font_color.0, settings.font_color.1, settings.font_color.2);
    let q = get(settings, source);
    let mut t = q.text;
    t.push('\"');
    t.insert(0, '\"');
    let mut a = q.author;
    a.insert_str(0, "- ");
    let mut lines = vec![];
    lines.push(t.split(' ').collect::<Vec<&str>>());
    let mut h = 0.0;

    // This should be refactored, but works for now
    let mut i = 0;
    while i < lines.len() {
        while context.text_extents(&lines.get(i).unwrap().join(" ")).width + (settings.padding * 2.0) > settings.width {
            let word = lines.get_mut(i).unwrap().pop();
            if let Some(word) = word {
                let word_clone = word.clone();
                let nl = lines.get_mut(i+1);
                if let Some(nl) = nl {
                    nl.insert(0, word_clone);
                } else {
                    lines.push(vec![word_clone]);
                }
            }
        }
        h += context.text_extents(&lines.get(i).unwrap().join(" ")).height + settings.spacing;
        i += 1;
    }

    for (i, line) in lines.iter().enumerate() {
        debug!("Writing: {}", line.join(" "));
        let ext = context.text_extents(&line.join(" "));
        context.move_to(settings.width/2.0 - ext.width/2.0, settings.height/2.0 - h/2.0 + (h/lines.len() as f64)*(i as f64));
        context.show_text(&line.join(" "));
    }

    let ext = context.text_extents(&a);
    context.move_to(settings.width - ext.width - settings.padding, settings.height - ext.height - settings.padding);
    context.show_text(&a);
}