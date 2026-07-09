use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tracepix", version, about, long_about = None)]
pub struct TracepixCliArguments {
    pub reference_img: PathBuf,

    pub target_img: PathBuf,

    #[arg(short, long)]
    pub output_path: Option<PathBuf>,

    #[arg(short, long)]
    pub threshold: f32,

    #[arg(short, long)]
    pub antialiasing: bool,
}
