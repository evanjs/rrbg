fn default_wallpaper_path() -> String {
    String::from("/mnt/gentoo/usr/share/wallpapers/custom/**/*.jpg")
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_wallpaper_path")]
    /// The path to the image file
    pub wallpaper_path: String,
}
