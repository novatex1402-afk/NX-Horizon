//! The Android activity the game runs in. The APK's native library (`horizon-android`)
//! hands it over before the game starts; the event loop and the touch controls read it.
use std::sync::OnceLock;
pub use winit::platform::android::activity::AndroidApp;

static APP: OnceLock<AndroidApp> = OnceLock::new();

/// Hands the game its activity; only the first call counts.
pub fn set_app(app: AndroidApp) {
    let _ = APP.set(app);
}

/// The activity, once [`set_app`] ran.
pub fn app() -> Option<AndroidApp> {
    APP.get().cloned()
}
