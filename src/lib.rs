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

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    // #[arg(short, long)]
    pub playlist_url: String,
    #[arg(short, long)]
    pub song_directory: String,
}


#[derive(Default)]
pub struct Stats {
    pub downloaded: u32, 
    pub skipped: u32,
    pub failed: u32
}