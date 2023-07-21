pub mod api;
mod error;

pub use error::Error;

#[derive(Clone, Debug)]
pub enum FireAuth {
    Cloud { api_key: String }, // web api key
    Emulator(/*host*/ String),
}

impl FireAuth {
    /// Creates a client Firebase Auth
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::Cloud {
            api_key: api_key.into(),
        }
    }

    /// Connection to Firebase Emulator
    /// host: "locahost:9099"
    pub fn emulator(host: impl Into<String>) -> Self {
        Self::Emulator(host.into())
    }

    pub(crate) fn toolkit_url(&self, segment: &str) -> String {
        match self {
            FireAuth::Cloud { api_key } => {
                format!("https://identitytoolkit.googleapis.com/v1/{segment}?key={api_key}")
            }
            FireAuth::Emulator(host) => {
                format!("http://{host}/identitytoolkit.googleapis.com/v1/{segment}?key=123")
            }
        }
    }

    pub(crate) fn secure_token_url(&self, segment: &str) -> String {
        match self {
            FireAuth::Cloud { api_key } => {
                format!("https://securetoken.googleapis.com/v1/token?key={api_key}")
            }
            FireAuth::Emulator(host) => {
                format!("http://{host}/securetoken.googleapis.com/v1/{segment}?key=123")
            }
        }
    }
}
