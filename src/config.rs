use std::env;
use std::collections::HashMap;


pub fn fetch_config (key: &str) -> String {

    let mut default_config = HashMap::new();
    default_config.insert("LISTEN_ADDR".to_string(), "127.0.0.1".to_string());
    default_config.insert("LISTEN_PORT".to_string(), "8088".to_string());

    match env::var(key) {
        Ok(val) => return val,
        Err(_) => return default_config.get(key).unwrap().to_string()
    };

}

