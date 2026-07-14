use crate::antialiasing::is_antialiased;
use crate::color::color_delta;
use crate::image_io::Image;

/// Largest possible value of the YIQ delta metric — used to normalize the
/// threshold so `threshold` reads as a 0.0..=1.0 fraction.
const MAX_YIQ_DELTA: f32 = 35215.0;

/// Color painted over pixels that genuinely differ.
const DIFF_COLOR: [u8; 4] = [255, 0, 0, 255];

/// Painted over pixels excused as anti-aliasing (not counted as diffs).
const AA_COLOR: [u8; 4] = [255, 255, 0, 255];

pub struct CompareOptions {
    pub threshold: f32,
    pub detect_antialiasing: bool,
    pub emit_diff_image: bool,
}

pub struct CompareResult {
    pub diff_count: usize,
    pub diff_percentage: f32,
    pub diff_image: Option<Image>,
}

pub fn compare_images(
    reference: &Image,
    target: &Image,
    options: &CompareOptions,
) -> CompareResult {
    let max_delta = MAX_YIQ_DELTA * options.threshold * options.threshold;
    let (width, height) = (reference.width as usize, reference.height as usize);
    let pixel_count = width * height;

    let mut diff_count = 0usize;
    let mut diff_data = options
        .emit_diff_image
        .then(|| vec![0u8; reference.data.len()]);

    let ref_px = reference.data.as_chunks::<4>().0;
    let target_px = target.data.as_chunks::<4>().0;
    let pairs = ref_px.iter().zip(target_px.iter());

    for (idx, (a, b)) in pairs.enumerate() {
        let mut is_diff = color_delta(a, b).abs() > max_delta;
        let mut is_aa = false;

        if is_diff && options.detect_antialiasing {
            let x = (idx % width) as u32;
            let y = (idx / width) as u32;

            if is_antialiased(reference, target, x, y) || is_antialiased(target, reference, x, y) {
                is_diff = false;
                is_aa = true;
            }
        }

        diff_count += is_diff as usize;

        if let Some(buffer) = diff_data.as_mut() {
            let output = &mut buffer[idx * 4..idx * 4 + 4];
            if is_diff {
                output.copy_from_slice(&DIFF_COLOR);
            } else if is_aa {
                output.copy_from_slice(&AA_COLOR);
            } else {
                let gray = grayscale(a);
                output.copy_from_slice(&[gray, gray, gray, 255]);
            }
        }
    }

    let diff_percentage = if diff_count == 0 {
        0.0
    } else {
        diff_count as f32 / pixel_count as f32 * 100.0
    };
    let diff_image = diff_data.map(|data| Image {
        width: reference.width,
        height: reference.height,
        data,
    });

    CompareResult {
        diff_count,
        diff_percentage,
        diff_image,
    }
}

/// Lightened luma of a pixel, for the dimmed diff-image background.
#[inline]
fn grayscale(px: &[u8; 4]) -> u8 {
    let luma = 0.299 * px[0] as f32 + 0.587 * px[1] as f32 + 0.114 * px[2] as f32;
    (255.0 - (255.0 - luma) * 0.1) as u8
}
