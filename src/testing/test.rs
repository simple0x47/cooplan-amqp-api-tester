use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::error::{Error, ErrorKind};

const REQUEST_HEADER: &str = "header";
const REQUEST_HEADER_TOKEN: &str = "token";

/// Test that contains a request and an expected response.
#[derive(Deserialize, Serialize, Clone)]
pub struct Test {
    name: String,
    request: Map<String, Value>,
    assert_script: String,
}

impl Test {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn request(&self) -> &Map<String, Value> {
        &self.request
    }

    /// Injects the token into the request's header.
    pub fn inject_token(&mut self, token: &str) -> Result<(), Error> {
        let header = match self.request.get_mut(REQUEST_HEADER) {
            Some(header) => match header.as_object_mut() {
                Some(header) => header,
                None => {
                    return Err(Error::new(
                        ErrorKind::InternalFailure,
                        format!("failed to get header as object"),
                    ));
                }
            },
            None => {
                return Err(Error::new(
                    ErrorKind::InternalFailure,
                    format!("failed to get header"),
                ));
            }
        };

        if header.contains_key(REQUEST_HEADER_TOKEN) {
            return Ok(());
        }

        header.insert(
            REQUEST_HEADER_TOKEN.to_string(),
            Value::String(token.to_string()),
        );

        Ok(())
    }

    pub fn assert_script(&self) -> &str {
        self.assert_script.as_str()
    }
}
