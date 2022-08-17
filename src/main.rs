use cute_songs::{Config, Stats};
use serde_json::{Result, Value};
use std::error::Error;
use std::process::Command;
use std::{fs, path::Path};

fn run(config: Config) -> std::result::Result<(), Box<dyn Error>> {
    // try to get json using yt-dlp
    let raw_playlist_json = Command::new("yt-dlp")
        .arg("--skip-download")
        .arg("--dump-single-json")
        .arg("--flat-playlist")
        .arg(config.playlist_url)
        .output()?;

    if raw_playlist_json.status.success() {
        // create an object out of the json
        let pl_info: Value = serde_json::from_slice(&raw_playlist_json.stdout)?;

        // ensure that songs directory exists
        fs::create_dir_all(&config.song_directory)?;

        // create a vector of all videos
        let local_songs = fs::read_dir(&config.song_directory)?
            .filter_map(|f| f.ok())
            .map(|f| f.file_name().into_string())
            .filter_map(|n| n.ok())
            .collect::<Vec<String>>();

        let mut stats = Stats::default();

        for video in pl_info
            .get("entries")
            .unwrap()
            .as_array()
            .expect("invalid json")
        {
            let id = video.get("id").unwrap().as_str().unwrap();
            let title = video.get("title").unwrap().as_str().unwrap();

            if !local_songs
                .iter()
                .any(|n| n.contains(id) || n.contains(title))
            {
                // download the song to the directory, making sure to include the title in the song name
                println!("downloading `{}`", title);

                let mut d = Command::new("yt-dlp")
                    .arg("-P")
                    .arg(&config.song_directory)
                    .arg("-o")
                    .arg("%(title)s-%(id)s.%(ext)s")
                    .arg("-f")
                    .arg("bestaudio")
                    // fixes bug where ids with hyphens in them are ignored
                    .arg("--")
                    .arg(id)
                    .spawn()
                    .unwrap();

                match d.wait().unwrap().code().unwrap() {
                    0 => {
                        stats.downloaded += 1;
                    }
                    _ => {
                        stats.failed += 1;
                    }
                }

            } else {
                stats.skipped += 1;

                // should install a logging crate, since we don't always want to flood the user with messages
                println!("skipped downloading `{}`", title);
            }
        }

        println!(
            "success. processed {} songs in total. skipped {}, downloaded {}, failed {}",
            stats.downloaded + stats.skipped + stats.failed,
            stats.skipped,
            stats.downloaded,
            stats.failed
        );
    } else {
        eprintln!("error occured with yt-dlp:");
        eprintln!(
            "{}",
            std::str::from_utf8(&raw_playlist_json.stderr)
                .expect("error unwrapping yt-dlp message")
        );
    }
    Ok(())
}

fn main() {
    let config = Config::new(std::env::args());
    if let Err(e) = run(config) {
        eprintln!("{}", e);
    }
}
