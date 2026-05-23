use std::path::PathBuf;

use directories::BaseDirs;

pub enum StorageType {
    JSON,
    #[cfg(feature = "sqlite")]
    Sqlite,
    // Remote
}

pub struct AppConfig {
    /// Maps to app minor version...?
    config_version: String,
    pub local_storage_type: StorageType,
    pub local_storage_path: PathBuf, // other than default
    pub auto_due_today: bool,        // do we always add "today" as the due date?
    pub preserve_history: bool,
    pub task_load_warning: Option<usize>, // number of tasks before an "overbooked" warning triggers
}

impl Default for AppConfig {
    fn default() -> Self {
        if let Some(base_dir) = BaseDirs::new() {
            Self {
                config_version: "0.4".into(),
                local_storage_type: StorageType::JSON,
                local_storage_path: base_dir.home_dir().join(".shittd.json"),
                auto_due_today: false,
                preserve_history: false,
                task_load_warning: None,
            }
        } else {
            // TODO warn that the base_dir could not be found, defaulting to current directory
            Self {
                config_version: "0.4".into(),
                local_storage_type: StorageType::JSON,
                local_storage_path: PathBuf::from(".shittd.json"),
                auto_due_today: false,
                preserve_history: false,
                task_load_warning: None,
            }
        }
    }
}

impl AppConfig {
    /// Generates a config file at .config/shittd/shittd.toml
    fn init() -> () {}

    fn get() -> Self {
        if let Some(base_dir) = BaseDirs::new() {
            let app_config_path = base_dir.config_dir().join("shittd/shittd.toml");
            if app_config_path.exists() {
                todo!()
                // also check config_version
            };
            todo!()
        } else {
            Self::default()
        }
    }
}
