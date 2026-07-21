use anyhow::{Context, Result};
use image;
use std::path::Path;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Image {
    pub fn load(path: &Path) -> Result<Self> {
        let rgba = image::open(path)
            .with_context(|| format!("Failed to open image: {}", path.display()))?
            .into_rgba8();

        Ok(Self {
            width: rgba.width(),
            height: rgba.height(),
            data: rgba.into_raw(),
        })
    }

    pub fn load_pair(path1: &Path, path2: &Path) -> Result<(Self, Self)> {
        let (img1, img2) = rayon::join(|| Self::load(path1), || Self::load(path2));
        Ok((img1?, img2?))
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        image::save_buffer(
            path,
            &self.data,
            self.width,
            self.height,
            image::ExtendedColorType::Rgba8,
        )
        .with_context(|| format!("failed to write {}", path.display()))
    }

    #[inline]
    pub fn pixel(&self, x: u32, y: u32) -> &[u8; 4] {
        let idx = (y as usize * self.width as usize + x as usize) * 4;
        self.data[idx..idx + 4].try_into().unwrap()
    }

    #[inline]
    pub fn same_dimensions(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
