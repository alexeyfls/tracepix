/// Luma-only difference between two pixels. Used by anti-aliasing detection,
/// which only cares about brightness. Signed: positive means `a` is brighter.
#[inline]
pub fn brightness_delta(a: &[u8; 4], b: &[u8; 4]) -> f32 {
    let [r1, g1, b1] = blend_to_rgb(a);
    let [r2, g2, b2] = blend_to_rgb(b);

    rgb2y(r1, g1, b1) - rgb2y(r2, g2, b2)
}

/// Signed perceptual delta between two RGBA pixel.
#[inline]
pub fn color_delta(a: &[u8; 4], b: &[u8; 4]) -> f32 {
    if a == b {
        return 0.0;
    }

    let [r1, g1, b1] = blend_to_rgb(a);
    let [r2, g2, b2] = blend_to_rgb(b);

    let y1 = rgb2y(r1, g1, b1);
    let y2 = rgb2y(r2, g2, b2);
    let i1 = rgb2i(r1, g1, b1);
    let i2 = rgb2i(r2, g2, b2);
    let q1 = rgb2q(r1, g1, b1);
    let q2 = rgb2q(r2, g2, b2);

    let y_diff = y1 - y2;
    let i_diff = i1 - i2;
    let q_diff = q1 - q2;

    let delta = 0.5053 * y_diff * y_diff + 0.299 * i_diff * i_diff + 0.1957 * q_diff * q_diff;

    if y1 > y2 { -delta } else { delta }
}

/// Composite an RGBA pixel onto white, yielding straight RGB as f32.
/// Fully-opaque pixels skip the blend entirely.
#[inline]
fn blend_to_rgb(px: &[u8; 4]) -> [f32; 3] {
    if px[3] == 255 {
        [px[0] as f32, px[1] as f32, px[2] as f32]
    } else {
        let alpha = px[3] as f32 / 255.0;
        [
            blend(px[0], alpha),
            blend(px[1], alpha),
            blend(px[2], alpha),
        ]
    }
}

/// Blend one channel of a semi-transparent pixel onto a white background.
/// `alpha` is normalized to 0.0..=1.0.
#[inline]
fn blend(channel: u8, alpha: f32) -> f32 {
    255.0 + (channel as f32 - 255.0) * alpha
}

// YIQ conversion — coefficients from the Kotsarenko–Ramos perceptual metric.

#[inline]
fn rgb2y(r: f32, g: f32, b: f32) -> f32 {
    0.29889531 * r + 0.58662247 * g + 0.11448223 * b
}

#[inline]
fn rgb2i(r: f32, g: f32, b: f32) -> f32 {
    0.59597799 * r - 0.27417610 * g - 0.32180189 * b
}

#[inline]
fn rgb2q(r: f32, g: f32, b: f32) -> f32 {
    0.21147017 * r - 0.52261711 * g + 0.31114694 * b
}
