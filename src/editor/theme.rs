//! Visual theme for the editor UI.
//!
//! Defines a consistent color palette, typography scale, and spacing tokens used
//! by every egui panel. Modeled loosely after VS Code's "Dark+" theme.

use bevy::prelude::{Color, Resource};
use bevy_egui::egui::{self, Color32, Stroke};

/// A self-contained theme descriptor for the editor UI.
#[derive(Resource, Debug, Clone)]
pub struct EditorTheme {
    /// Background color of the main window.
    pub bg: Color32,
    /// Background color of panels (slightly lighter than `bg`).
    pub panel_bg: Color32,
    /// Background color of widgets (buttons, inputs).
    pub widget_bg: Color32,
    /// Background color of hovered widgets.
    pub widget_hover: Color32,
    /// Foreground text color.
    pub fg: Color32,
    /// Muted / secondary text color.
    pub fg_muted: Color32,
    /// Primary accent color (selection, focus rings, primary buttons).
    pub accent: Color32,
    /// Secondary accent color (hover states).
    pub accent_hover: Color32,
    /// Warning color.
    pub warning: Color32,
    /// Error / danger color.
    pub danger: Color32,
    /// Success color.
    pub success: Color32,
    /// Stroke used by selected items.
    pub selection_stroke: Stroke,
    /// Stroke used by hovered items.
    pub hover_stroke: Stroke,
}

impl Default for EditorTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl EditorTheme {
    /// The default dark theme (VS Code "Dark+" inspired).
    pub fn dark() -> Self {
        Self {
            bg: Color32::from_rgb(30, 30, 30),
            panel_bg: Color32::from_rgb(37, 37, 38),
            widget_bg: Color32::from_rgb(60, 60, 60),
            widget_hover: Color32::from_rgb(75, 75, 75),
            fg: Color32::from_rgb(220, 220, 220),
            fg_muted: Color32::from_rgb(153, 153, 153),
            accent: Color32::from_rgb(0, 122, 204),
            accent_hover: Color32::from_rgb(14, 140, 228),
            warning: Color32::from_rgb(204, 153, 0),
            danger: Color32::from_rgb(204, 0, 0),
            success: Color32::from_rgb(40, 180, 80),
            selection_stroke: Stroke::new(1.5, Color32::from_rgb(0, 122, 204)),
            hover_stroke: Stroke::new(1.0, Color32::from_rgb(120, 120, 120)),
        }
    }

    /// A light theme (VS Code "Light+" inspired).
    pub fn light() -> Self {
        Self {
            bg: Color32::from_rgb(245, 245, 245),
            panel_bg: Color32::from_rgb(252, 252, 252),
            widget_bg: Color32::from_rgb(220, 220, 220),
            widget_hover: Color32::from_rgb(200, 200, 200),
            fg: Color32::from_rgb(40, 40, 40),
            fg_muted: Color32::from_rgb(120, 120, 120),
            accent: Color32::from_rgb(0, 122, 204),
            accent_hover: Color32::from_rgb(14, 140, 228),
            warning: Color32::from_rgb(180, 140, 0),
            danger: Color32::from_rgb(204, 0, 0),
            success: Color32::from_rgb(40, 160, 70),
            selection_stroke: Stroke::new(1.5, Color32::from_rgb(0, 122, 204)),
            hover_stroke: Stroke::new(1.0, Color32::from_rgb(120, 120, 120)),
        }
    }

    /// Apply this theme to an [`egui::Context`].
    pub fn apply(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals = self.to_visuals();
        style.spacing.item_spacing = egui::vec2(6.0, 4.0);
        style.spacing.window_margin = egui::Margin::same(8.0);
        ctx.set_style(style);
    }

    fn to_visuals(&self) -> egui::Visuals {
        let mut v = egui::Visuals::dark();
        v.panel_fill = self.panel_bg;
        v.window_fill = self.panel_bg;
        v.extreme_bg_color = self.bg;
        v.faint_bg_color = self.widget_bg;
        v.widgets.noninteractive.bg_fill = self.widget_bg;
        v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, self.fg_muted);
        v.widgets.inactive.bg_fill = self.widget_bg;
        v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, self.fg);
        v.widgets.hovered.bg_fill = self.widget_hover;
        v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, self.fg);
        v.widgets.active.bg_fill = self.accent;
        v.widgets.active.fg_stroke = egui::Stroke::new(1.0, self.fg);
        v.widgets.open.bg_fill = self.accent_hover;
        v.selection.bg_fill = self.accent;
        v.selection.stroke = self.selection_stroke;
        v.hyperlink_color = self.accent_hover;
        v
    }
}

/// Convert a Bevy [`Color`] to an egui [`Color32`].
pub fn bevy_color_to_egui(color: Color) -> Color32 {
    let srgba: bevy::color::Srgba = color.into();
    let [r, g, b, a] = srgba.to_u8_array();
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

/// Convert an egui [`Color32`] to a Bevy [`Color`].
pub fn egui_color_to_bevy(color: Color32) -> Color {
    let [r, g, b, a] = color.to_array();
    Color::srgba(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )
}
