# File Studio

A modern, lightweight Windows file explorer and text editor built with **Rust** and **Iced 0.14**, featuring a Visual Studio Code-inspired interface and customizable JSON themes.

## ✨ Features

### 📁 File Explorer

- Native directory browsing.
- Navigate between folders and subdirectories.
- Navigate to parent directories.
- Direct path navigation using the address bar.
- Refresh directory contents.
- Automatically sort directories before files.
- Alphabetical sorting (case-insensitive).
- Click-to-open files and folders.
- Scrollable file and folder listing.
- File-type-specific icons for supported extensions.
- Displays the current working directory.

### 📝 Text Editor

- Integrated text editor.
- Monospaced editor font.
- UTF-8 text file support.
- Open existing text files directly from the explorer.
- Create new files.
- Edit file contents.
- Save existing files.
- Save As functionality.
- Native Windows file dialogs.
- Automatic filename tracking.
- Unsaved changes detection.
- Visual indicators for modified and saved documents.
- Active document filename displayed in the editor tab.
- Plain-text editing environment.

### 🎨 Customizable Themes

- VS Code-inspired Dark+ theme.
- Monokai-inspired theme.
- High Contrast theme.
- Import custom themes using JSON files.
- Automatic theme validation.
- Instant theme switching.
- Persistent theme preferences.
- Automatically restore the previously selected theme on startup.
- Customizable workspace colors.

#### Custom Theme Support

Themes use a simple JSON configuration with the following customizable properties:

| Property      | Description                        |
| ------------- | ---------------------------------- |
| `name`        | Theme display name                 |
| `background`  | Main workspace background          |
| `surface`     | Sidebar and panel backgrounds      |
| `surface_alt` | Secondary surface and hover colors |
| `border`      | Panel borders and separators       |
| `foreground`  | Primary text color                 |
| `muted`       | Secondary text and metadata        |
| `accent`      | Primary accent color               |
| `selection`   | Selection color                    |
| `success`     | Success and saved-state color      |
| `warning`     | Warning and modified-state color   |
| `danger`      | Error and destructive-state color  |

All theme colors use standard six-digit hexadecimal RGB values (`#RRGGBB`).

### 🖥️ User Interface

- Modern VS Code-inspired workspace.
- Native desktop application interface.
- Dedicated activity bar.
- Collapsible file explorer sidebar.
- Integrated editor workspace.
- Editor tab with active filename.
- Breadcrumb navigation.
- Workspace address bar.
- Application branding and navigation area.
- Theme selection controls.
- Dedicated application status bar.
- Current directory information.
- File encoding indicator.
- Plain-text language indicator.
- Saved and unsaved document indicators.
- Dynamic status messages for file operations and errors.
- Resizable application window.

### ⚙️ Application

- Built with Rust.
- Powered by Iced 0.14.
- Rust 2024 edition.
- Native Windows desktop application.
- Lightweight dependency stack.
- Native file dialogs through `rfd`.
- JSON serialization and deserialization through Serde.
- Persistent theme configuration.
- Windows application configuration storage using `%APPDATA%`.
- Graceful handling of file and directory errors.

---

## 🛠️ Built With

| Technology                                     | Purpose                           |
| ---------------------------------------------- | --------------------------------- |
| [Rust](https://www.rust-lang.org/)             | Core application language         |
| [Iced](https://iced.rs/)                       | Native desktop GUI framework      |
| [Serde](https://serde.rs/)                     | Serialization and deserialization |
| [Serde JSON](https://github.com/serde-rs/json) | JSON theme configuration          |
| [RFD](https://github.com/PolyMeilex/rfd)       | Native file dialogs               |

## 🚀 Getting Started

### Requirements

- Windows
- Rust toolchain
- Cargo

### Installation

Clone the repository:

```bash
git clone https://github.com/Krimsinxx/file-studio.git
```

Navigate into the project directory:

```bash
cd file_studio
```

Run the application:

```bash
cargo run
```

### Build

Build an optimized release executable:

```bash
cargo build --release
```

The executable will be generated at:

```text
target/release/FileStudio.exe
```

---

## 📂 Project Structure

```text
file_studio/
├── src/
│   ├── main.rs
│   └── theme.rs
├── themes/
│   └── vscode-dark-plus.json
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
└── README.md
```

## 🗺️ Roadmap

Planned improvements and future development:

- [ ] Multiple editor tabs.
- [ ] Advanced file management operations.
- [ ] File creation and deletion directly from the explorer.
- [ ] Rename and move files and folders.
- [ ] Copy, cut, and paste support.
- [ ] Search files and folders.
- [ ] Find and replace within documents.
- [ ] Syntax highlighting.
- [ ] Language detection.
- [ ] Line numbers.
- [ ] Code folding.
- [ ] Improved editor customization.
- [ ] Theme creation and editing directly inside the application.
- [ ] Light themes.
- [ ] Theme import and export improvements.
- [ ] Keyboard shortcuts.
- [ ] Drag-and-drop file support.
- [ ] Recent files and directories.
- [ ] File operation progress indicators.
- [ ] Improved application settings.

---

## 📜 License

See the repository for licensing information.
