//! The game as the platform runs it, for an executable to start (docs/layers.md, "How a Horizon
//! crate plugs in"). The official executable (`horizon-official`, Horizon layer) registers its
//! hooks (`horizon_platform::hooks`) and then calls [`main`]; it never names an engine crate.
//!
//! An executable built on it:
//!
//! ```ignore
//! #[global_allocator]
//! static ALLOCATOR: horizon_game::Allocator = horizon_game::ALLOCATOR;
//! static MOD_API_LEVEL: [u8; horizon_game::MOD_API_LEVEL.len()] = horizon_game::MOD_API_LEVEL;
//!
//! fn main() {
//!     std::hint::black_box(&MOD_API_LEVEL);
//!     my_crate::register();
//!     horizon_game::main();
//! }
//! ```
//!
//! The platform's own parts register here, in [`main`], as they leave the engine (layer plan M6).
#![forbid(unsafe_code)]

/// The game's allocator (mimalloc, with its thread tags), for the executable's
/// `#[global_allocator]`.
pub use horizon_threads::{ALLOCATOR, Allocator};

/// The mod API level this game serves. The executable keeps it in the program, where the
/// launcher reads it without running it (`horizon_mods::api_level::MARK`): a `static` its
/// `main` reads, so the linker keeps it.
pub const MOD_API_LEVEL: [u8; horizon_mods::api_level::MARK.len()] = horizon_mods::api_level::MARK;

/// Runs the game with the hooks registered so far, and returns when it ends. Register hooks
/// (`horizon_platform::hooks::register`) before calling it.
pub fn main() {
    horizon_client::entry::main();
}

/// Android: runs the game in `app`, the activity the APK's native library was started in,
/// with `args` as its command line (Android has none). Register hooks first, as for [`main`].
#[cfg(target_os = "android")]
pub fn main_android(app: AndroidApp, args: Vec<String>) {
    horizon_client::game::android::set_app(app);
    horizon_client::entry::set_args(args);
    horizon_client::entry::main();
}

/// The activity handle [`main_android`] takes (android-activity's, as winit re-exports it).
#[cfg(target_os = "android")]
pub use horizon_client::game::android::AndroidApp;
