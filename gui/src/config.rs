use std::path::PathBuf;

pub const APP_ID: Option<&str> = option_env!("APP_ID");
pub const RESOURCES_FILE: Option<&str> = option_env!("RESOURCES_FILE");

pub fn app_id() -> &'static str {
    APP_ID.expect("APP_ID env var not set at compile time")
}

pub fn resources_file() -> PathBuf {
    let resources_file = RESOURCES_FILE.expect("RESOURCES_FILE env var not set at compile time");

    if let Some(snap) = std::env::var_os("SNAP")
        && resources_file.starts_with('/')
    {
        return PathBuf::from(snap).join(resources_file.trim_start_matches('/'));
    }

    PathBuf::from(resources_file)
}
