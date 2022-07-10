use config::Config;
use std::collections::HashMap;

/// Read a suppiled config file amd return the results as a HashMap
pub fn get_config(file: &str) -> HashMap<String, String> {
    let reader = Config::builder()
    .add_source(config::File::with_name(file))
    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    // .add_source(config::Environment::with_prefix("APP"))
    .build()
    .unwrap();

    reader.try_deserialize::<HashMap<String,String>>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_within_package() {
        let config_path = "../../libs/config-loader/test_config";
        let config = get_config(config_path);
        assert!(config.contains_key("test"));
        assert!(config.get("test").unwrap() == "test");
    }

    #[test]
    fn test_config_within_workspace() {
        let config_path = "../../bins/api-wrapper/config";
        let config = get_config(config_path);
        assert!(config.contains_key("AccountPositions"));
        assert!(config.get("AccountPositions").unwrap() == "/accounts/{}/positions");
    }
}
