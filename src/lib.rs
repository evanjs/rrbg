extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use std::process::ExitStatus;
use std::sync::Arc;
use std::sync::Mutex;

use glob::glob;
use imagesize::size;
use rayon::prelude::*;
use x11::xlib;

pub mod config;

#[derive(Debug)]
/// A wallpaper entry
pub struct Wallpaper {
    /// The path to the wallpaper image
    pub path: String,
    pub resolution: Resolution,
}

#[derive(Debug, Eq, PartialEq)]
/// The resolution of the image
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub struct Screen {
    pub resolution: Resolution,
    pub screen_number: u8,
}

/// Return wallpapers (images) found under the specified directory
/// Note: This function accepts glob strings, e.g. "**/*.jpg"
/// ```rust
/// let wallpapers = get_walls("/mnt/gentoo/usr/share/wallpapers/custom/**/*.jpg");
/// ```
pub fn get_walls(path: String) -> Box<Arc<Mutex<Vec<Wallpaper>>>> {
    let papers = Arc::new(Mutex::new(Vec::<Wallpaper>::new()));
    let mut entries: Vec<_> = glob(&path)
        .unwrap()
        .map(|entry| {
            entry
                .expect("failed to get glob entry")
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect();
    entries.par_iter_mut().for_each(|entry| {
        let resolution = get_image_resolution(entry.to_string());
        papers.lock().unwrap().push(Wallpaper {
            path: entry.to_string(),
            resolution,
        });
    });

    Box::new(papers)
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
            width = o.width as u32;
            height = o.height as u32;
        }
        Err(why) => error!("Failed to get image resolution"),
    };

    Resolution { width, height }
}

/// Get the width and height for each attached screen
pub unsafe fn get_resolutions(display: *mut xlib::Display, screen_count: i32) -> Vec<Resolution> {
    let mut resolutions = Vec::new();
    for x in 0..screen_count {
        let screen = xlib::XScreenOfDisplay(display, x);
        resolutions.push(Resolution {
            width: (*screen).width as u32,
            height: (*screen).height as u32,
        })
    }
    resolutions
}

/// Sets the wallpaper for the given display
/// Currently, this calls feh internally, but
/// it would be interesting to see how a pure libc binding
/// or similar implementation could be done
pub unsafe fn set_wallpaper(
    display: *mut xlib::Display,
    path: &str,
) -> Result<ExitStatus, std::io::Error> {
    Command::new("feh")
        .args(&["--bg-fill", path])
        .spawn()
        .expect("failed to call feh")
        .wait()
}
