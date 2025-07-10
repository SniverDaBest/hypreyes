use egui::Color32;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::Read, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub eye_color: (u8, u8, u8),
    pub pupil_color: (u8, u8, u8),
    pub background_color: (u8, u8, u8),
    pub transparent_background: bool,
}

impl Config {
    pub fn color32ify_eye_color(&self) -> Color32 {
        return Color32::from_rgb(self.eye_color.0, self.eye_color.1, self.eye_color.2);
    }

    pub fn color32ify_pupil_color(&self) -> Color32 {
        return Color32::from_rgb(self.pupil_color.0, self.pupil_color.1, self.pupil_color.2);
    }

    pub fn color32ify_background_color(&self) -> Color32 {
        return Color32::from_rgb(self.background_color.0, self.background_color.1, self.background_color.2);
    }
}

impl Default for Config {
    fn default() -> Self {
        return Self {
            eye_color: (255, 255, 255),
            pupil_color: (0, 0, 0),
            background_color: (0, 0, 0),
            transparent_background: false,
        };
    }
}

pub fn get_config() -> Config {
    let mut buf = String::new();

    let config_path_str = format!(
        "{}/hypreyes/hypreyes.ron",
        env::var("XDG_CONFIG_HOME").expect("Unable to get XDG_CONFIG_HOME env variable!")
    );
    let config_path = Path::new(&config_path_str);
    let mut config_file = match File::open(config_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Unable to open the config file! Error: {}", e);
            return Config::default();
        }
    };

    match config_file.read_to_string(&mut buf) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Unable to read the config file! Error: {}", e);
            return Config::default();
        }
    }

    match ron::from_str::<Config>(buf.as_str()) {
        Ok(c) => return c,
        Err(e) => {
            eprintln!(
                "Unable to convert config file data to config struct! Error: {}",
                e
            );
            return Config::default();
        }
    }
}
