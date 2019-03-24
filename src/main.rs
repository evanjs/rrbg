use rand::prelude::*;

use rrbg::config::Config;
use rrbg::*;
use log::*;

use clap::{App, load_yaml,value_t};

fn main() {
    drop(dotenv::dotenv());
    let yml = load_yaml!("clap.yaml");
    let app = App::from_yaml(yml);
    let matches = app.get_matches();



    let wallpaper_path: String;

    match envy::from_env::<Config>() {
        Ok(config) => {
            wallpaper_path = config.wallpaper_path;
        }
        Err(e) => {
            panic!("{:#?}", e);
        }
    }
    debug!("{:?}", &wallpaper_path);
    let update = value_t!(matches, "update", bool).unwrap_or(false);
    debug!("{:?}", &update);

    let papers = get_walls(wallpaper_path);



    if update {
        trace!("Updating cache");
        write_cache(papers.clone());

    }

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
