use std::path::{PathBuf, Path};
use std::string::ToString;
use directories::UserDirs;

fn default_wallpaper_directory() -> PathBuf {
    if let Some(user_dirs) = UserDirs::new() {
        Path::join(user_dirs.picture_dir().expect("Failed to find picture directory"), "Wallpapers")
    } else {
        todo!("Check for user configured path here");
    }
}

fn default_wallpaper_path() -> String {
    let wallpaper_directory = default_wallpaper_directory();
    let wallpaper_path = wallpaper_directory.to_owned();
    wallpaper_path.join("/**/*.jpg").to_string_lossy().to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_wallpaper_path")]
    /// The path to the image file
    pub wallpaper_path: String,
}
