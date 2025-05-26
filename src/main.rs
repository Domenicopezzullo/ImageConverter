use std::path;

use eframe::{egui::{CentralPanel, ComboBox, IconData, ViewportBuilder}, run_native, HardwareAcceleration, NativeOptions};
use image::ImageFormat;

#[derive(PartialEq, Debug)]
enum ImageExt {
    PNG,
    JPEG,
}


struct Converter {
    file_path: String,
    ext: ImageExt,
}

impl Converter {
    fn new() -> Self {
        Self { ext: ImageExt::PNG, file_path: String::new() }
    }
}
impl eframe::App for Converter {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(&ctx, |ui| {

            ui.text_edit_singleline(&mut self.file_path);
            ComboBox::from_label("Image Extension").selected_text(format!("{:?}", self.ext)).show_ui(ui, |ui| {
                ui.selectable_value(&mut self.ext, ImageExt::PNG, "PNG");
                ui.selectable_value(&mut self.ext, ImageExt::JPEG, "JPEG");
            });
            if ui.button("Convert").clicked() {
                let path = path::Path::new(&self.file_path);
                if !path.exists() {
                    eprintln!("File does not exist");
                    return;
                }
                let ext = path.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.to_ascii_lowercase())
                    .unwrap_or_default();

                match self.ext {
                    ImageExt::PNG if ext == "png" => {
                        eprintln!("Already a PNG file!");
                        return;
                    },
                    ImageExt::JPEG if ext == "jpeg" || ext == "jpg" => {
                        eprintln!("Already a JPEG file!");
                        return;
                    },
                    _ => {}
                }



                let image  = image::open(path).expect("Image could not be opened");
                let output_path = path.with_extension(
                    match self.ext {
                        ImageExt::PNG => "png",
                        ImageExt::JPEG => "jpg",
                    }
                );
                match self.ext {
                    ImageExt::PNG => {
                        if let Err(e) = image.save_with_format(&output_path, ImageFormat::Png) {
                            eprintln!("Conversion failed: {}", e);
                        };
                    },
                    ImageExt::JPEG => {
                        let rgb_image = image.to_rgb8();
                        if let Err(e) = rgb_image.save_with_format(&output_path, ImageFormat::Jpeg) {
                                eprintln!("Conversion to JPEG failed: {}", e);
                        }
                    }
                }
            }
        });
    }
}

fn load_icon() -> IconData {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!(".././skibidi.png");
		let image = image::load_from_memory(icon)
			.expect("Failed to open icon path")
			.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};

	IconData {
	    rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	}
}



fn main() -> eframe::Result {
    let mut options = NativeOptions::default();
    let icon = load_icon();
    options.centered = true;
    options.viewport = ViewportBuilder::default().with_icon(icon);
    options.hardware_acceleration = HardwareAcceleration::Preferred;
    run_native("File Converter", options, Box::new(|_cc| Ok(Box::new(Converter::new()))))
}
