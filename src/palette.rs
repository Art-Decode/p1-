// Colors tools
// Alexis Andre (@mactuitui)

use nannou::color::{Hsv, Rgb, Srgb};

pub struct Palette {
    pub colors: Vec<Rgb>,
    pub len: usize,
}

impl Palette {
    pub fn new(cols_hex: Vec<u32>) -> Self {
        let mut cols_rgb: Vec<Rgb> = cols_hex
            .into_iter()
            .map(|c| {
                let blue: u8 = (c & 0xFF) as u8;
                let green: u8 = ((c >> 8) & 0xFF) as u8;
                let red: u8 = ((c >> 16) & 0xFF) as u8;
                let c = Srgb::new(
                    red as f32 / 255.0,
                    green as f32 / 255.0,
                    blue as f32 / 255.0,
                );
                c
            })
            .collect();

        cols_rgb.sort_by(|&a, &b| {
            let ahsv: Hsv = a.into();
            let bhsv: Hsv = b.into();
            let ahue = ahsv.hue.to_positive_radians();
            let bhue = bhsv.hue.to_positive_radians();
            ahue.partial_cmp(&bhue).unwrap()
        });

        let len = cols_rgb.len();
        Palette {
            colors: cols_rgb,
            len,
        }
    }
}
