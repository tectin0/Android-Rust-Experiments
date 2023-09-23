#[cfg(target_os = "android")]
pub(crate) const STORAGE_PATH: &str = "/data/data/com.example.android_rust_experiments/files";

#[cfg(not(target_os = "android"))]
pub(crate) const STORAGE_PATH: &str = ".";
