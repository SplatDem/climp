use clap::Parser;
use rodio::{Decoder, OutputStream, Sink, Source};

use std::{env, error::Error, io::BufReader, time::Duration};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    audio: String,

    #[arg(short, long, default_value_t = 0)]
    design: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let current_dir = env::current_dir().expect("Failed to get current dir ...");
    let get_current_dir = format!("{}/{}", current_dir.display(), args.audio);

    let song_duration = match get_song_duration(&get_current_dir) {
        Ok(duration) => duration,
        Err(e) => {
            println!("{}", e);
            Duration::from_secs(0)
        }
    };

    get_metadata(&get_current_dir, args.design, song_duration);
    play(&get_current_dir);

    Ok(())
}

fn play(path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end();
    sink.play();
}

fn get_song_duration(path: &str) -> Result<Duration, String> {
    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    let duration_sec = source.total_duration().unwrap_or(Duration::from_secs(0));
    let duration_min = duration_sec / 60;
    let rounded_duration = round_duration(duration_min, 2);

    Ok(rounded_duration)
}

fn round_duration(duration: Duration, decimal_places: u32) -> Duration {
    let duration_secs = duration.as_secs_f64();
    let rounded_secs = (duration_secs * 10_f64.powi(decimal_places as i32)).round()
        / 10_f64.powi(decimal_places as i32);
    Duration::from_secs_f64(rounded_secs)
}

fn get_metadata(path: &str, which_design: u8, time: Duration) {
    let data = taglib::File::new(path).unwrap();

    if which_design == 1 {
        match data.tag() {
            Ok(tag) => {
                println!(
                    " Title -- {:?} -- {:?}plat mins",
                    tag.title().unwrap_or("Unknown".into()),
                    time
                );
                println!("Artist -- {:?}", tag.artist().unwrap_or("Unknown".into()));
                println!(" Album -- {:?}", tag.album().unwrap_or("Unknown".into()));
                println!("  Year -- {:?}", tag.year().unwrap_or(0));
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
