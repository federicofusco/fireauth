pub mod api;
mod error;

pub use error::Error;

#[derive(Clone, Debug)]
pub struct FireAuth {
    pub api_key: String, // web api key
}

impl FireAuth {
    /// A constructor for the FireAuth struct
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}
