use crate::colors::*;

use rodio::{Decoder, OutputStream, Sink, Source};
use std::{io::BufReader, process::Command, time::Duration};

pub fn play(path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end();
    sink.play();
}

pub fn get_song_duration(path: &str) -> Result<Duration, String> {
    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    let duration_sec = source.total_duration().unwrap_or(Duration::from_secs(0));
    let duration_min = duration_sec / 60;
    let rounded_duration = round_duration(duration_min, 2);

    Ok(rounded_duration)
}

pub fn round_duration(duration: Duration, decimal_places: u32) -> Duration {
    let duration_secs = duration.as_secs_f64();
    let rounded_secs = (duration_secs * 10_f64.powi(decimal_places as i32)).round()
        / 10_f64.powi(decimal_places as i32);
    Duration::from_secs_f64(rounded_secs)
}

pub fn get_metadata(path: &str, which_design: bool, time: Duration) {
    let data = taglib::File::new(path).unwrap();

    if which_design {
        match data.tag() {
            Ok(tag) => {
                println!(
                    " Title -- {GREEN}{:?} -- {:?}plat mins{RESET}",
                    tag.title().unwrap_or("Unknown".into()),
                    time
                );
                println!(
                    "Artist -- {GREEN}{:?}{RESET}",
                    tag.artist().unwrap_or("Unknown".into())
                );
                println!(
                    " Album -- {GREEN}{:?}{RESET}",
                    tag.album().unwrap_or("{RED}Unknown{RESET}".into())
                );
                println!("  Year -- {GREEN}{:?}{RESET}", tag.year().unwrap_or(0));
            }
            Err(_) => {
                println!("Failed to get metadata");
            }
        }
    } else {
        match data.tag() {
            Ok(tag) => {
                println!(
                    "Title: {:?}|({:?}plat mins)",
                    tag.title().unwrap_or("Unknown".into()),
                    time
                );
                println!("Artist: {:?}", tag.artist().unwrap_or("Unknown".into()));
                println!("Album: {:?}", tag.album().unwrap_or("Unknown".into()));
                println!("Year: {:?}", tag.year().unwrap_or(0));
            }
            Err(_) => {
                println!("Failed to get metadata");
            }
        }
    }
}

pub fn save_track(get_current_track: &str, config: &str) -> Result<(), std::io::Error> {
    let _save_track = Command::new("cp")
        .arg(get_current_track)
        .arg(config)
        .spawn()
        .unwrap();

    Ok(())
}
