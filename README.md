# rrbg - Random Background/Wallpaper Selector
The purpose of this application is to determine the resolution of 
all connected screens, and randomly select a wallpaper for each screen
with a matching resolution



### Note for barebones Linux desktop envrionments (e.g. xmonad)
The default path checks for the XDG picture directory (i.e. $XDG_PICTURES_DIR)
See [this](https://wiki.archlinux.org/index.php/XDG_user_directories) page for more information.

The long term goal is to fail if this directory is not found, check ENV, CLI options, and a config file, etc.
For now, simply initialize the XDG user dirs (macOS and Windows users shoudl already be set) and use ~/Pictures/Wallpapers.

#### NixOS
- Install (or launch a `nix-shell`(?) with) xdg-user-dirs and run `xdg-user-dirs-update`
