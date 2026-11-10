//! 编译期内嵌的资源索引。封面、图标、字体和音效都随 exe 打包。

use anyhow::{Context, Result};
use include_dir::{Dir, include_dir};

static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub struct AssetService {
    cards: Vec<&'static str>,
    logos: Vec<&'static str>,
}

impl AssetService {
    pub fn new() -> Self {
        // 预先排序，保证随机封面和启动动画的顺序稳定。
        let mut cards = collect_files("images/card", ".jpg");
        let mut logos = collect_files("images/logo", ".jpg");
        cards.sort_unstable();
        logos.sort_unstable();
        Self { cards, logos }
    }

    pub fn card(&self, file_name: &str) -> Option<&'static [u8]> {
        let path = format!("images/card/{file_name}");
        ASSETS.get_file(path).map(|file| file.contents())
    }

    pub fn sd(&self, file_name: &str) -> Option<&'static [u8]> {
        let path = format!("images/sd/{file_name}");
        ASSETS.get_file(path).map(|file| file.contents())
    }

    pub fn sd_images_excluding(&self, excluded: &str) -> Vec<(&'static str, &'static [u8])> {
        let mut files = collect_files("images/sd", ".png");
        files.sort_unstable();
        files
            .into_iter()
            .filter(|path| path.rsplit('/').next() != Some(excluded))
            .filter_map(|path| {
                let name = path.rsplit('/').next()?;
                ASSETS.get_file(path).map(|file| (name, file.contents()))
            })
            .collect()
    }

    pub fn random_card_excluding(
        &self,
        current: Option<&str>,
    ) -> Option<(&'static str, &'static [u8])> {
        // 排除占位封面和当前封面，避免连续两次抽到同一张。
        let candidates: Vec<(&'static str, &'static str)> = self
            .cards
            .iter()
            .filter_map(|path| {
                let name = path.rsplit('/').next()?;
                (name != "card0001.jpg" && Some(name) != current).then_some((*path, name))
            })
            .collect();

        if candidates.is_empty() {
            return self
                .card("card0001.jpg")
                .map(|bytes| ("card0001.jpg", bytes));
        }
        let (path, name) = candidates[fastrand::usize(..candidates.len())];
        ASSETS.get_file(path).map(|file| (name, file.contents()))
    }

    pub fn logo(&self, index: usize) -> Option<&'static [u8]> {
        let path = *self.logos.get(index)?;
        ASSETS.get_file(path).map(|file| file.contents())
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

fn collect_files(directory: &str, extension: &str) -> Vec<&'static str> {
    ASSETS
        .get_dir(directory)
        .map(|dir| {
            dir.files()
                .filter_map(|file| file.path().to_str())
                .filter(|path| path.ends_with(extension))
                .collect()
        })
        .unwrap_or_default()
}
