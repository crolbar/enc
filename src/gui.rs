use eframe::egui;
use egui::Align;
use crate::gui_main;

pub fn main() -> Result<(), eframe::Error> {
    let mut key_path = String::new();
    let mut file_path = String::new();
    let mut output_path = String::new();
    let mut cli_output = String::new();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 400.0)),
        app_id: Some("enc".to_string()),
        drag_and_drop_support: true,
        ..Default::default()
    };


    eframe::run_simple_native("enc", options, move |ctx, frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::gui_zoom::zoom_with_keyboard_shortcuts(ctx, frame.info().native_pixels_per_point);
            ctx.set_visuals(egui::Visuals { 
                panel_fill: egui::Color32::BLACK, 
                override_text_color: Some(egui::Color32::WHITE),
                ..Default::default() 
            });

            // close button
            ui.with_layout(egui::Layout::right_to_left(Align::TOP), |ui| {
                if ui.add(egui::Button::new(egui::RichText::new("X").size(20.0)).fill(egui::Color32::BLACK).min_size(egui::Vec2 { x: 20.0, y: 20.0 })).clicked() {
                    std::process::exit(0);
                } 
            });

            ui.shrink_height_to_current();
            ui.vertical_centered( |ui| {

                // file for encoding/decoding
                ui.add_space(5.0);
                if ui.add(egui::Button::new("File to encdoe/decode").fill(egui::Color32::BLACK).min_size(egui::Vec2 { x: 100.0, y: 50.0 })).clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        file_path = path.display().to_string();
                    }
                }
                if !file_path.is_empty() {
                    ui.label("Picked file:");
                    ui.monospace(&file_path);
                }


                // key file
                ui.add_space(5.0);
                let key_button = ui.add(egui::Button::new("Key").fill(egui::Color32::BLACK).min_size(egui::Vec2 { x: 100.0, y: 50.0 })).on_hover_text("generated by encoding a file and can be used for encoding and decoding");
                if key_button.clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        key_path = path.display().to_string();
                    }
                } else if key_button.secondary_clicked() {
                    key_path.clear()
                }
                if !key_path.is_empty() {
                    ui.label("Picked key:");
                    ui.monospace(&key_path);
                }


                // output location of the encode/decoded file   
                ui.add_space(5.0);
                let output_button = ui.add(egui::Button::new("Output location (Optional)").fill(egui::Color32::BLACK).min_size(egui::Vec2 { x: 100.0, y: 50.0 })).on_hover_text("if not specified the input file is the output location");
                if output_button.clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        output_path = path.display().to_string();
                    }
                } else if output_button.secondary_clicked() {
                    output_path.clear()
                }
                if !output_path.is_empty() {
                    ui.label("Output path:");
                    ui.monospace(&output_path);
                }


                // encode/decode buttons
                if !file_path.is_empty() && !key_path.is_empty() {
                    if ui.add(egui::Button::new("Encode/Decode").fill(egui::Color32::BLACK).min_size(egui::Vec2 { x: 100.0, y: 50.0 })).clicked() {
                        cli_output = gui_main(&file_path, output_file_path(&file_path, &output_path), &key_path);
                    } 
                } else if !file_path.is_empty() {
                    if ui.add(egui::Button::new("Encode").fill(egui::Color32::BLACK).min_size(egui::Vec2 { x: 100.0, y: 50.0 })).clicked() {
                        cli_output = gui_main(&file_path, output_file_path(&file_path, &output_path), &key_path);
                    } 
                }


                ui.add_space(30.0);
                if !cli_output.is_empty() { ui.label(&cli_output); }
            });
        });
    })
}

fn output_file_path(file_path: &String, output_path: &String) -> String {
    if output_path.is_empty() { 
        file_path.to_string()
    } else {
        output_path.to_string()
    }
}