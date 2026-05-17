#[cfg(test)]
mod tests_over {
    use serde_with_env::serde_with_env;

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleRewrite {
        #[with_env(over = "OPT")]
        _field: String,
    }

    #[test]
    fn simple_rewrite() {
        unsafe { std::env::set_var("OPT", "test") }

        let result = serde_json::from_str::<SimpleRewrite>("{\"_field\": \"1\"}").unwrap();
        assert_eq!(
            result,
            SimpleRewrite {
                _field: String::from("test")
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleError {
        #[with_env(over = "OPT_NOT_SET")]
        _field: String,
    }

    #[test]
    fn simple_err() {
        let result = serde_json::from_str::<SimpleError>("{}");
        assert!(result.is_err());
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct Simple {
        #[with_env(over = "OPT")]
        _field: String,
    }

    #[test]
    fn simple() {
        unsafe { std::env::set_var("OPT", "test") }

        let result = serde_json::from_str::<Simple>("{}").unwrap();
        assert_eq!(
            result,
            Simple {
                _field: String::from("test")
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleOptional {
        #[with_env(over = "OPT_OPTIONAL_SET")]
        _field: Option<String>,
    }

    #[test]
    fn simple_optional() {
        unsafe { std::env::set_var("OPT_OPTIONAL_SET", "test2") }

        let result = serde_json::from_str::<SimpleOptional>("{}").unwrap();
        assert_eq!(
            result,
            SimpleOptional {
                _field: Some(String::from("test2"))
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleOptionalNotSet {
        #[with_env(over = "OPT_NOT_SET")]
        _field: Option<String>,
    }

    #[test]
    fn simple_optional_not_set() {
        let result = serde_json::from_str::<SimpleOptionalNotSet>("{}").unwrap();
        assert_eq!(result, SimpleOptionalNotSet { _field: None });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleInt {
        #[with_env(over = "OPT_INT")]
        _field: u32,
    }

    #[test]
    fn simple_int() {
        unsafe { std::env::set_var("OPT_INT", "10") }

        let result = serde_json::from_str::<SimpleInt>("{}").unwrap();
        assert_eq!(result, SimpleInt { _field: 10 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimplBool {
        #[with_env(over = "OPT_BOOL")]
        _field: bool,
    }

    #[test]
    fn simple_bool() {
        unsafe { std::env::set_var("OPT_BOOL", "true") }

        let result = serde_json::from_str::<SimplBool>("{}").unwrap();
        assert_eq!(result, SimplBool { _field: true });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefault {
        #[with_env(over = "OPT_DEFAULT", default = "default_val")]
        _field: String,
    }

    #[test]
    fn simple_default() {
        let result = serde_json::from_str::<SimpleDefault>("{}").unwrap();
        assert_eq!(
            result,
            SimpleDefault {
                _field: String::from("default_val")
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultInt {
        #[with_env(over = "OPT_DEFAULT", default = 16i32)]
        _field: i32,
    }

    #[test]
    fn simple_default_int() {
        let result = serde_json::from_str::<SimpleDefaultInt>("{}").unwrap();
        assert_eq!(result, SimpleDefaultInt { _field: 16 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultOptional {
        #[with_env(over = "OPT_DEFAULT", default = "default_val")]
        _field: Option<String>,
    }

    #[test]
    fn simple_optional_default() {
        let result = serde_json::from_str::<SimpleDefaultOptional>("{}").unwrap();
        assert_eq!(
            result,
            SimpleDefaultOptional {
                _field: Some(String::from("default_val"))
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultIntOptional {
        #[with_env(over = "OPT_DEFAULT", default = 16i32)]
        _field: Option<i32>,
    }

    #[test]
    fn simple_optional_default_int() {
        let result = serde_json::from_str::<SimpleDefaultIntOptional>("{}").unwrap();
        assert_eq!(result, SimpleDefaultIntOptional { _field: Some(16) });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultIntOptionalSet {
        #[with_env(over = "OPT_DEFAULT_INT_SET", default = 16i32)]
        _field: Option<i32>,
    }

    #[test]
    fn simple_optional_default_int_set() {
        unsafe { std::env::set_var("OPT_DEFAULT_INT_SET", "32") }

        let result = serde_json::from_str::<SimpleDefaultIntOptionalSet>("{}").unwrap();
        assert_eq!(result, SimpleDefaultIntOptionalSet { _field: Some(32) });
    }

    fn simple_convert_fn(val: String) -> Result<i32, String> {
        Ok(val.len() as i32)
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvert {
        #[with_env(over = "OPT_CONVERT", convert = "simple_convert_fn")]
        _field: i32,
    }

    #[test]
    fn simple_convert() {
        unsafe { std::env::set_var("OPT_CONVERT", "test") }

        let result = serde_json::from_str::<SimpleConvert>("{}").unwrap();
        assert_eq!(result, SimpleConvert { _field: 4 });
    }

    fn simple_convert_optional_fn(val: String) -> Result<Option<i32>, String> {
        Ok(Some(val.len() as i32))
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptional {
        #[with_env(over = "OPT_CONVERT", convert = "simple_convert_optional_fn")]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional() {
        unsafe { std::env::set_var("OPT_CONVERT", "test") }

        let result = serde_json::from_str::<SimpleConvertOptional>("{}").unwrap();
        assert_eq!(result, SimpleConvertOptional { _field: Some(4) });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptionalNotSet {
        #[with_env(over = "OPT_CONVERT_NOT_SET", convert = "simple_convert_optional_fn")]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional_not_set() {
        let result = serde_json::from_str::<SimpleConvertOptionalNotSet>("{}").unwrap();
        assert_eq!(result, SimpleConvertOptionalNotSet { _field: None });
    }

    fn simple_convert_default_fn(val: String) -> Result<i32, String> {
        Ok(val.len() as i32)
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertDefault {
        #[with_env(
            over = "OPT_CONVERT_DEFAULT",
            default = 12,
            convert = "simple_convert_default_fn"
        )]
        _field: i32,
    }

    #[test]
    fn simple_convert_default() {
        unsafe { std::env::set_var("OPT_CONVERT_DEFAULT", "test") }

        let result = serde_json::from_str::<SimpleConvertDefault>("{}").unwrap();
        assert_eq!(result, SimpleConvertDefault { _field: 4 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertDefaultNotSet {
        #[with_env(
            over = "OPT_CONVERT_DEFAULT_NOT_SET",
            default = 12,
            convert = "simple_convert_default_fn"
        )]
        _field: i32,
    }

    #[test]
    fn simple_convert_default_not_set() {
        let result = serde_json::from_str::<SimpleConvertDefaultNotSet>("{}").unwrap();
        assert_eq!(result, SimpleConvertDefaultNotSet { _field: 12 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptionalDefault {
        #[with_env(
            over = "OPT_CONVERT_OPTIONAL_DEFAULT",
            default = 12,
            convert = "simple_convert_optional_fn"
        )]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional_default() {
        unsafe { std::env::set_var("OPT_CONVERT_OPTIONAL_DEFAULT", "test") }

        let result = serde_json::from_str::<SimpleConvertOptionalDefault>("{}").unwrap();
        assert_eq!(result, SimpleConvertOptionalDefault { _field: Some(4) });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptionalDefaultNotSet {
        #[with_env(
            over = "OPT_CONVERT_NOT_SET",
            default = 12,
            convert = "simple_convert_optional_fn"
        )]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional_default_not_set() {
        let result = serde_json::from_str::<SimpleConvertOptionalDefaultNotSet>("{}").unwrap();
        assert_eq!(
            result,
            SimpleConvertOptionalDefaultNotSet { _field: Some(12) }
        );
    }

    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexInner {
        val: String,
    }

    fn complex_inner_convert(val: String) -> Result<ComplexInner, String> {
        Ok(ComplexInner { val })
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvert {
        #[with_env(over = "OPT_COMPLEX_CONVERT", convert = "complex_inner_convert")]
        _field: ComplexInner,
    }

    #[test]
    fn complex_convert() {
        unsafe { std::env::set_var("OPT_COMPLEX_CONVERT", "test") }

        let result = serde_json::from_str::<ComplexConvert>("{}").unwrap();
        assert_eq!(
            result,
            ComplexConvert {
                _field: ComplexInner { val: "test".into() }
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvertNotSet {
        #[with_env(
            over = "OPT_COMPLEX_CONVERT_NOT_SET",
            convert = "complex_inner_convert"
        )]
        _field: ComplexInner,
    }

    #[test]
    fn complex_convert_not_set() {
        let result = serde_json::from_str::<ComplexConvertNotSet>("{}");
        assert!(result.is_err());
    }

    fn complex_inner_convert_optional(val: String) -> Result<Option<ComplexInner>, String> {
        Ok(Some(ComplexInner { val }))
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvertOption {
        #[with_env(
            over = "OPT_COMPLEX_CONVERT",
            convert = "complex_inner_convert_optional"
        )]
        _field: Option<ComplexInner>,
    }

    #[test]
    fn complex_convert_option() {
        unsafe { std::env::set_var("OPT_COMPLEX_CONVERT", "test") }

        let result = serde_json::from_str::<ComplexConvertOption>("{}").unwrap();
        assert_eq!(
            result,
            ComplexConvertOption {
                _field: Some(ComplexInner { val: "test".into() })
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvertOptionNotSet {
        #[with_env(
            over = "OPT_COMPLEX_CONVERT_NOT_SET",
            convert = "complex_inner_convert_optional"
        )]
        _field: Option<ComplexInner>,
    }

    #[test]
    fn complex_convert_option_not_set() {
        let result = serde_json::from_str::<ComplexConvertOptionNotSet>("{}").unwrap();
        assert_eq!(result, ComplexConvertOptionNotSet { _field: None });
    }
}

#[cfg(test)]
mod tests_or {
    use serde_with_env::serde_with_env;

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleNonRewrite {
        #[with_env(or = "OPT")]
        _field: String,
    }

    #[test]
    fn simple_rewrite() {
        unsafe { std::env::set_var("OPT", "test") }

        let result = serde_json::from_str::<SimpleNonRewrite>("{\"_field\": \"1\"}").unwrap();
        assert_eq!(
            result,
            SimpleNonRewrite {
                _field: String::from("1")
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleError {
        #[with_env(or = "OPT_NOT_SET")]
        _field: String,
    }

    #[test]
    fn simple_err() {
        let result = serde_json::from_str::<SimpleError>("{}");
        assert!(result.is_err());
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct Simple {
        #[with_env(or = "OPT")]
        _field: String,
    }

    #[test]
    fn simple() {
        unsafe { std::env::set_var("OPT", "test") }

        let result = serde_json::from_str::<Simple>("{}").unwrap();
        assert_eq!(
            result,
            Simple {
                _field: String::from("test")
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleOptional {
        #[with_env(or = "OPT_OPTIONAL_SET")]
        _field: Option<String>,
    }

    #[test]
    fn simple_optional() {
        unsafe { std::env::set_var("OPT_OPTIONAL_SET", "test2") }

        let result = serde_json::from_str::<SimpleOptional>("{}").unwrap();
        assert_eq!(
            result,
            SimpleOptional {
                _field: Some(String::from("test2"))
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleOptionalNotSet {
        #[with_env(or = "OPT_NOT_SET")]
        _field: Option<String>,
    }

    #[test]
    fn simple_optional_not_set() {
        let result = serde_json::from_str::<SimpleOptionalNotSet>("{}").unwrap();
        assert_eq!(result, SimpleOptionalNotSet { _field: None });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleInt {
        #[with_env(or = "OPT_INT")]
        _field: u32,
    }

    #[test]
    fn simple_int() {
        unsafe { std::env::set_var("OPT_INT", "10") }

        let result = serde_json::from_str::<SimpleInt>("{}").unwrap();
        assert_eq!(result, SimpleInt { _field: 10 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimplBool {
        #[with_env(or = "OPT_BOOL")]
        _field: bool,
    }

    #[test]
    fn simple_bool() {
        unsafe { std::env::set_var("OPT_BOOL", "true") }

        let result = serde_json::from_str::<SimplBool>("{}").unwrap();
        assert_eq!(result, SimplBool { _field: true });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefault {
        #[with_env(or = "OPT_DEFAULT", default = "default_val")]
        _field: String,
    }

    #[test]
    fn simple_default() {
        let result = serde_json::from_str::<SimpleDefault>("{}").unwrap();
        assert_eq!(
            result,
            SimpleDefault {
                _field: String::from("default_val")
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultInt {
        #[with_env(or = "OPT_DEFAULT", default = 16i32)]
        _field: i32,
    }

    #[test]
    fn simple_default_int() {
        let result = serde_json::from_str::<SimpleDefaultInt>("{}").unwrap();
        assert_eq!(result, SimpleDefaultInt { _field: 16 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultOptional {
        #[with_env(or = "OPT_DEFAULT", default = "default_val")]
        _field: Option<String>,
    }

    #[test]
    fn simple_optional_default() {
        let result = serde_json::from_str::<SimpleDefaultOptional>("{}").unwrap();
        assert_eq!(
            result,
            SimpleDefaultOptional {
                _field: Some(String::from("default_val"))
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultIntOptional {
        #[with_env(or = "OPT_DEFAULT", default = 16i32)]
        _field: Option<i32>,
    }

    #[test]
    fn simple_optional_default_int() {
        let result = serde_json::from_str::<SimpleDefaultIntOptional>("{}").unwrap();
        assert_eq!(result, SimpleDefaultIntOptional { _field: Some(16) });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultIntOptionalSet {
        #[with_env(or = "OPT_DEFAULT_INT_SET", default = 16i32)]
        _field: Option<i32>,
    }

    #[test]
    fn simple_optional_default_int_set() {
        unsafe { std::env::set_var("OPT_DEFAULT_INT_SET", "32") }

        let result = serde_json::from_str::<SimpleDefaultIntOptionalSet>("{}").unwrap();
        assert_eq!(result, SimpleDefaultIntOptionalSet { _field: Some(32) });
    }

    fn simple_convert_fn(val: String) -> Result<i32, String> {
        Ok(val.len() as i32)
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvert {
        #[with_env(or = "OPT_CONVERT", convert = "simple_convert_fn")]
        _field: i32,
    }

    #[test]
    fn simple_convert() {
        unsafe { std::env::set_var("OPT_CONVERT", "test") }

        let result = serde_json::from_str::<SimpleConvert>("{}").unwrap();
        assert_eq!(result, SimpleConvert { _field: 4 });
    }

    fn simple_convert_optional_fn(val: String) -> Result<Option<i32>, String> {
        Ok(Some(val.len() as i32))
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptional {
        #[with_env(or = "OPT_CONVERT", convert = "simple_convert_optional_fn")]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional() {
        unsafe { std::env::set_var("OPT_CONVERT", "test") }

        let result = serde_json::from_str::<SimpleConvertOptional>("{}").unwrap();
        assert_eq!(result, SimpleConvertOptional { _field: Some(4) });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptionalNotSet {
        #[with_env(or = "OPT_CONVERT_NOT_SET", convert = "simple_convert_optional_fn")]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional_not_set() {
        let result = serde_json::from_str::<SimpleConvertOptionalNotSet>("{}").unwrap();
        assert_eq!(result, SimpleConvertOptionalNotSet { _field: None });
    }

    fn simple_convert_default_fn(val: String) -> Result<i32, String> {
        Ok(val.len() as i32)
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertDefault {
        #[with_env(
            or = "OPT_CONVERT_DEFAULT",
            default = 12,
            convert = "simple_convert_default_fn"
        )]
        _field: i32,
    }

    #[test]
    fn simple_convert_default() {
        unsafe { std::env::set_var("OPT_CONVERT_DEFAULT", "test") }

        let result = serde_json::from_str::<SimpleConvertDefault>("{}").unwrap();
        assert_eq!(result, SimpleConvertDefault { _field: 4 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertDefaultNotSet {
        #[with_env(
            or = "OPT_CONVERT_DEFAULT_NOT_SET",
            default = 12,
            convert = "simple_convert_default_fn"
        )]
        _field: i32,
    }

    #[test]
    fn simple_convert_default_not_set() {
        let result = serde_json::from_str::<SimpleConvertDefaultNotSet>("{}").unwrap();
        assert_eq!(result, SimpleConvertDefaultNotSet { _field: 12 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptionalDefault {
        #[with_env(
            or = "OPT_CONVERT_OPTIONAL_DEFAULT",
            default = 12,
            convert = "simple_convert_optional_fn"
        )]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional_default() {
        unsafe { std::env::set_var("OPT_CONVERT_OPTIONAL_DEFAULT", "test") }

        let result = serde_json::from_str::<SimpleConvertOptionalDefault>("{}").unwrap();
        assert_eq!(result, SimpleConvertOptionalDefault { _field: Some(4) });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptionalDefaultNotSet {
        #[with_env(
            or = "OPT_CONVERT_NOT_SET",
            default = 12,
            convert = "simple_convert_optional_fn"
        )]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional_default_not_set() {
        let result = serde_json::from_str::<SimpleConvertOptionalDefaultNotSet>("{}").unwrap();
        assert_eq!(
            result,
            SimpleConvertOptionalDefaultNotSet { _field: Some(12) }
        );
    }

    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexInner {
        val: String,
    }

    fn complex_inner_convert(val: String) -> Result<ComplexInner, String> {
        Ok(ComplexInner { val })
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvert {
        #[with_env(or = "OPT_COMPLEX_CONVERT", convert = "complex_inner_convert")]
        _field: ComplexInner,
    }

    #[test]
    fn complex_convert() {
        unsafe { std::env::set_var("OPT_COMPLEX_CONVERT", "test") }

        let result = serde_json::from_str::<ComplexConvert>("{}").unwrap();
        assert_eq!(
            result,
            ComplexConvert {
                _field: ComplexInner { val: "test".into() }
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvertNotSet {
        #[with_env(or = "OPT_COMPLEX_CONVERT_NOT_SET", convert = "complex_inner_convert")]
        _field: ComplexInner,
    }

    #[test]
    fn complex_convert_not_set() {
        let result = serde_json::from_str::<ComplexConvertNotSet>("{}");
        assert!(result.is_err());
    }

    fn complex_inner_convert_optional(val: String) -> Result<Option<ComplexInner>, String> {
        Ok(Some(ComplexInner { val }))
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvertOption {
        #[with_env(or = "OPT_COMPLEX_CONVERT", convert = "complex_inner_convert_optional")]
        _field: Option<ComplexInner>,
    }

    #[test]
    fn complex_convert_option() {
        unsafe { std::env::set_var("OPT_COMPLEX_CONVERT", "test") }

        let result = serde_json::from_str::<ComplexConvertOption>("{}").unwrap();
        assert_eq!(
            result,
            ComplexConvertOption {
                _field: Some(ComplexInner { val: "test".into() })
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvertOptionNotSet {
        #[with_env(
            or = "OPT_COMPLEX_CONVERT_NOT_SET",
            convert = "complex_inner_convert_optional"
        )]
        _field: Option<ComplexInner>,
    }

    #[test]
    fn complex_convert_option_not_set() {
        let result = serde_json::from_str::<ComplexConvertOptionNotSet>("{}").unwrap();
        assert_eq!(result, ComplexConvertOptionNotSet { _field: None });
    }
}

#[cfg(test)]
mod tests_only {
    use serde_with_env::serde_with_env;

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleRewrite {
        #[with_env(only = "OPT_REWRITE")]
        _field: String,
    }

    #[test]
    fn simple_rewrite() {
        let result = serde_json::from_str::<SimpleRewrite>("{\"_field\": \"1\"}");
        assert!(result.is_err());
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleError {
        #[with_env(only = "OPT_NOT_SET")]
        _field: String,
    }

    #[test]
    fn simple_err() {
        let result = serde_json::from_str::<SimpleError>("{}");
        assert!(result.is_err());
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct Simple {
        #[with_env(only = "OPT")]
        _field: String,
    }

    #[test]
    fn simple() {
        unsafe { std::env::set_var("OPT", "test") }

        let result = serde_json::from_str::<Simple>("{}").unwrap();
        assert_eq!(
            result,
            Simple {
                _field: String::from("test")
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleOptional {
        #[with_env(only = "OPT_OPTIONAL_SET")]
        _field: Option<String>,
    }

    #[test]
    fn simple_optional() {
        unsafe { std::env::set_var("OPT_OPTIONAL_SET", "test2") }

        let result = serde_json::from_str::<SimpleOptional>("{}").unwrap();
        assert_eq!(
            result,
            SimpleOptional {
                _field: Some(String::from("test2"))
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleOptionalNotSet {
        #[with_env(only = "OPT_NOT_SET")]
        _field: Option<String>,
    }

    #[test]
    fn simple_optional_not_set() {
        let result = serde_json::from_str::<SimpleOptionalNotSet>("{}").unwrap();
        assert_eq!(result, SimpleOptionalNotSet { _field: None });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleInt {
        #[with_env(only = "OPT_INT")]
        _field: u32,
    }

    #[test]
    fn simple_int() {
        unsafe { std::env::set_var("OPT_INT", "10") }

        let result = serde_json::from_str::<SimpleInt>("{}").unwrap();
        assert_eq!(result, SimpleInt { _field: 10 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimplBool {
        #[with_env(only = "OPT_BOOL")]
        _field: bool,
    }

    #[test]
    fn simple_bool() {
        unsafe { std::env::set_var("OPT_BOOL", "true") }

        let result = serde_json::from_str::<SimplBool>("{}").unwrap();
        assert_eq!(result, SimplBool { _field: true });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefault {
        #[with_env(only = "OPT_DEFAULT", default = "default_val")]
        _field: String,
    }

    #[test]
    fn simple_default() {
        let result = serde_json::from_str::<SimpleDefault>("{}").unwrap();
        assert_eq!(
            result,
            SimpleDefault {
                _field: String::from("default_val")
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultInt {
        #[with_env(only = "OPT_DEFAULT", default = 16i32)]
        _field: i32,
    }

    #[test]
    fn simple_default_int() {
        let result = serde_json::from_str::<SimpleDefaultInt>("{}").unwrap();
        assert_eq!(result, SimpleDefaultInt { _field: 16 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultOptional {
        #[with_env(only = "OPT_DEFAULT", default = "default_val")]
        _field: Option<String>,
    }

    #[test]
    fn simple_optional_default() {
        let result = serde_json::from_str::<SimpleDefaultOptional>("{}").unwrap();
        assert_eq!(
            result,
            SimpleDefaultOptional {
                _field: Some(String::from("default_val"))
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultIntOptional {
        #[with_env(only = "OPT_DEFAULT", default = 16i32)]
        _field: Option<i32>,
    }

    #[test]
    fn simple_optional_default_int() {
        let result = serde_json::from_str::<SimpleDefaultIntOptional>("{}").unwrap();
        assert_eq!(result, SimpleDefaultIntOptional { _field: Some(16) });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleDefaultIntOptionalSet {
        #[with_env(only = "OPT_DEFAULT_INT_SET", default = 16i32)]
        _field: Option<i32>,
    }

    #[test]
    fn simple_optional_default_int_set() {
        unsafe { std::env::set_var("OPT_DEFAULT_INT_SET", "32") }

        let result = serde_json::from_str::<SimpleDefaultIntOptionalSet>("{}").unwrap();
        assert_eq!(result, SimpleDefaultIntOptionalSet { _field: Some(32) });
    }

    fn simple_convert_fn(val: String) -> Result<i32, String> {
        Ok(val.len() as i32)
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvert {
        #[with_env(only = "OPT_CONVERT", convert = "simple_convert_fn")]
        _field: i32,
    }

    #[test]
    fn simple_convert() {
        unsafe { std::env::set_var("OPT_CONVERT", "test") }

        let result = serde_json::from_str::<SimpleConvert>("{}").unwrap();
        assert_eq!(result, SimpleConvert { _field: 4 });
    }

    fn simple_convert_optional_fn(val: String) -> Result<Option<i32>, String> {
        Ok(Some(val.len() as i32))
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptional {
        #[with_env(only = "OPT_CONVERT", convert = "simple_convert_optional_fn")]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional() {
        unsafe { std::env::set_var("OPT_CONVERT", "test") }

        let result = serde_json::from_str::<SimpleConvertOptional>("{}").unwrap();
        assert_eq!(result, SimpleConvertOptional { _field: Some(4) });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptionalNotSet {
        #[with_env(only = "OPT_CONVERT_NOT_SET", convert = "simple_convert_optional_fn")]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional_not_set() {
        let result = serde_json::from_str::<SimpleConvertOptionalNotSet>("{}").unwrap();
        assert_eq!(result, SimpleConvertOptionalNotSet { _field: None });
    }

    fn simple_convert_default_fn(val: String) -> Result<i32, String> {
        Ok(val.len() as i32)
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertDefault {
        #[with_env(
            only = "OPT_CONVERT_DEFAULT",
            default = 12,
            convert = "simple_convert_default_fn"
        )]
        _field: i32,
    }

    #[test]
    fn simple_convert_default() {
        unsafe { std::env::set_var("OPT_CONVERT_DEFAULT", "test") }

        let result = serde_json::from_str::<SimpleConvertDefault>("{}").unwrap();
        assert_eq!(result, SimpleConvertDefault { _field: 4 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertDefaultNotSet {
        #[with_env(
            only = "OPT_CONVERT_DEFAULT_NOT_SET",
            default = 12,
            convert = "simple_convert_default_fn"
        )]
        _field: i32,
    }

    #[test]
    fn simple_convert_default_not_set() {
        let result = serde_json::from_str::<SimpleConvertDefaultNotSet>("{}").unwrap();
        assert_eq!(result, SimpleConvertDefaultNotSet { _field: 12 });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptionalDefault {
        #[with_env(
            only = "OPT_CONVERT_OPTIONAL_DEFAULT",
            default = 12,
            convert = "simple_convert_optional_fn"
        )]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional_default() {
        unsafe { std::env::set_var("OPT_CONVERT_OPTIONAL_DEFAULT", "test") }

        let result = serde_json::from_str::<SimpleConvertOptionalDefault>("{}").unwrap();
        assert_eq!(result, SimpleConvertOptionalDefault { _field: Some(4) });
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct SimpleConvertOptionalDefaultNotSet {
        #[with_env(
            only = "OPT_CONVERT_NOT_SET",
            default = 12,
            convert = "simple_convert_optional_fn"
        )]
        _field: Option<i32>,
    }

    #[test]
    fn simple_convert_optional_default_not_set() {
        let result = serde_json::from_str::<SimpleConvertOptionalDefaultNotSet>("{}").unwrap();
        assert_eq!(
            result,
            SimpleConvertOptionalDefaultNotSet { _field: Some(12) }
        );
    }

    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexInner {
        val: String,
    }

    fn complex_inner_convert(val: String) -> Result<ComplexInner, String> {
        Ok(ComplexInner { val })
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvert {
        #[with_env(only = "OPT_COMPLEX_CONVERT", convert = "complex_inner_convert")]
        _field: ComplexInner,
    }

    #[test]
    fn complex_convert() {
        unsafe { std::env::set_var("OPT_COMPLEX_CONVERT", "test") }

        let result = serde_json::from_str::<ComplexConvert>("{}").unwrap();
        assert_eq!(
            result,
            ComplexConvert {
                _field: ComplexInner { val: "test".into() }
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvertNotSet {
        #[with_env(
            only = "OPT_COMPLEX_CONVERT_NOT_SET",
            convert = "complex_inner_convert"
        )]
        _field: ComplexInner,
    }

    #[test]
    fn complex_convert_not_set() {
        let result = serde_json::from_str::<ComplexConvertNotSet>("{}");
        assert!(result.is_err());
    }

    fn complex_inner_convert_optional(val: String) -> Result<Option<ComplexInner>, String> {
        Ok(Some(ComplexInner { val }))
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvertOption {
        #[with_env(
            only = "OPT_COMPLEX_CONVERT",
            convert = "complex_inner_convert_optional"
        )]
        _field: Option<ComplexInner>,
    }

    #[test]
    fn complex_convert_option() {
        unsafe { std::env::set_var("OPT_COMPLEX_CONVERT", "test") }

        let result = serde_json::from_str::<ComplexConvertOption>("{}").unwrap();
        assert_eq!(
            result,
            ComplexConvertOption {
                _field: Some(ComplexInner { val: "test".into() })
            }
        );
    }

    #[serde_with_env]
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ComplexConvertOptionNotSet {
        #[with_env(
            only = "OPT_COMPLEX_CONVERT_NOT_SET",
            convert = "complex_inner_convert_optional"
        )]
        _field: Option<ComplexInner>,
    }

    #[test]
    fn complex_convert_option_not_set() {
        let result = serde_json::from_str::<ComplexConvertOptionNotSet>("{}").unwrap();
        assert_eq!(result, ComplexConvertOptionNotSet { _field: None });
    }
}
