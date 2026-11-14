fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let mut res = winres::WindowsResource::new();
    let version = env!("CARGO_PKG_VERSION");
    let version = format!("{}.0", version);

    // res.set_manifest_file("windows.manifest");

    res.set("ProductName", "File Studio");
    res.set("FileDescription", "File Explorer and Text Editor");
    res.set("CompanyName", "Krimsinx");
    res.set("LegalCopyright", "MIT License");

    res.set("InternalName", "FileStudio");
    res.set("OriginalFilename", "FileStudio.exe");

    res.set("FileVersion", &version);
    res.set("ProductVersion", &version);

    res.set("Comments", "A modern Windows file explorer and text editor.");

    res.compile().expect("Failed to compile Windows resources");
}