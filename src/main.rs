#![windows_subsystem = "windows"]

mod theme;

use std::{fs, path::PathBuf};

use iced::{
    Alignment, Background, Border, Color, Element, Fill, Task, Theme,
    widget::{button, column, container, row, scrollable, text, text_editor, text_input},
};
use theme::ThemeConfig;

fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .title("File Studio")
        .theme(|app: &App| app.theme.iced_theme())
        .window(iced::window::Settings {
            size: iced::Size::new(1280.0, 720.0),
            min_size: Some(iced::Size::new(650.0, 450.0)),
            ..iced::window::Settings::default()
        })
        .run()
}

#[derive(Debug, Clone)]
struct FileEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
}

struct App {
    current_dir: PathBuf,
    path_input: String,
    entries: Vec<FileEntry>,
    editor: text_editor::Content,
    active_file: Option<PathBuf>,
    dirty: bool,
    status: String,
    theme: ThemeConfig,
    show_explorer: bool,
}

impl Default for App {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut app = Self {
            path_input: current_dir.display().to_string(),
            current_dir,
            entries: Vec::new(),
            editor: text_editor::Content::new(),
            active_file: None,
            dirty: false,
            status: "Ready".into(),
            theme: load_saved_theme(),
            show_explorer: true,
        };
        app.read_directory();
        app
    }
}

#[derive(Debug, Clone)]
enum Message {
    PathChanged(String),
    Navigate,
    GoUp,
    Open(PathBuf),
    EditorAction(text_editor::Action),
    NewFile,
    Save,
    SaveAs,
    Refresh,
    ImportTheme,
    UseVsCodeTheme,
    UseMonokai,
    UseHighContrast,
    ToggleExplorer,
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::PathChanged(value) => app.path_input = value,
        Message::Navigate => {
            let path = PathBuf::from(app.path_input.trim());
            if path.is_dir() {
                app.current_dir = path;
                app.path_input = app.current_dir.display().to_string();
                app.read_directory();
                app.status = "Folder opened".into();
            } else {
                app.status = "Folder does not exist".into();
            }
        }
        Message::GoUp => {
            if let Some(parent) = app.current_dir.parent() {
                app.current_dir = parent.to_path_buf();
                app.path_input = app.current_dir.display().to_string();
                app.read_directory();
            }
        }
        Message::Open(path) => app.open_file(path),
        Message::EditorAction(action) => {
            app.editor.perform(action);
            app.dirty = true;
            app.status = "Modified".into();
        }
        Message::NewFile => {
            app.active_file = None;
            app.editor = text_editor::Content::new();
            app.dirty = false;
            app.status = "New file".into();
        }
        Message::Save => app.save_file(),
        Message::SaveAs => app.save_file_as(),
        Message::Refresh => app.read_directory(),
        Message::ImportTheme => app.import_theme(),
        Message::UseVsCodeTheme => app.set_theme(ThemeConfig::vscode_dark()),
        Message::UseMonokai => app.set_theme(ThemeConfig::monokai()),
        Message::UseHighContrast => app.set_theme(ThemeConfig::high_contrast()),
        Message::ToggleExplorer => app.show_explorer = !app.show_explorer,
    }
    Task::none()
}

impl App {
    fn read_directory(&mut self) {
        let mut entries = Vec::new();
        match fs::read_dir(&self.current_dir) {
            Ok(read_dir) => {
                for item in read_dir.flatten() {
                    let path = item.path();
                    let is_dir = path.is_dir();
                    let name = item.file_name().to_string_lossy().into_owned();
                    entries.push(FileEntry { name, path, is_dir });
                }
                entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())));
                self.entries = entries;
                self.status = format!("{} items", self.entries.len());
            }
            Err(error) => {
                self.entries.clear();
                self.status = format!("Unable to read folder: {error}");
            }
        }
    }

    fn open_file(&mut self, path: PathBuf) {
        if path.is_dir() {
            self.current_dir = path;
            self.path_input = self.current_dir.display().to_string();
            self.read_directory();
            return;
        }
        match fs::read_to_string(&path) {
            Ok(contents) => {
                self.editor = text_editor::Content::with_text(&contents);
                self.active_file = Some(path.clone());
                self.dirty = false;
                self.status = format!("Opened {}", path.file_name().unwrap_or_default().to_string_lossy());
            }
            Err(error) => self.status = format!("Could not open file: {error}"),
        }
    }

    fn save_file(&mut self) {
        if let Some(path) = self.active_file.clone() {
            self.write_file(path);
        } else {
            self.save_file_as();
        }
    }

    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new().set_directory(&self.current_dir).set_file_name("untitled.txt").save_file() {
            self.write_file(path);
        }
    }

    fn write_file(&mut self, path: PathBuf) {
        match fs::write(&path, self.editor.text()) {
            Ok(()) => {
                self.active_file = Some(path.clone());
                self.dirty = false;
                self.status = format!("Saved {}", path.file_name().unwrap_or_default().to_string_lossy());
                self.read_directory();
            }
            Err(error) => self.status = format!("Could not save file: {error}"),
        }
    }

    fn import_theme(&mut self) {
        let selected = rfd::FileDialog::new()
            .add_filter("File Studio theme", &["json"])
            .set_title("Import File Studio Theme")
            .pick_file();
        if let Some(path) = selected {
            match ThemeConfig::load(&path) {
                Ok(theme) => {
                    let name = theme.name.clone();
                    self.set_theme(theme);
                    self.status = format!("Theme loaded: {name}");
                }
                Err(error) => self.status = format!("Theme import failed: {error}"),
            }
        }
    }

    fn set_theme(&mut self, theme: ThemeConfig) {
        self.theme = theme;
        let status = format!("Theme: {}", self.theme.name);
        match save_theme(&self.theme) {
            Ok(()) => self.status = status,
            Err(error) => self.status = format!("{status} (could not save preference: {error})"),
        }
    }
}

fn theme_file_path() -> Option<PathBuf> {
    let base = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("LOCALAPPDATA").map(PathBuf::from))?;
    Some(base.join("FileStudio").join("theme.json"))
}

fn load_saved_theme() -> ThemeConfig {
    theme_file_path()
        .and_then(|path| ThemeConfig::load(&path).ok())
        .unwrap_or_default()
}

fn save_theme(theme: &ThemeConfig) -> Result<(), String> {
    let path = theme_file_path().ok_or_else(|| "APPDATA is unavailable".to_owned())?;
    let parent = path.parent().ok_or_else(|| "Invalid theme path".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let json = serde_json::to_vec_pretty(theme).map_err(|error| error.to_string())?;
    fs::write(path, json).map_err(|error| error.to_string())
}

fn panel_style(color: Color, border: Color) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(color)),
        border: Border { color: border, width: 1.0, radius: 0.0.into() },
        ..Default::default()
    }
}

fn view(app: &App) -> Element<'_, Message> {
    let c = &app.theme;
    let bg = ThemeConfig::color(&c.background);
    let surface = ThemeConfig::color(&c.surface);
    let border = ThemeConfig::color(&c.border);
    let fg = ThemeConfig::color(&c.foreground);
    let muted = ThemeConfig::color(&c.muted);
    let accent = ThemeConfig::color(&c.accent);

    let brand = row![
        container(text("F").size(17).color(Color::WHITE)).padding([5, 10]).style(panel_style(accent, accent)),
        column![
            text("FILE STUDIO").size(13).color(fg),
            text("WORKSPACE").size(9).color(muted)
        ].spacing(1),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    // Keep navigation visually centered between the app branding and save state.
    let titlebar_navigation = row![
        button(text("Save").size(11)).padding([7, 10]).on_press(Message::Save).style(button::secondary),
        button(text("Save As").size(11)).padding([7, 10]).on_press(Message::SaveAs).style(button::secondary),
        button(text("↑").size(14)).padding([7, 11]).on_press(Message::GoUp).style(button::secondary),
        text_input("Current directory", &app.path_input)
            .on_input(Message::PathChanged)
            .on_submit(Message::Navigate)
            .size(12)
            .padding([8, 11])
            .width(Fill),
        button(text("Go").size(11)).padding([8, 13]).on_press(Message::Navigate).style(button::secondary),
    ]
    .spacing(7)
    .align_y(Alignment::Center);

    let save_state = row![
        text("●").size(10).color(if app.dirty {
            ThemeConfig::color(&c.warning)
        } else {
            ThemeConfig::color(&c.success)
        }),
        text(if app.dirty { "Unsaved changes" } else { "All changes saved" })
            .size(11)
            .color(muted),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let titlebar = container(
        row![
            brand,
            container(titlebar_navigation)
                .width(Fill)
                .padding([0, 22]),
            save_state,
        ]
        .spacing(14)
        .align_y(Alignment::Center),
    )
    .padding([9, 16])
    .width(Fill)
    .style(panel_style(surface, border));

    let activity = container(column![
        activity_button("▤", Message::ToggleExplorer, accent),
        activity_button("▣", Message::NewFile, muted),
        activity_button("◈", Message::ImportTheme, muted),
        iced::widget::Space::new().height(Fill),
        activity_button("⚙", Message::ImportTheme, muted),
    ].spacing(12).align_x(Alignment::Center).padding([14, 7]))
        .width(54).height(Fill).style(panel_style(surface, border));

    let explorer = if app.show_explorer {
        let header = row![text("EXPLORER").size(11).color(fg), iced::widget::Space::new().width(Fill),
            button(text("↻").size(15)).padding(4).on_press(Message::Refresh).style(button::text),
            button(text("＋").size(15)).padding(4).on_press(Message::NewFile).style(button::text),
        ].align_y(Alignment::Center);
        let mut files = column![].spacing(2);
        for entry in &app.entries {
            let icon = if entry.is_dir { "▸" } else { file_icon(&entry.name) };
            let label = row![text(icon).size(13).color(if entry.is_dir { accent } else { muted }), text(&entry.name).size(12).color(fg)].spacing(8).align_y(Alignment::Center);
            files = files.push(button(label).width(Fill).padding([6, 9]).on_press(Message::Open(entry.path.clone())).style(button::text));
        }
        container(column![header, text("FOLDERS & FILES").size(9).color(muted), scrollable(files).height(Fill)]
            .spacing(12).padding(12))
            .width(260).height(Fill).style(panel_style(surface, border))
    } else {
        container(iced::widget::Space::new()).width(0).height(Fill)
    };

    let filename = app.active_file.as_ref().and_then(|p| p.file_name()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "Untitled".into());
    let tab = row![
        container(row![text("▤").color(accent), text(if app.dirty { format!("{filename} ●") } else { filename.clone() }).size(12).color(fg)].spacing(9).align_y(Alignment::Center))
            .padding([10, 14]).style(panel_style(bg, accent)),
        iced::widget::Space::new().width(Fill),
        button(text("＋ New file").size(11)).padding([7, 10]).on_press(Message::NewFile).style(button::text),
    ].align_y(Alignment::Center);

    let breadcrumb = row![text("workspace").size(11).color(muted), text("›").color(muted), text(filename).size(11).color(fg),
        iced::widget::Space::new().width(Fill), text("UTF-8").size(10).color(muted), text("·").color(muted), text("Plain Text").size(10).color(muted)]
        .spacing(8).align_y(Alignment::Center);

    let editor = text_editor(&app.editor)
        .on_action(Message::EditorAction)
        .height(Fill)
        .padding(18)
        .font(iced::Font::MONOSPACE)
        .size(14);

    let editor_area = container(column![tab, container(breadcrumb).padding([8, 18]).style(panel_style(bg, border)), editor]
        .spacing(0).height(Fill))
        .width(Fill).height(Fill).style(panel_style(bg, border));

    let center = row![activity, explorer, editor_area].spacing(0).height(Fill);

    let statusbar = container(row![
        text("◆").size(12).color(Color::WHITE), text("File Studio").size(12).color(Color::WHITE),
        text(&app.status).size(12).color(Color::WHITE),
        iced::widget::Space::new().width(Fill),
        text(app.current_dir.display().to_string()).size(12).color(Color::WHITE),
        text(format!("v{}", env!("CARGO_PKG_VERSION"))).size(12).color(Color::WHITE),
    ].spacing(14).align_y(Alignment::Center)).padding([6, 12]).style(panel_style(accent, accent));

    let theme_menu = row![
        text("THEME").size(9).color(muted),
        button(text("Dark+").size(10)).padding([5, 9]).on_press(Message::UseVsCodeTheme).style(button::text),
        button(text("Monokai").size(10)).padding([5, 9]).on_press(Message::UseMonokai).style(button::text),
        button(text("High contrast").size(10)).padding([5, 9]).on_press(Message::UseHighContrast).style(button::text),
        button(text("＋ Import JSON").size(10)).padding([5, 9]).on_press(Message::ImportTheme).style(button::secondary),
    ].spacing(7).align_y(Alignment::Center);

    let footer = container(row![theme_menu, iced::widget::Space::new().width(Fill), text("CUSTOMIZABLE WORKSPACE").size(9).color(muted)]
        .align_y(Alignment::Center)).padding([5, 12]).style(panel_style(surface, border));

    container(column![titlebar, center, footer, statusbar].spacing(0).height(Fill))
        .width(Fill).height(Fill).style(panel_style(bg, border)).into()
}

fn activity_button<'a>(label: &'a str, message: Message, color: Color) -> Element<'a, Message> {
    button(text(label).size(21).color(color))
        .padding(5)
        .on_press(message)
        .style(button::text)
        .into()
}

fn file_icon(name: &str) -> &'static str {
    match name.rsplit('.').next().unwrap_or("").to_ascii_lowercase().as_str() {
        "rs" => "⚙", "toml" | "json" | "yaml" | "yml" => "◇", "md" | "txt" => "≡",
        "png" | "jpg" | "jpeg" | "gif" | "webp" => "▧", "exe" | "dll" => "▣", _ => "·",
    }
}
