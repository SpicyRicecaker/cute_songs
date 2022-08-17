//! This crate attempts to download *only songs that have not been downloaded* from a youtube playlist by matching the title and id of local files to their youtube titles and ids. 
//! 
//! ## Requirements
//! This app requires the installation of [yt-dlp](https://github.com/yt-dlp/yt-dlp) to download videos.
//! 
//! ## Example Usage
//! ```shell
//! # create a directory to store songs, or alternatively you can skip this line and use an existing directory
//! mkdir songs
//! cute_songs https://youtube.com/playlist?list=PLBML8SXyfQ6fJ_GJfCh9c2E6lIdYy3LIl --song-directory songs
//! ```
//! 
//! ## CLI options
//! - **required** `--song-directory [song directory]` describes where the current songs will be sourced *from* and downloaded *to*
//! - **required** `[playlist url]`

pub struct Config {
  pub song_directory: String,
  pub playlist_url: String
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Self {
    args.next();

    // a hashset seems to be the best way to do this
    let mut song_directory = None;
    let mut playlist_url = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--song-directory" => {
                if song_directory.is_some() {
                    panic!("already set `--song-directory`")
                }
                if let Some(arg) = args.next() {
                    song_directory = Some(arg);
                } else {
                    panic!("no song listed after --song-directory")
                }
            }
            "--help" => {
                println!(r##"--song-directory [song] // provide a song directory
                --help // print this message
                [url] // provide a url to a playlist
                "##);
                std::process::exit(0);
            }
            _ => {
                if playlist_url.is_some() {
                    panic!("unexpected argument {}", arg)
                }
                playlist_url = Some(arg);
            }
        }
    }

    if song_directory.is_none() {
        panic!("--song-directory [song] not provided");
    }

    if playlist_url.is_none() {
        panic!("playlist_url not provided");
    }

    if let Some(song_directory) = song_directory && let Some(playlist_url) = playlist_url {
      Self {
        song_directory,
        playlist_url
      }
    } else {
        unreachable!();
    }
  }
}
