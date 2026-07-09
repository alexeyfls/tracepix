use std::process::ExitCode;

use anyhow::bail;
use clap::Parser;
use cli_arguments::TracepixCliArguments;

use crate::image_io::Image;

mod cli_arguments;
mod image_io;

pub fn run() -> anyhow::Result<ExitCode> {
    match TracepixCliArguments::try_parse() {
        Ok(args) => process(args),
        Err(_) => Ok(ExitCode::FAILURE),
    }
}

fn process(args: TracepixCliArguments) -> anyhow::Result<ExitCode> {
    let (reference, target) = Image::load_pair(&args.reference_img, &args.target_img)?;

    if !reference.same_dimensions(&target) {
        bail!(
            "Image dimensions differ {}x{} vs {}x{}",
            reference.width,
            reference.height,
            target.width,
            target.height
        );
    }

    Ok(ExitCode::SUCCESS)
}
