use eframe::egui;
use std::io::Read;

pub fn load_texture(
    ctx: &egui::Context,
    zip_path: &str,
    internal_path: &str,
) -> Option<egui::TextureHandle> {
    let file = std::fs::File::open(zip_path).ok()?;
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(a) => a,
        Err(e) => { println!("Media Error: Failed to open zip: {}", e); return None; }
    };

    let mut img_file = match archive.by_name(internal_path) {
        Ok(f) => f,
        Err(e) => { println!("Media Error: Could not find '{}' in zip: {}", internal_path, e); return None; }
    };

    let mut buffer = Vec::new();
    let _ = img_file.read_to_end(&mut buffer);

    let img = match image::load_from_memory(&buffer) {
        Ok(i) => i,
        Err(e) => { println!("Media Error: Failed to decode image: {}", e); return None; }
    };

    let size = [img.width() as _, img.height() as _];
    let color_img = egui::ColorImage::from_rgba_unmultiplied(
        size, 
        img.to_rgba8().as_flat_samples().as_slice()
    );

    Some(ctx.load_texture("card_img", color_img, Default::default()))
}
