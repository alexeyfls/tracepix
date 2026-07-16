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

#[cfg(test)]
mod tests {
    use super::*;

    const WHITE: [u8; 4] = [255, 255, 255, 255];
    const GRAY: [u8; 4] = [128, 128, 128, 255];
    const BLACK: [u8; 4] = [0, 0, 0, 255];

    fn image(width: u32, height: u32, pixels: &[[u8; 4]]) -> Image {
        assert_eq!(pixels.len(), (width * height) as usize);

        let mut data = Vec::<u8>::with_capacity(pixels.len() * 4);
        for px in pixels {
            data.extend_from_slice(px);
        }

        Image {
            width,
            height,
            data,
        }
    }

    fn solid(width: u32, height: u32, px: [u8; 4]) -> Image {
        image(width, height, &vec![px; (width * height) as usize])
    }

    fn count(reference: &Image, target: &Image, detect_antialiasing: bool) -> usize {
        compare_images(
            reference,
            target,
            &CompareOptions {
                threshold: 0.0,
                detect_antialiasing,
                emit_diff_image: false,
            },
        )
        .diff_count
    }

    #[test]
    fn identical_images_have_no_diff() {
        let a = solid(4, 4, GRAY);
        let b = solid(4, 4, GRAY);

        assert_eq!(count(&a, &b, false), 0);
    }

    #[test]
    fn single_changed_pixel_counts_once() {
        let a = solid(4, 4, WHITE);
        let mut b = solid(4, 4, WHITE);
        b.data[0..4].copy_from_slice(&BLACK);

        assert_eq!(count(&a, &b, false), 1);
    }

    #[test]
    fn same_luma_different_color_is_flagged() {
        // Pure red and a gray of near-equal brightness: a luma-only metric would
        // miss this, but the YIQ chroma terms catch it.
        let red = [255, 0, 0, 255];
        let gray76 = [76, 76, 76, 255];

        let a = solid(4, 4, red);
        let mut b = solid(4, 4, red);
        b.data[0..4].copy_from_slice(&gray76);

        assert_eq!(count(&a, &b, false), 1);
    }

    #[test]
    fn antialiased_pixel_is_excused() {
        // A vertical white|gray|black edge. The gray transition pixel on the
        // middle row shifts slightly: without AA detection it's a real diff,
        // with it the 3×3 neighborhood recognizes anti-aliasing.
        let rows = 5;
        let mut ref_px = Vec::new();
        for _ in 0..rows {
            ref_px.extend_from_slice(&[WHITE, GRAY, BLACK]);
        }

        let reference = image(3, rows, &ref_px);
        let mut target = image(3, rows, &ref_px);

        let idx = (2 * 3 + 1) * 4;
        target.data[idx..idx + 4].copy_from_slice(&[120, 120, 120, 255]);

        assert_eq!(count(&reference, &target, false), 1);
        assert_eq!(count(&reference, &target, true), 0);
    }
}
