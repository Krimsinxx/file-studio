use iced::{Color, Theme};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    pub name: String,
    pub background: String,
    pub surface: String,
    pub surface_alt: String,
    pub border: String,
    pub foreground: String,
    pub muted: String,
    pub accent: String,
    pub selection: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::vscode_dark()
    }
}

impl ThemeConfig {
    pub fn vscode_dark() -> Self {
        Self {
            name: "VS Code Dark+".into(),
            background: "#1E1E1E".into(),
            surface: "#252526".into(),
            surface_alt: "#333333".into(),
            border: "#3C3C3C".into(),
            foreground: "#D4D4D4".into(),
            muted: "#858585".into(),
            accent: "#007ACC".into(),
            selection: "#264F78".into(),
            success: "#4EC9B0".into(),
            warning: "#DCDCAA".into(),
            danger: "#F14C4C".into(),
        }
    }

    pub fn monokai() -> Self {
        Self {
            name: "Monokai-inspired".into(),
            background: "#272822".into(),
            surface: "#1E1F1C".into(),
            surface_alt: "#3E3D32".into(),
            border: "#49483E".into(),
            foreground: "#F8F8F2".into(),
            muted: "#A6A69C".into(),
            accent: "#A6E22E".into(),
            selection: "#49483E".into(),
            success: "#A6E22E".into(),
            warning: "#E6DB74".into(),
            danger: "#F92672".into(),
        }
    }

    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".into(),
            background: "#000000".into(),
            surface: "#0A0A0A".into(),
            surface_alt: "#202020".into(),
            border: "#FFFFFF".into(),
            foreground: "#FFFFFF".into(),
            muted: "#D0D0D0".into(),
            accent: "#00A8FF".into(),
            selection: "#004A80".into(),
            success: "#00FF9C".into(),
            warning: "#FFFF00".into(),
            danger: "#FF5555".into(),
        }
    }

    pub fn color(value: &str) -> Color {
        let hex = value.trim().trim_start_matches('#');
        if hex.len() != 6 {
            return Color::WHITE;
        }
        let Ok(rgb) = u32::from_str_radix(hex, 16) else {
            return Color::WHITE;
        };
        Color::from_rgb8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
    }

    pub fn iced_theme(&self) -> Theme {
        Theme::custom(
            self.name.clone(),
            iced::theme::Palette {
                background: Self::color(&self.background),
                text: Self::color(&self.foreground),
                primary: Self::color(&self.accent),
                success: Self::color(&self.success),
                danger: Self::color(&self.danger),
                warning: Self::color(&self.warning),
            },
        )
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
        let theme: Self = serde_json::from_str(&contents).map_err(|error| error.to_string())?;
        theme.validate()?;
        Ok(theme)
    }

    pub fn validate(&self) -> Result<(), String> {
        for (label, value) in [
            ("background", &self.background), ("surface", &self.surface),
            ("surface_alt", &self.surface_alt), ("border", &self.border),
            ("foreground", &self.foreground), ("muted", &self.muted),
            ("accent", &self.accent), ("selection", &self.selection),
            ("success", &self.success), ("warning", &self.warning), ("danger", &self.danger),
        ] {
            let hex = value.trim().trim_start_matches('#');
            if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(format!("Invalid {label} color: expected #RRGGBB"));
            }
        }
        if self.name.trim().is_empty() {
            return Err("Theme name cannot be empty".into());
        }
        Ok(())
    }
}
