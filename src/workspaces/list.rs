use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    ui.vertical_centered(|ui| {
        if let Some(data) = &app.deck {
            ui.heading(egui::RichText::new(&data.title).strong().size(24.0));
        }
    });

    ui.horizontal(|ui| {
        ui.label("🔍 Search:");
        ui.text_edit_singleline(&mut app.search_query);
    });
    ui.separator();

    let mut next_selection = None;
    if let Some(data) = &app.deck {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, card) in data.cards.iter().enumerate() {
                if !app.search_query.is_empty()
                    && !card
                        .term
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&app.search_query.to_lowercase())
                {
                    continue;
                }
                let is_selected = i == app.selected_index;
                let response = ui.add(egui::SelectableLabel::new(
                    is_selected,
                    format!("{}: {}", i + 1, card.term.as_deref().unwrap_or("(Untitled)")),
                ));
                if response.clicked() {
                    next_selection = Some(i);
                }
                if is_selected {
                    response.scroll_to_me(Some(egui::Align::Center));
                }
            }
        });
    }

    if let Some(i) = next_selection {
        app.set_index(i, ctx);
        app.workspace = crate::Workspace::Flashcard;
    }
}