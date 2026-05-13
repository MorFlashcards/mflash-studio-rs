use crate::MFlashStudioApp;
use eframe::egui;

const TILE_WIDTH: f32 = 170.0;
const TILE_HEIGHT: f32 = 28.0;
const TILE_GAP_X: f32 = 8.0;
const TILE_GAP_Y: f32 = 6.0;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    ui.vertical_centered(|ui| {
        if let Some(data) = &app.deck {
            ui.heading(egui::RichText::new(&data.title).strong().size(24.0));
        }
    });

    ui.horizontal(|ui| {
        ui.label("🔍 Search:");
        ui.add_sized(
            [320.0, 22.0],
            egui::TextEdit::singleline(&mut app.search_query),
        );
    });

    ui.separator();

    let mut next_selection = None;

    let Some(data) = &app.deck else {
        return;
    };

    let search = app.search_query.trim().to_lowercase();

    let filtered_indices: Vec<usize> = data
        .cards
        .iter()
        .enumerate()
        .filter_map(|(i, card)| {
            let term = card.term.as_deref().unwrap_or("");

            if search.is_empty() || term.to_lowercase().contains(&search) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    if filtered_indices.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label("No matching cards.");
        });
        return;
    }

    let available_width = ui.available_width().max(TILE_WIDTH);
    let columns = ((available_width + TILE_GAP_X) / (TILE_WIDTH + TILE_GAP_X))
        .floor()
        .max(1.0) as usize;

    let row_count = filtered_indices.len().div_ceil(columns);
    let row_height = TILE_HEIGHT + TILE_GAP_Y;

    ui.label(
        egui::RichText::new(format!(
            "{} card{} shown · {} column{}",
            filtered_indices.len(),
            if filtered_indices.len() == 1 { "" } else { "s" },
            columns,
            if columns == 1 { "" } else { "s" },
        ))
        .weak(),
    );

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show_rows(ui, row_height, row_count, |ui, row_range| {
            for row in row_range {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = TILE_GAP_X;

                    for col in 0..columns {
                        let filtered_pos = row * columns + col;

                        let Some(&card_index) = filtered_indices.get(filtered_pos) else {
                            break;
                        };

                        let card = &data.cards[card_index];
                        let term = card.term.as_deref().unwrap_or("(Untitled)");
                        let is_selected = card_index == app.selected_index;

                        let label = format!("{}: {}", card_index + 1, term);

                        let response = ui.add_sized(
                            [TILE_WIDTH, TILE_HEIGHT],
                            egui::SelectableLabel::new(is_selected, label),
                        );

                        if response.clicked() {
                            next_selection = Some(card_index);
                        }
                    }
                });
            }
        });

    if let Some(i) = next_selection {
        app.set_index(i, ctx);
        app.workspace = crate::Workspace::VisualEditor;
    }
}
