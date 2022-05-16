use egui::{Slider, Ui};

use crate::editor_data::EditorGraphicsData;

use super::{PaletteEditor, SpriteEditor, SpriteSheetEditor};

#[derive(Clone, Debug, PartialEq)]
pub enum GraphicsEditorMode {
    Palette,
    SpriteSheet,
    Sprite,
}

impl Default for GraphicsEditor {
    fn default() -> Self {
        Self {
            mode: GraphicsEditorMode::Palette,
            palette_editor: PaletteEditor::default(),
            sprite_sheet_editor: SpriteSheetEditor::default(),
            sprite_editor: SpriteEditor::default(),

            scale: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphicsEditor {
    pub mode: GraphicsEditorMode,
    pub palette_editor: PaletteEditor,
    pub sprite_sheet_editor: SpriteSheetEditor,
    pub sprite_editor: SpriteEditor,

    pub scale: usize,
}

impl GraphicsEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, GraphicsEditorMode::Palette, "Palettes");
        ui.selectable_value(
            &mut self.mode,
            GraphicsEditorMode::SpriteSheet,
            "Sprite Sheets",
        );
        ui.selectable_value(&mut self.mode, GraphicsEditorMode::Sprite, "Sprite Editor");
    }

    pub fn draw_contents(&mut self, ui: &mut Ui, data: &mut EditorGraphicsData) {
        match self.mode {
            GraphicsEditorMode::Palette => {
                self.palette_editor
                    .draw(ui, data, &self.sprite_sheet_editor)
            }
            GraphicsEditorMode::SpriteSheet => {
                self.sprite_sheet_editor
                    .draw(ui, data, &self.palette_editor, self.scale)
            }
            GraphicsEditorMode::Sprite => self.sprite_editor.draw(ui),
        };
    }

    pub fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Sprite Scaling:");
            ui.add(Slider::new(&mut self.scale, 1..=100));
        });
    }
}
