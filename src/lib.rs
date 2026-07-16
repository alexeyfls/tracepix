use anyhow::Context;
use clap::Parser;
use image::save_buffer;

use cli_arguments::TracepixCliArguments;
pub use image_io::Image;

pub use crate::compare::{CompareOptions, compare_images};

mod antialiasing;
mod cli_arguments;
mod color;
mod compare;
mod image_io;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TracepixResult {
    Identical,
    Different {
        diff_count: usize,
        percentage: f32,
    },
    LayoutMismatch {
        reference: (u32, u32),
        target: (u32, u32),
    },
}

pub fn run() -> anyhow::Result<TracepixResult> {
    process(TracepixCliArguments::parse())
}

fn process(args: TracepixCliArguments) -> anyhow::Result<TracepixResult> {
    let (reference, target) = Image::load_pair(&args.reference_img, &args.target_img)?;

    if !reference.same_dimensions(&target) {
        return Ok(TracepixResult::LayoutMismatch {
            reference: (reference.width, reference.height),
            target: (target.width, target.height),
        });
    }

    let result = compare_images(
        &reference,
        &target,
        &CompareOptions {
            threshold: args.threshold,
            detect_antialiasing: args.antialiasing,
            emit_diff_image: args.output_path.is_some(),
        },
    );

    if let (Some(path), Some(image)) = (args.output_path, result.diff_image) {
        save_buffer(
            &path,
            &image.data,
            image.width,
            image.height,
            image::ExtendedColorType::Rgba8,
        )
        .with_context(|| format!("Failed to write diff to image to {}", path.display()))?;
    }

    if result.diff_count == 0 {
        Ok(TracepixResult::Identical)
    } else {
        Ok(TracepixResult::Different {
            diff_count: result.diff_count,
            percentage: result.diff_percentage,
        })
    }
}
