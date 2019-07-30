extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use failure::*;
use glob::glob;
use imagesize::size;
use std::error::Error;
use std::path::*;
use std::process::{Command, ExitStatus};
use std::path::*;
use std::ptr;
use failure::*;

use dirs::*;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use serde::ser::Serialize;
use std::io::BufReader;

pub mod config;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A wallpaper entry
pub struct Wallpaper {
    /// The path to the wallpaper image
    pub path: String,
    pub resolution: Resolution,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
/// The resolution of the image
pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

pub struct Screen {
    pub resolution: Resolution,
    pub screen_number: u8,
}

/// Return wallpapers (images) found under the specified directory
/// Note: This function accepts glob strings, e.g. "**/*.jpg"
pub fn get_walls1(path: String) -> Vec<Wallpaper> {
    let mut papers = Vec::new();
    let entries: Vec<_> = glob(&path)
        .unwrap()
        .map(|entry| {
            entry
                .expect("failed to get glob entry")
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect();

    entries.iter().for_each(|entry| {
        let resolution = get_image_resolution(entry.to_owned());
        papers.push(Wallpaper {
            path: entry.to_owned(),
            resolution,
        });
    });

    papers
}

fn get_cache() -> Vec<Wallpaper>  {
    // create path
    let cache = std::path::Path::join(&dirs::config_dir().expect("Failed to get config directory"), "rrbg").join("cache");
    if !cache.parent().expect("Failed to get parent directory of cache file").exists() {
        info!("No config directory found.  Creating ...");
        std::fs::create_dir_all(cache.parent().expect("Failed to get parent directory of cache file")).expect("Failed to create config directory");
    }
    if !cache.exists() {
        return Vec::new();
    }
    // open file
    let mut file = File::open(cache).expect("Failed to open cache file");
    // get contents
    let mut contents = String::new();
    if file.metadata().unwrap().len() == 0 {
        return Vec::new()
    }
    file.read_to_string(&mut contents).expect("Failed to read cache file");
    serde_json::from_str(&contents).expect("Failed to deserialize cache")
}

pub fn write_cache(wallpapers: Vec<Wallpaper>) {
    // todo: see if cache path can be made into a constant function
    // create path
    let cache = std::path::Path::join(&dirs::config_dir().unwrap(), "rrbg").join("cache");
    // open file
    let mut file = File::create(cache).expect("Failed to open cache file");

    // Serialize data
    serde_json::to_writer_pretty(file, &wallpapers).unwrap();
    info!("Wrote stuff");
}

pub fn get_walls(path: String) -> Vec<Wallpaper> {
    let mut config = get_cache();
    let mut walls = Vec::new();
    match config.len() {
        0 => {
            &walls.append(&mut get_walls1(path));
        },
        _ => {
            &walls.append(&mut config);
        }
    };
    walls
}

/// Get the width and height of an image
/// This function does not fully load images, but utilizes the imagesize crate
/// to quickly determine image sizes
pub fn get_image_resolution(path: String) -> Resolution {
    let size = size(path);
    let mut width = 0;
    let mut height = 0;
    match size {
        Ok(o) => {
            width = o.width as i32;
            height = o.height as i32;
        }
        Err(why) => error!("Failed to get image resolution - {:#?}", why),
    };

    Resolution { width, height }
}

/// Get the width and height for each attached screen
pub fn get_resolutions() -> Vec<Resolution> {
    let sdl_context = sdl2::init().unwrap();
    let mut resolutions = Vec::new();

    let video_subsystem = sdl_context
        .video()
        .expect("Failed to get video subsystem from sdl context");

    let num_displays = video_subsystem
        .num_video_displays()
        .expect("Failed to get number of displays");
    for n in 0..num_displays {
        let mode = video_subsystem
            .current_display_mode(n)
            .expect("Failed to get current display mode");
        &resolutions.push(Resolution {
            width: mode.w,
            height: mode.h,
        });
    }

    resolutions
}

/// Sets the wallpaper for the given display
/// Currently, this calls feh internally, but
/// it would be interesting to see how a pure libc binding
/// or similar implementation could be done
pub fn set_wallpaper(path: Vec<String>) -> Result<ExitStatus, std::io::Error> {
    Command::new("feh")
        .args(&["--bg-center"])
        .args(path)
        .spawn()
        .expect("failed to call feh")
        .wait()
}
