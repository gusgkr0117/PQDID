use core::fmt;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ApiResult {
    pub result: bool,
    pub value: serde_json::Value,
    pub err_msg: Option<&'static str>,
}

impl ApiResult {
    pub fn ok<T: Serialize>(value: Option<T>) -> Self {
        let json_value = serde_json::to_value(value).unwrap();
        ApiResult {
            result: true,
            value: json_value,
            err_msg: None,
        }
    }

    pub fn err(err_msg: &'static str) -> Self {
        ApiResult {
            result: false,
            value: serde_json::Value::Null,
            err_msg: Some(err_msg),
        }
    }
}

impl fmt::Display for ApiResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
