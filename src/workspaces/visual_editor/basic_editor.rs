// src/workspaces/visual_editor/basic_editor.rs

use crate::models::{Card, LexicalInfo};
use eframe::egui;

pub struct BasicEditorOptions<'a> {
    pub editor_mode: bool,

    pub show_lang_codes: bool,
    pub show_phonetic: bool,
    pub show_part_of_speech: bool,
    pub show_notes: bool,
    pub show_tags: bool,

    pub lang_search_query: &'a mut String,

    pub deck_term_fallback: &'a str,
    pub deck_def_fallback: &'a str,

    pub font_size_header: f32,
    pub font_size_body: f32,

    pub show_images: bool,
    pub current_texture: Option<&'a egui::TextureHandle>,
}

#[derive(Debug, Default)]
pub struct BasicEditorActions {
    pub json_needs_sync: bool,
    pub core_needs_sync: bool,
    pub action_add_sentence: bool,
    pub action_remove_sentence: Option<usize>,
}

impl BasicEditorActions {
    fn mark_changed(&mut self) {
        self.json_needs_sync = true;
        self.core_needs_sync = true;
    }
}

pub fn render(
    ui: &mut egui::Ui,
    card: &mut Card,
    options: BasicEditorOptions<'_>,
) -> BasicEditorActions {
    let mut actions = BasicEditorActions::default();

    // Copy the right-column values before `options` is moved into the left column.
    let right_show_images = options.show_images;
    let right_editor_mode = options.editor_mode;
    let right_current_texture = options.current_texture;

    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            ui.add_space(20.0);

            if options.editor_mode {
                render_edit_left_column(ui, card, options, &mut actions);
            } else {
                render_reader_left_column(ui, card, options);
            }
        });

        cols[1].vertical_centered(|ui| {
            super::media_panel::render(
                ui,
                super::media_panel::MediaPanel {
                    show_images: right_show_images,
                    editor_mode: right_editor_mode,
                    current_texture: right_current_texture,
                },
            );
        });
    });

    actions
}

fn render_edit_left_column(
    ui: &mut egui::Ui,
    card: &mut Card,
    options: BasicEditorOptions<'_>,
    actions: &mut BasicEditorActions,
) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Term")
                .strong()
                .color(egui::Color32::GRAY),
        );

        if options.show_lang_codes {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if super::language_picker::render(
                    ui,
                    super::language_picker::LanguagePicker {
                        id_source: "term_lang_combo",
                        current_lang: &mut card.term_lang,
                        fallback_lang: options.deck_term_fallback,
                        search_query: options.lang_search_query,
                    },
                ) {
                    actions.mark_changed();
                }
            });
        }
    });

    let term_ref = card.term.get_or_insert_with(String::new);
    let term_resp = ui.add(
        egui::TextEdit::singleline(term_ref)
            .font(egui::TextStyle::Heading)
            .frame(false)
            .desired_width(f32::INFINITY),
    );

    if term_resp.changed() {
        actions.mark_changed();
    }

    if options.show_phonetic || options.show_part_of_speech {
        let lexical = card.lexical.get_or_insert_with(|| LexicalInfo {
            phonetic: None,
            part_of_speech: None,
            forms: vec![],
            synonyms: vec![],
            antonyms: vec![],
        });

        if options.show_phonetic {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Phonetic:").weak().size(12.0));

                let mut phonetic_str = lexical.phonetic.clone().unwrap_or_default();

                if ui
                    .add(egui::TextEdit::singleline(&mut phonetic_str).desired_width(150.0))
                    .changed()
                {
                    lexical.phonetic = if phonetic_str.trim().is_empty() {
                        None
                    } else {
                        Some(phonetic_str.trim().to_string())
                    };

                    actions.mark_changed();
                }
            });
        }

        if options.show_part_of_speech {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Part of Speech:")
                        .weak()
                        .size(12.0),
                );

                let mut pos_str = lexical.part_of_speech.clone().unwrap_or_default();

                if ui
                    .add(egui::TextEdit::singleline(&mut pos_str).desired_width(150.0))
                    .changed()
                {
                    lexical.part_of_speech = if pos_str.trim().is_empty() {
                        None
                    } else {
                        Some(pos_str.trim().to_string())
                    };

                    actions.mark_changed();
                }
            });
        }
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Definition")
                .strong()
                .color(egui::Color32::GRAY),
        );

        if options.show_lang_codes {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if super::language_picker::render(
                    ui,
                    super::language_picker::LanguagePicker {
                        id_source: "def_lang_combo",
                        current_lang: &mut card.def_lang,
                        fallback_lang: options.deck_def_fallback,
                        search_query: options.lang_search_query,
                    },
                ) {
                    actions.mark_changed();
                }
            });
        }
    });

    let definition_ref = card.definition.get_or_insert_with(String::new);
    let def_resp = ui.add(
        egui::TextEdit::multiline(definition_ref)
            .font(egui::TextStyle::Body)
            .frame(false)
            .desired_width(f32::INFINITY),
    );

    if def_resp.changed() {
        actions.mark_changed();
    }

    if options.show_notes {
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Notes:").weak().size(12.0));

        let mut notes_str = card.notes.clone().unwrap_or_default();

        if ui
            .add(
                egui::TextEdit::multiline(&mut notes_str)
                    .frame(false)
                    .desired_width(f32::INFINITY),
            )
            .changed()
        {
            card.notes = if notes_str.trim().is_empty() {
                None
            } else {
                Some(notes_str.trim().to_string())
            };

            actions.mark_changed();
        }
    }

    if options.show_tags {
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Tags (comma separated):")
                    .weak()
                    .size(12.0),
            );

            let mut tags_str = card.tags.join(", ");

            if ui
                .add(egui::TextEdit::singleline(&mut tags_str).desired_width(f32::INFINITY))
                .changed()
            {
                card.tags = tags_str
                    .split(',')
                    .map(|tag| tag.trim().to_string())
                    .filter(|tag| !tag.is_empty())
                    .collect();

                actions.mark_changed();
            }
        });
    }

    let example_actions = super::examples::render_edit(ui, card);

    if example_actions.add_example {
        actions.action_add_sentence = true;
    }

    if let Some(index) = example_actions.remove_example {
        actions.action_remove_sentence = Some(index);
    }

    if example_actions.changed {
        actions.mark_changed();
    }
}

fn render_reader_left_column(
    ui: &mut egui::Ui,
    card: &Card,
    options: BasicEditorOptions<'_>,
) {
    ui.heading(
        egui::RichText::new(card.term.as_deref().unwrap_or(""))
            .size(options.font_size_header)
            .strong(),
    );

    if let Some(lexical) = &card.lexical {
        if let Some(phonetic) = &lexical.phonetic {
            ui.label(
                egui::RichText::new(format!("/ {} /", phonetic))
                    .color(egui::Color32::GRAY),
            );
        }

        if let Some(pos) = &lexical.part_of_speech {
            ui.label(egui::RichText::new(pos).italics().size(12.0));
        }
    }

    ui.separator();

    ui.label(
        egui::RichText::new(card.definition.as_deref().unwrap_or(""))
            .size(options.font_size_body),
    );

    super::examples::render_read(ui, card);

    if let Some(notes) = &card.notes {
        ui.add_space(15.0);
        ui.label(egui::RichText::new("Notes:").strong().size(12.0));
        ui.label(notes);
    }

    if !card.tags.is_empty() {
        ui.add_space(10.0);

        ui.horizontal_wrapped(|ui| {
            for tag in &card.tags {
                ui.label(egui::RichText::new(format!("#{}", tag)).weak());
            }
        });
    }
}