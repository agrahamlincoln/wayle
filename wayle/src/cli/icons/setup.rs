use std::{fs, path::PathBuf};

use wayle_icons::IconRegistry;

use crate::cli::CliAction;

/// Bundled component SVGs live under `<icon base>/hicolor/scalable/actions`.
const ACTIONS_SUBDIR: &str = "hicolor/scalable/actions";

/// In-tree bundle location, used when running from a source checkout (e.g.
/// `cargo run`) where the icons have not been installed to a system prefix.
const SOURCE_RESOURCES_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../resources/icons/hicolor/scalable/actions"
);

/// Resolves the directory holding the bundled component icons.
///
/// Prefers an installed system bundle (`/usr/share/wayle/icons` plus any
/// `XDG_DATA_DIRS` entry, via [`IconRegistry::system_icon_paths`]) so packaged
/// installs work, falling back to the in-tree `resources/` dir for source
/// checkouts. Previously this resolved only the compile-time `CARGO_MANIFEST_DIR`
/// path, which never exists on a packaged install — `wayle icons setup` failed
/// for every non-source user.
fn resolve_source_dir() -> Option<PathBuf> {
    IconRegistry::system_icon_paths()
        .into_iter()
        .map(|base| base.join(ACTIONS_SUBDIR))
        .find(|path| path.is_dir())
        .or_else(|| {
            let source = PathBuf::from(SOURCE_RESOURCES_DIR);
            source.is_dir().then_some(source)
        })
}

/// Installs bundled icons from the resources directory.
///
/// # Errors
///
/// Returns error if no bundle can be located or a copy fails.
pub fn execute() -> CliAction {
    let source_dir = resolve_source_dir().ok_or_else(|| {
        "Bundled icons not found. Install the wayle package or run from a source checkout."
            .to_string()
    })?;
    let source_dir = source_dir.as_path();

    let registry = IconRegistry::new().map_err(|err| err.to_string())?;
    let dest_dir = registry.icons_dir();

    fs::create_dir_all(&dest_dir)
        .map_err(|err| format!("Failed to create icons directory: {err}"))?;

    let entries = fs::read_dir(source_dir)
        .map_err(|err| format!("Failed to read resources directory: {err}"))?;

    let mut count = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(filename) = path.file_name() else {
            continue;
        };
        if path.extension().is_some_and(|ext| ext == "svg") {
            let dest_path = dest_dir.join(filename);
            fs::copy(&path, &dest_path)
                .map_err(|err| format!("Failed to copy {}: {err}", path.display()))?;
            println!(
                "Installed: {}",
                filename.to_string_lossy().trim_end_matches(".svg")
            );
            count += 1;
        }
    }

    println!("\n{count} icons installed to {}", dest_dir.display());
    Ok(())
}
