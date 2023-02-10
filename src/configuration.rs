#[derive(serde::Deserialize)]
pub struct Settings {
    pub animes: Vec<String>,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine current directory.");
    let config_directory = base_path.join("configuration");

    settings.merge(
        config::File::from(
            config_directory.join("anime")
        ).required(true)
   )?;

   settings.try_into()
}