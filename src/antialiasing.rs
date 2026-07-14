use crate::color::brightness_delta;
use crate::image_io::Image;

/// Does the pixel at (x, y) look like anti-aliasing rather than a real change?
/// Inspects the 3×3 neighborhood; requires the edge structure to exist in
/// *both* images so it can't mask a genuine difference.
pub fn is_antialiased(image: &Image, other: &Image, x: u32, y: u32) -> bool {
    let (width, height) = (image.width, image.height);

    let x0 = x.saturating_sub(1);
    let y0 = y.saturating_sub(1);
    let x2 = (x + 1).min(width - 1);
    let y2 = (y + 1).min(height - 1);

    let mut min = 0.0f32;
    let mut max = 0.0f32;
    let mut min_pos = (0u32, 0u32);
    let mut max_pos = (0u32, 0u32);

    let mut zeroes = usize::from(x == x0 || x == x2 || y == y0 || y == y2);

    let center = image.pixel(x, y);

    for nx in x0..=x2 {
        for ny in y0..=y2 {
            if nx == x && ny == y {
                continue;
            }

            let delta = brightness_delta(center, image.pixel(nx, ny));
            if delta == 0.0 {
                zeroes += 1;
                if zeroes > 2 {
                    return false;
                }
            } else if delta < min {
                min = delta;
                min_pos = (nx, ny);
            } else if delta > max {
                max = delta;
                max_pos = (nx, ny);
            }
        }
    }

    if min == 0.0 || max == 0.0 {
        return false;
    }

    (has_many_siblings(image, min_pos.0, min_pos.1)
        && has_many_siblings(other, min_pos.0, min_pos.1))
        || (has_many_siblings(image, max_pos.0, max_pos.1)
            && has_many_siblings(other, max_pos.0, max_pos.1))
}

fn has_many_siblings(image: &Image, x: u32, y: u32) -> bool {
    let (width, height) = (image.width, image.height);

    let x0 = x.saturating_sub(1);
    let y0 = y.saturating_sub(1);
    let x2 = (x + 1).min(width - 1);
    let y2 = (y + 1).min(height - 1);

    let center = image.pixel(x, y);
    let mut zeroes = usize::from(x == x0 || x == x2 || y == y0 || y == y2);

    for nx in x0..=x2 {
        for ny in y0..=y2 {
            if nx == x && ny == y {
                continue;
            }

            if center == image.pixel(nx, ny) {
                zeroes += 1;
                if zeroes > 2 {
                    return true;
                }
            }
        }
    }

    false
}
