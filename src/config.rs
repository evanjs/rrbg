use std::path::{PathBuf, Path};
use std::string::ToString;
use directories::UserDirs;

fn default_wallpaper_directory() -> PathBuf {
    if let Some(user_dirs) = UserDirs::new() {
        let picture_directory = user_dirs.picture_dir().expect("Failed to find picture directory");
        debug!("Found picture directory at {:?}", picture_directory);
        let path = Path::join(picture_directory, "Wallpapers");
        debug!("Set default wallpaper path to {:?}", path);
        path
    } else {
        todo!("Check for user configured path here");
    }
}

fn default_wallpaper_path() -> String {
    trace!("Trying to determine default wallpaper directory ...");
    let wallpaper_directory = default_wallpaper_directory();
    debug!("Got default wallpaper directory: {:?}", wallpaper_directory);
    let wallpaper_path = wallpaper_directory.to_owned();
    let wallpaper_string = wallpaper_path.to_str().expect("Failed to get path to wallpaper directory");
    let final_string = format!("{}{}", wallpaper_string, "/**/*.jpg");
    debug!("Default wallpaper directory is {:?}", final_string);
    final_string.to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_wallpaper_path")]
    /// The path to the image file
    pub wallpaper_path: String,
}
