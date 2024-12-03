mod colors;
mod track;

use colors::*;
use track::*;

use clap::Parser;

use std::{env, error::Error, fs, time::Duration};

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct Args {
    /// Choose audio file
    #[arg(short, long)]
    track: String,

    /// Choose audio from current directory
    #[arg(short, long)]
    current: bool,

    /// Choose track from playlist
    #[arg(short, long)]
    playlist: bool,

    /// 1/2 design
    #[arg(short, long)]
    design: bool,

    /// Save track to playlist
    #[arg(short, long)]
    save: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    #[allow(deprecated)]
    let home_dir_path = match env::home_dir() {
        Some(path) => path.to_string_lossy().into_owned(),
        None => "/".to_string(),
    };
    let config = format!("{}/.config/climp/", home_dir_path);

    let args = Args::parse();
    let current_dir = env::current_dir().expect("Failed to get current dir ...");

    let get_current_track = format!("{}/{}", current_dir.display(), args.track);
    let get_playlist_track = format!("{}/.config/climp/{}", home_dir_path, args.track);

    if args.save {
        match fs::metadata(&config) {
            Ok(_) => {
                println!("{GREEN}Directory exist...{RESET} \n");

                match save_track(&get_current_track, &config) {
                    Ok(_) => println!("{GREEN} -- Saved! {RESET}"),
                    Err(_) => println!("{RED} Failed to save track ... {RESET}"),
                }
            }
            Err(_) => {
                println!("{RED}Directory does not exist{RESET} ...\n");

                println!("Creating directory ~/.config/{GREEN}climp{RESET}/ ... ");

                match fs::create_dir(config) {
                    Ok(_) => println!("-- {GREEN}Succes!{RESET}\n"),
                    Err(e) => println!("-- {RED}Failed to create directory ... {} {RESET}", e),
                }
            }
        }
    }

    if args.current {
        let song_duration = match get_song_duration(&get_current_track) {
            Ok(duration) => duration,
            Err(e) => {
                println!("{}", e);
                Duration::from_secs(0)
            }
        };

        get_metadata(&get_current_track, args.design, song_duration);
        play(&get_current_track);
    }

    if args.playlist {
        let song_duration = match get_song_duration(&get_playlist_track) {
            Ok(duration) => duration,
            Err(e) => {
                println!("{}", e);
                Duration::from_secs(0)
            }
        };

        get_metadata(&get_playlist_track, args.design, song_duration);
        play(&get_playlist_track);
    }

    Ok(())
}
