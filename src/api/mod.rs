mod sign_in;
mod sign_up;
mod token;
mod user;

pub use sign_in::Response as SignInResponse;
pub use sign_up::Response as SignUpResponse;
pub use token::RefreshIdToken;
pub use user::{ProviderUserInfo, SendOobCode, UpdateUser, User};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailResponse {
    error: FailResponseBody,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailResponseBody {
    // code: u16,
    message: String,
}
