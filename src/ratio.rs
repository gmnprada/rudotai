use std::cmp::Ordering;

enum AspectRatio {
    Square,
    WidescreenCinema,
    Cinema,
    Film35mm,
    TraditionalPC,
    Computer,
    HighDefinitionTV,
    Widescreen,
    UltraWideScreen,
}

impl AspectRatio {
    pub fn from_str(s: &str) -> Option<AspectRatio> {
        match s {
            "1:1" => Some(AspectRatio::Square),
            "1.85:1" => Some(AspectRatio::WidescreenCinema),
            "2.35:1" => Some(AspectRatio::Cinema),
            "3:2" => Some(AspectRatio::Film35mm),
            "4:3" => Some(AspectRatio::TraditionalPC),
            "5:4" => Some(AspectRatio::Computer),
            "16:9" => Some(AspectRatio::HighDefinitionTV),
            "16:10" => Some(AspectRatio::Widescreen),
            "21:9" => Some(AspectRatio::UltraWideScreen),
            _ => None,
        }
    }

    pub fn from_resolution(width: u32, height: u32) -> Option<AspectRatio> {
        let ratio = width as f32 / height as f32;
        match ratio.partial_cmp(&0.0) {
            Some(Ordering::Equal) => Some(AspectRatio::Square),
            Some(Ordering::Greater) => match ratio {
                r if r < 1.33 => Some(AspectRatio::TraditionalPC),
                r if r < 1.6 => Some(AspectRatio::Film35mm),
                r if r < 1.77 => Some(AspectRatio::HighDefinitionTV),
                r if r < 1.85 => Some(AspectRatio::Widescreen),
                r if r < 2.35 => Some(AspectRatio::WidescreenCinema),
                _ => Some(AspectRatio::Cinema),
            },
            Some(Ordering::Less) => match ratio {
                r if r > 0.75 => Some(AspectRatio::Computer),
                _ => None,
            },
            None => None,
        }
    }
}


