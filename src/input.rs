use crate::{MFlashStudioApp, Workspace};
use eframe::egui;

pub fn parse_shortcut(input: &str) -> Option<egui::KeyboardShortcut> {
    let mut modifiers = egui::Modifiers::NONE;
    let mut key: Option<egui::Key> = None;

    for raw_part in input.split('+') {
        let part = raw_part.trim().to_lowercase();

        match part.as_str() {
            "ctrl" | "cmd" | "command" => modifiers.command = true,
            "shift" => modifiers.shift = true,
            "alt" | "option" => modifiers.alt = true,

            "a" => key = Some(egui::Key::A),
            "b" => key = Some(egui::Key::B),
            "c" => key = Some(egui::Key::C),
            "d" => key = Some(egui::Key::D),
            "e" => key = Some(egui::Key::E),
            "f" => key = Some(egui::Key::F),
            "g" => key = Some(egui::Key::G),
            "h" => key = Some(egui::Key::H),
            "i" => key = Some(egui::Key::I),
            "j" => key = Some(egui::Key::J),
            "k" => key = Some(egui::Key::K),
            "l" => key = Some(egui::Key::L),
            "m" => key = Some(egui::Key::M),
            "n" => key = Some(egui::Key::N),
            "o" => key = Some(egui::Key::O),
            "p" => key = Some(egui::Key::P),
            "q" => key = Some(egui::Key::Q),
            "r" => key = Some(egui::Key::R),
            "s" => key = Some(egui::Key::S),
            "t" => key = Some(egui::Key::T),
            "u" => key = Some(egui::Key::U),
            "v" => key = Some(egui::Key::V),
            "w" => key = Some(egui::Key::W),
            "x" => key = Some(egui::Key::X),
            "y" => key = Some(egui::Key::Y),
            "z" => key = Some(egui::Key::Z),

            "0" | "num0" => key = Some(egui::Key::Num0),
            "1" | "num1" => key = Some(egui::Key::Num1),
            "2" | "num2" => key = Some(egui::Key::Num2),
            "3" | "num3" => key = Some(egui::Key::Num3),
            "4" | "num4" => key = Some(egui::Key::Num4),
            "5" | "num5" => key = Some(egui::Key::Num5),
            "6" | "num6" => key = Some(egui::Key::Num6),
            "7" | "num7" => key = Some(egui::Key::Num7),
            "8" | "num8" => key = Some(egui::Key::Num8),
            "9" | "num9" => key = Some(egui::Key::Num9),

            "," | "comma" => key = Some(egui::Key::Comma),
            "." | "period" => key = Some(egui::Key::Period),
            "plus" | "+" | "=" => key = Some(egui::Key::Equals),
            "minus" | "-" => key = Some(egui::Key::Minus),

            "enter" | "return" => key = Some(egui::Key::Enter),
            "escape" | "esc" => key = Some(egui::Key::Escape),
            "space" => key = Some(egui::Key::Space),
            "tab" => key = Some(egui::Key::Tab),
            "backspace" => key = Some(egui::Key::Backspace),
            "delete" | "del" => key = Some(egui::Key::Delete),

            "arrowup" | "up" => key = Some(egui::Key::ArrowUp),
            "arrowdown" | "down" => key = Some(egui::Key::ArrowDown),
            "arrowleft" | "left" => key = Some(egui::Key::ArrowLeft),
            "arrowright" | "right" => key = Some(egui::Key::ArrowRight),

            "home" => key = Some(egui::Key::Home),
            "end" => key = Some(egui::Key::End),
            "pageup" => key = Some(egui::Key::PageUp),
            "pagedown" => key = Some(egui::Key::PageDown),

            "f1" => key = Some(egui::Key::F1),
            "f2" => key = Some(egui::Key::F2),
            "f3" => key = Some(egui::Key::F3),
            "f4" => key = Some(egui::Key::F4),
            "f5" => key = Some(egui::Key::F5),
            "f6" => key = Some(egui::Key::F6),
            "f7" => key = Some(egui::Key::F7),
            "f8" => key = Some(egui::Key::F8),
            "f9" => key = Some(egui::Key::F9),
            "f10" => key = Some(egui::Key::F10),
            "f11" => key = Some(egui::Key::F11),
            "f12" => key = Some(egui::Key::F12),

            _ => return None,
        }
    }

    key.map(|key| egui::KeyboardShortcut::new(modifiers, key))
}

fn shortcut_pressed(ctx: &egui::Context, shortcut_text: &str) -> bool {
    let Some(shortcut) = parse_shortcut(shortcut_text) else {
        return false;
    };

    ctx.input_mut(|input| input.consume_shortcut(&shortcut))
}

impl MFlashStudioApp {
    fn visible_workspaces(&self) -> Vec<Workspace> {
        let mut workspaces = Vec::new();

        if self.config.workspaces.show_deck {
            workspaces.push(Workspace::Deck);
        }

        if self.config.workspaces.show_browse {
            workspaces.push(Workspace::Browse);
        }

        if self.config.workspaces.show_visual_editor {
            workspaces.push(Workspace::VisualEditor);
        }

        if self.config.workspaces.show_media {
            workspaces.push(Workspace::Media);
        }

        if self.config.workspaces.show_schema_editor {
            workspaces.push(Workspace::SchemaEditor);
        }

        if workspaces.is_empty() {
            workspaces.push(Workspace::Browse);
        }

        workspaces
    }

    fn switch_workspace_by_offset(&mut self, offset: isize, ctx: &egui::Context) {
        let workspaces = self.visible_workspaces();

        if workspaces.is_empty() {
            return;
        }

        let current_index = workspaces
            .iter()
            .position(|workspace| *workspace == self.workspace)
            .unwrap_or(0);

        let len = workspaces.len() as isize;
        let next_index = (current_index as isize + offset).rem_euclid(len) as usize;

        self.switch_workspace(workspaces[next_index], ctx);
    }

    fn switch_to_visible_workspace(&mut self, target: Workspace, ctx: &egui::Context) {
        if self.visible_workspaces().contains(&target) {
            self.switch_workspace(target, ctx);
        }
    }

    pub fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let shortcuts = self.config.shortcuts.clone();

        if shortcut_pressed(ctx, &shortcuts.open_settings) {
            self.show_settings = true;
        }

        if shortcut_pressed(ctx, &shortcuts.save_deck) {
            self.save_deck();
        }

        if shortcut_pressed(ctx, "Ctrl+Z") {
            self.undo(ctx);
        }

        if shortcut_pressed(ctx, "Ctrl+Y") {
            self.redo(ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.next_workspace) {
            self.switch_workspace_by_offset(1, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.prev_workspace) {
            self.switch_workspace_by_offset(-1, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.workspace_deck) {
            self.switch_to_visible_workspace(Workspace::Deck, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.workspace_browse) {
            self.switch_to_visible_workspace(Workspace::Browse, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.workspace_visual_editor) {
            self.switch_to_visible_workspace(Workspace::VisualEditor, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.workspace_media) {
            self.switch_to_visible_workspace(Workspace::Media, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.workspace_schema_editor) {
            self.switch_to_visible_workspace(Workspace::SchemaEditor, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.toggle_find) {
            if self.workspace == Workspace::SchemaEditor {
                self.find_visible = !self.find_visible;

                if !self.find_visible {
                    self.find_matches.clear();
                    self.current_match_idx = 0;
                    self.replace_visible = false;
                }
            }
        }

        if shortcut_pressed(ctx, "Ctrl+H") {
            if self.workspace == Workspace::SchemaEditor {
                self.find_visible = true;
                self.replace_visible = !self.replace_visible;
            }
        }

        if shortcut_pressed(ctx, &shortcuts.next_list_item) {
            let next = self.selected_index + 1;
            self.set_index(next, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.prev_list_item) {
            let prev = self.selected_index.saturating_sub(1);
            self.set_index(prev, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.next_card) {
            let next = self.selected_index + 1;
            self.set_index(next, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.prev_card) {
            let prev = self.selected_index.saturating_sub(1);
            self.set_index(prev, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.view_card) {
            self.switch_to_visible_workspace(Workspace::VisualEditor, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.back_to_list) {
            self.switch_to_visible_workspace(Workspace::Browse, ctx);
        }

        if shortcut_pressed(ctx, &shortcuts.exit_fullscreen) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        }

        if shortcut_pressed(ctx, &shortcuts.zoom_in) {
            let zoom = ctx.zoom_factor();
            ctx.set_zoom_factor((zoom + 0.1).min(3.0));
        }

        if shortcut_pressed(ctx, &shortcuts.zoom_out) {
            let zoom = ctx.zoom_factor();
            ctx.set_zoom_factor((zoom - 0.1).max(0.5));
        }

        if shortcut_pressed(ctx, &shortcuts.actual_size) {
            ctx.set_zoom_factor(1.0);
        }
    }
}
