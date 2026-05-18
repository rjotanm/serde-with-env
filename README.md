# serde-with-env

Provides a way to obtain values from the environment after any serde deserialization.

**Attention**: this is an alpha version, so it has not been thoroughly tested with all serde functionality.

### Usage

Apply `#[serde_with_env]` to a struct and use the field attribute `#[with_env(...)]`.

There are three variants for value processing:
- `or` - tries to get the value from the environment when it's not provided in the original data;
- `over` - tries to get the value from the environment; if not set, falls back to the original data;
- `only` - tries to get the value from the environment; the value from the original data is NOT used.

Additionally, there are two other options:
- `default` - a literal that can be used when the value is not provided by the original data or the environment;
- `convert` - a path to a method that validates input from the environment and can change its type.
  - `fn some_convert_func(val: String) -> Result</* TYPE OF FIELD */, String> { todo!() }`

`#[with_env(or\over\only = "ENV_VAR", default = "literal", convert = "fn path")]`

**Note**: be careful with serde's `default`, `with`, and other deserialization options on `with_env` fields, because they are called before the environment is consulted.

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

The `serde_with_env` macro simply manipulates the token input.

First, the macro creates a shadow copy of the struct with `Option` for the `with_env` fields.
**Note**: you can also use `Option<T>` fields with `with_env`.
```rust
#[derive(Debug, serde::Deserialize)]
pub struct __PostgresConfig {
    #[serde(default = "PostgresConfig::scheme_default")]
    scheme: String,
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    // note: password is not generated because 'only' option is used
    database: Option<String>,
    pub pool_size: usize,
}
```

Second, the macro sets the serde `try_from` attribute for the original struct.
```rust
#[derive(Debug, serde::Deserialize)]
#[serde(try_from = "__serde_with_env__PostgresConfig::__PostgresConfig")]
pub struct PostgresConfig {
    /* unchanged */
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