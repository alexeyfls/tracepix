use std::path::Path;

pub use image_io::Image;

pub use crate::compare::{CompareOptions, compare_images};

mod antialiasing;
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

pub struct Comparison {
    pub result: TracepixResult,
    pub diff_image: Option<Image>,
}

pub fn compare(
    reference: &Path,
    target: &Path,
    options: &CompareOptions,
) -> anyhow::Result<Comparison> {
    let (reference, target) = Image::load_pair(reference, target)?;

    if !reference.same_dimensions(&target) {
        return Ok(Comparison {
            result: TracepixResult::LayoutMismatch {
                reference: (reference.width, reference.height),
                target: (target.width, target.height),
            },
            diff_image: None,
        });
    }

    let outcome = compare_images(&reference, &target, options);

    let result = if outcome.diff_count == 0 {
        TracepixResult::Identical
    } else {
        TracepixResult::Different {
            diff_count: outcome.diff_count,
            percentage: outcome.diff_percentage,
        }
    };

    Ok(Comparison {
        result,
        diff_image: outcome.diff_image,
    })
}
