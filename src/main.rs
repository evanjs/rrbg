extern crate log;

use rand::prelude::*;

use rrbg::config::Config;
use rrbg::*;

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

    let display = get_display();
    let screen_resolutions = get_resolutions(display);

    let mut walls = Vec::new();

    screen_resolutions.iter().for_each(|resolution| {
        let mut rng = thread_rng();
        let filtered = papers.iter().filter(|&item| item.resolution.eq(resolution));
        let choice = filtered
            .choose(&mut rng)
            .expect("failed to select random wallpaper");
        &walls.push(choice.path.to_owned());
        drop(rng);
    });

    drop(set_wallpaper(walls.to_owned()));
}
