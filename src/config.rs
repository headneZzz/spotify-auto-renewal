use config::{Config, File};

pub fn load_config() -> String {
    let mut config = Config::default();
    config.merge(File::with_name("config")).unwrap();

    config.get_string("app.cron").unwrap()
}