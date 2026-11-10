//! The APK's game library (`libhorizon.so`). Android's NativeActivity loads it and calls
//! [`android_main`] on the activity's own thread; it starts the official game there, as the
//! desktop executable's `main` does (`horizon-official`).
//!
//! Android has no command line: the launcher writes the game's arguments, one per line, to
//! `launch-args.txt` in the app's internal files folder before it starts the game activity.
#![cfg(target_os = "android")]

use horizon_game::AndroidApp;
use std::path::Path;

#[global_allocator]
static ALLOCATOR: horizon_game::Allocator = horizon_game::ALLOCATOR;

/// The mod API level this game serves, kept in the library for the launcher to read
/// without loading it; [`android_main`] reads it, so the linker keeps it.
static MOD_API_LEVEL: [u8; horizon_game::MOD_API_LEVEL.len()] = horizon_game::MOD_API_LEVEL;

/// The file the launcher leaves the game's arguments in, inside the app's files folder.
const ARGS_FILE: &str = "launch-args.txt";

/// The game's arguments: one per line in [`ARGS_FILE`], blank lines skipped. Without the
/// file, the game starts in the app's files folder as its game and home folder.
fn launch_args(files: &Path) -> Vec<String> {
    match std::fs::read_to_string(files.join(ARGS_FILE)) {
        Ok(text) => text
            .lines()
            .map(str::trim_end)
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect(),
        Err(_) => {
            let dir = files.join("game").to_string_lossy().into_owned();
            let home = files.join("home").to_string_lossy().into_owned();
            vec!["--game-dir".into(), dir, "--home".into(), home]
        }
    }
}

/// NativeActivity's entry point (android-activity), on the activity's main native thread.
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    std::hint::black_box(&MOD_API_LEVEL);
    let files = app
        .internal_data_path()
        .unwrap_or_else(|| "/data/local/tmp/horizon".into());
    let args = launch_args(&files);
    // The game resolves relative paths against the working directory, as on the desktop.
    let _ = std::fs::create_dir_all(&files);
    let _ = std::env::set_current_dir(&files);
    horizon_official::register();
    horizon_game::main_android(app, args);
}
