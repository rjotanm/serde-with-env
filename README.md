# serde-with-env

This crate provide a way to get value from env after standard serde deserialization.

**attention**: this is alpha version, so not tested as well with some serde functionality.

### Usage

Apply `#[serde_with_env]` to struct.
And use field attribute `#[with_env(...)]`

We have three workaround variants for processing value:
- `or` - try value from env when it's not provided in original data; 
- `over` - try value from env and when not set, get from original data; 
- `only` - try value from env, value from original data NOT usage.

Besides, we have two another option:
- `default` - literal, that can be used, when value does not provide from original data or env;
- `convert` - path to method, than validate input from env and can change its type.
  - `fn some_convert_func(val: String) -> Result</* TYPE OF FIELD */, String> { todo!() }`

**note**: be carefully with serde default option, because it calls before trying from env. 

```rust
use serde_with_env::serde_with_env;
use serde_json::json;

fn username_convert(val: String) -> Result<String, String> {
    match val.to_lowercase().as_str() {
        "postgres" => Err("postgres user not allowed".to_string()),
        _ => Ok(val),
    }
}

#[serde_with_env]
#[derive(Debug, serde::Deserialize)]
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
}
```

### How it works?

`serde_with_env` macro manipulate with token input.

First, we crate shadow copy of struct, with Option over `with_env` fields.
```rust
#[derive(Debug, serde::Deserialize)]
pub struct __PostgresConfig {
    #[serde(default = "PostgresConfig::scheme_default")]
    scheme: String,
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    // note: password not generated, because only option choice
    database: Option<String>,
    pub pool_size: usize,
}
```

Second, we provide serde `try_from` attribute for original struct.
```rust
#[derive(Debug, serde::Deserialize)]
#[serde(try_from = "__serde_with_env__PostgresConfig::__PostgresConfig")]
pub struct PostgresConfig {
    /* without changes */
}
```

```rust 
impl TryFrom<__PostgresConfig> for PostgresConfig {
    type Error = String;
    fn try_from(v: __PostgresConfig) -> Result<Self, Self::Error> {
        let host = if let Some(v) = v.host {
            Ok(v)
        } else {
            match std::env::var("POSTGRES_HOST") {
                Ok(v) => {
                    v.parse()
                        .map_err(|err| { 
                          format!("Cant parse \"POSTGRES_HOST\" environment variable: {err}")
                        })
                }
                Err(err) => Ok("localhost".into()),
            }
        }?;
        /* other fields */ 
        Ok(Self {
            scheme: v.scheme,
            host,
            port,
            user,
            password,
            database,
            pool_size: v.pool_size,
        })
    }
}
```