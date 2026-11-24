//! 编译期内嵌的资源索引。封面、图标、字体和音效都随 exe 打包。

use anyhow::{Context, Result};
use include_dir::{Dir, include_dir};

use crate::config::COVER_PLACEHOLDER;

static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub struct AssetService {
    cards: Vec<(&'static str, &'static [u8])>,
}

impl AssetService {
    pub fn new() -> Self {
        // 预先排序，保证随机封面的候选顺序稳定。
        Self {
            cards: collect_entries("images/card", ".jpg"),
        }
    }

    pub fn card(&self, file_name: &str) -> Option<&'static [u8]> {
        self.cards
            .iter()
            .find_map(|&(name, bytes)| (name == file_name).then_some(bytes))
    }

    pub fn sd(&self, file_name: &str) -> Option<&'static [u8]> {
        let path = format!("images/sd/{file_name}");
        ASSETS.get_file(path).map(|file| file.contents())
    }

    pub fn sd_images_excluding(&self, excluded: &str) -> Vec<(&'static str, &'static [u8])> {
        collect_entries("images/sd", ".png")
            .into_iter()
            .filter(|&(name, _)| name != excluded)
            .collect()
    }

    pub fn random_card_excluding(
        &self,
        current: Option<&str>,
    ) -> Option<(&'static str, &'static [u8])> {
        // 排除占位封面和当前封面，避免连续两次抽到同一张。
        let candidates: Vec<(&'static str, &'static [u8])> = self
            .cards
            .iter()
            .copied()
            .filter(|&(name, _)| name != COVER_PLACEHOLDER && Some(name) != current)
            .collect();

        if candidates.is_empty() {
            return self
                .card(COVER_PLACEHOLDER)
                .map(|bytes| (COVER_PLACEHOLDER, bytes));
        }
        Some(candidates[fastrand::usize(..candidates.len())])
    }

    pub fn logos(&self) -> Vec<&'static [u8]> {
        collect_entries("images/logo", ".jpg")
            .into_iter()
            .map(|(_, bytes)| bytes)
            .collect()
    }

    pub fn sound(&self, name: &str) -> Option<&'static [u8]> {
        let path = format!("sounds/{name}.wav");
        ASSETS.get_file(path).map(|file| file.contents())
    }

    pub fn font(&self) -> &'static [u8] {
        ASSETS
            .get_file("fonts/Lolita.ttf")
            .expect("embedded Lolita.ttf is missing")
            .contents()
    }

    pub fn icon(&self) -> Option<&'static [u8]> {
        ASSETS.get_file("imouto.ico").map(|file| file.contents())
    }

    pub fn load_color_image(&self, bytes: &[u8]) -> Result<eframe::egui::ColorImage> {
        let image = image::load_from_memory(bytes).context("无法解码内嵌图片")?;
        let rgba = image.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        Ok(eframe::egui::ColorImage::from_rgba_unmultiplied(
            size,
            rgba.as_raw(),
        ))
    }
}

fn collect_entries(directory: &str, extension: &str) -> Vec<(&'static str, &'static [u8])> {
    let mut entries: Vec<(&'static str, &'static [u8])> = ASSETS
        .get_dir(directory)
        .map(|dir| {
            dir.files()
                .filter_map(|file| {
                    let path = file.path().to_str()?;
                    if !path.ends_with(extension) {
                        return None;
                    }
                    let name = path.rsplit('/').next()?;
                    Some((name, file.contents()))
                })
                .collect()
        })
        .unwrap_or_default();
    entries.sort_unstable_by_key(|&(name, _)| name);
    entries
}
