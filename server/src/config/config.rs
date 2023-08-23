fn get_env_var(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| panic!("{} must be set", var_name))
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub app_host: String,
    pub app_port: String,
}

impl Settings {
    pub fn init() -> Settings {
        let app_host = get_env_var("APP_HOST");
        let app_port = get_env_var("APP_PORT");

        Settings { app_host, app_port }
    }
}
