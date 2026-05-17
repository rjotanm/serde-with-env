use serde_with_env::serde_with_env;
use serde_json::json;

fn username_convert(val: String) -> Result<String, String> {
    match val.to_lowercase().as_str() {
        "postgres" => Err("postgres user not allowed".to_string()),
        _ => Ok(val),
    }
}

#[serde_with_env]
#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct PostgresConfig {
    #[serde(default = "PostgresConfig::scheme_default")]
    scheme: String,
    #[with_env(or = "POSTGRES_HOST", default = "localhost")]
    host: String,
    #[with_env(or = "POSTGRES_PORT")]
    #[with_env(default = 5432u16)]
    port: u16,
    #[with_env(or = "POSTGRES_USERNAME", convert = "username_convert")]
    user: String,
    #[with_env(only = "POSTGRES_PASSWORD")]
    password: String,
    #[with_env(or = "POSTGRES_DATABASE")]
    database: String,
    pub pool_size: usize,
}
impl PostgresConfig {
    fn scheme_default() -> String {
        String::from("postgres")
    }
}


fn main() {
    unsafe {
        std::env::set_var("POSTGRES_USERNAME", "username");
        std::env::set_var("POSTGRES_PASSWORD", "passwd");
        std::env::set_var("POSTGRES_DATABASE", "test_db");
    }
    let result = serde_json::from_str::<PostgresConfig>(&json!({
        "pool_size": 10
    }).to_string()).unwrap();

    assert_eq!(result, PostgresConfig {
        scheme: "postgres".to_string(),
        host: "localhost".to_string(),
        port: 5432,
        user: "username".to_string(),
        password: "passwd".to_string(),
        database: "test_db".to_string(),
        pool_size: 10,
    });

    println!("{:?}", result);
}