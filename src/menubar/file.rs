use crate::MFlashStudioApp;
use eframe::egui;
use rfd::FileDialog;
use uuid::Uuid;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.menu_button("File", |ui| {
        if ui.button("📂 Open Deck...").clicked() {
            ui.close_menu();

            // 1. Pop open the native OS file picker
            if let Some(file_path) = FileDialog::new()
                .add_filter("mflash decks", &["mflash"])
                .pick_file()
            {
                println!("Opening file: {}", file_path.display());

                let workspace_id = Uuid::new_v4().to_string();

                // 2. Fire up the Core Engine
                match mflash_core::workspace::unpack_deck(&file_path, &workspace_id) {
                    Ok(cache_dir) => {
                        println!("Engine unpacked to: {}", cache_dir.display());

                        // 3. Connect to SQLite and dump the live deck state to JSON
                        match mflash_core::db::init_workspace_db(&cache_dir) {
                            Ok(mut conn) => {
                                // If the unpacked workspace has a deck.pb, sync it into SQLite first.
                                if let Ok(deck) = mflash_core::schema::read_deck(&cache_dir) {
                                    if let Err(e) = mflash_core::db::import_pb_to_db(&mut conn, &deck) {
                                        eprintln!("Failed to sync deck.pb into SQLite: {}", e);
                                        return;
                                    }
                                } else {
                                    println!("No valid deck.pb found. Opening as empty workspace.");
                                }

                                match mflash_core::db::export_db_to_pb(&conn) {
                                    Ok(pb_deck) => {
                                        match mflash_core::translator::to_json(&pb_deck) {
                                            Ok(json_dump) => {
                                                // 4. Save it to the GUI's text memory
                                                app.active_workspace_id = Some(workspace_id);
                                                app.active_schema_json = Some(json_dump.clone());

                                                // Keep the existing app fields updated too
                                                app.path = file_path.display().to_string();
                                                app.active_schema_format = crate::SchemaFormat::Json;
                                                app.raw_schema_text = json_dump;
                                                app.json_error = None;

                                                // 5. Sync the text into the Rust structs!
                                                if app.sync_text_to_deck() {
                                                    println!("✅ Successfully synced engine data to UI structs!");
                                                } else {
                                                    eprintln!("❌ Failed to parse engine JSON into UI structs!");
                                                    // Print the exact Serde error so we know what to fix!
                                                    eprintln!("Serde Error: {:?}", app.json_error);
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("Failed to translate deck to JSON: {}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Failed to export SQLite database to Protobuf: {}",
                                            e
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to initialize workspace database: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Engine failed to unpack deck: {}", e);
                    }
                }
            }
        }

        ui.separator();

        if ui.button("Save").clicked() {
            app.save_deck();
            ui.close_menu();
        }

        if ui.button("Save As...").clicked() {
            if let Some(path) = FileDialog::new()
                .add_filter("mflash files", &["mflash"])
                .set_file_name("untitled.mflash")
                .save_file()
            {
                app.save_deck_as(path.display().to_string());
            }
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Export to JSON...").clicked() {
            if let Some(path) = FileDialog::new()
                .add_filter("JSON files", &["json"])
                .set_file_name("export.json")
                .save_file()
            {
                let _ = std::fs::write(path, &app.raw_schema_text);
            }
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Close Deck").clicked() {
            app.deck = None;
            app.path = String::new();
            app.raw_schema_text = String::new();
            app.current_texture = None;
            app.active_workspace_id = None;
            app.active_schema_json = None;
            app.json_error = None;
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Quit").clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}