#[macro_use]
extern crate log;

use std::ptr;

use rand::prelude::*;
use rayon::prelude::*;
use x11::xlib;

use rrbg::*;
use rrbg::config::Config;
use rrbg::Wallpaper;

fn main() {
    drop(dotenv::dotenv());
    let wallpaper_path: String;

    match envy::from_env::<Config>() {
        Ok(config) => {
            wallpaper_path = config.wallpaper_path;
        }
        Err(e) => {
            panic!("{:#?}", e);
        }
    }

    let papers = get_walls(wallpaper_path);

    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("XOpenDisplay failed");
        }

        // get the number of screens attached
        let screen_count = xlib::XScreenCount(display);
        let screen_resolutions = get_resolutions(display, screen_count);

        screen_resolutions.iter().for_each(|resolution| {
            let mut rng = thread_rng();
            let selected = papers.lock().unwrap();
            let choice = selected.choose(&mut rng).unwrap();
            set_wallpaper(display, &choice.path).expect("failed to set wallpaper");
        });
    }
}
