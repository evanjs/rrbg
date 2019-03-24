extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use glob::glob;
use imagesize::size;
use std::process::{Command, ExitStatus};
use std::ptr;
use x11::{xlib, xrandr};

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
/// use rrbg::get_walls;
/// let wallpapers = get_walls(String::from("/mnt/gentoo/usr/share/wallpapers/custom/**/*.jpg"));
/// ```
pub fn get_walls(path: String) -> Vec<Wallpaper> {
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
        Err(why) => error!("Failed to get image resolution - {:#?}", why),
    };

    Resolution { width, height }
}

pub fn get_display() -> *mut xlib::Display {
    unsafe { xlib::XOpenDisplay(ptr::null()) }
}

/// Get the width and height for each attached screen
pub fn get_resolutions(display: *mut xlib::Display) -> Vec<Resolution> {
    let default_root_window = unsafe { xlib::XDefaultRootWindow(display) };
    let screens = unsafe { xrandr::XRRGetScreenResources(display, default_root_window) };

    let mut resolutions = Vec::new();

    for i in 0..unsafe { *screens }.ncrtc as usize {
        unsafe {
            let info = xrandr::XRRGetCrtcInfo(display, screens, *(*screens).crtcs.add(i));
            match ((*info).height, (*info).width) {
                (0, 0) => (),
                _ => resolutions.push(Resolution {
                    width: (*info).width,
                    height: (*info).height,
                }),
            }
            xrandr::XRRFreeCrtcInfo(info);
        }
    }
    unsafe {
        xrandr::XRRFreeScreenResources(screens);
        xlib::XCloseDisplay(display);
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
