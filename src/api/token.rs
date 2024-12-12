use super::FailResponse;
use crate::error::Error;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

impl crate::FireAuth {
    pub async fn refresh_id_token(&self, refresh_token: &str) -> Result<RefreshIdToken, Error> {
        let url = format!(
            "https://securetoken.googleapis.com/v1/token?key={}",
            self.api_key,
        );

        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&RefreshIdTokenPayload {
                grant_type: "refresh_token",
                refresh_token,
            })
            .send()
            .await?;

        if resp.status() != 200 {
            let error = resp.json::<FailResponse>().await?.error;
            return Err(Error::Token(error.message));
        }

        let body = resp.json::<RefreshIdToken>().await?;
        Ok(body)
    }

    pub async fn verify_id_token(&self, id_token: &str) -> Result<IdTokenClaims, Error> {
        // Gets the kid property of the token header
        let kid = decode_header(id_token)
            .map_err(|_| Error::Token("Malformed token header!".into()))?
            .kid
            .ok_or(Error::Token("Missing kid in token header!".into()))?;

        // Fetches the possible decoding keys
        let url = String::from ( "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com" );
        let client = reqwest::Client::new();
        let resp = client.get(url).send().await?;

        if resp.status() != 200 {
            // Cannot guarantee an error message from the response
            return Err(Error::API("Failed to fetch keys!".into()));
        }

        let body: std::collections::HashMap<String, String> =
            serde_json::from_str(&resp.text().await?)
                .map_err(|_| Error::API("Failed to parse keys!".into()))?;

        // Gets the key that will verify the ID token
        let decoding_key = body
            .get(&kid)
            .ok_or(Error::Token("No match decoding key!".into()))?;
        let decoding_key = &DecodingKey::from_rsa_pem(decoding_key.as_bytes())
            .map_err(|_| Error::Token("Failed to parse decoding key!".into()))?;

        // Decodes and validates the ID token.
        Ok(
            decode::<IdTokenClaims>(id_token, decoding_key, &Validation::new(Algorithm::RS256))
                .map_err(|_| Error::Token("Invalid ID token!".into()))?
                .claims,
        )
    }
}

#[derive(Debug, Serialize)]
struct RefreshIdTokenPayload<'a> {
    grant_type: &'a str,
    refresh_token: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshIdToken {
    pub access_token: String,
    pub expires_in: String,
    pub token_type: String,
    pub refresh_token: String,
    pub id_token: String,
    pub user_id: String,
    pub project_id: String,
}

// The firebase ID token claims
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdTokenClaims {
    pub exp: u64,
    pub iat: u64,
    pub iss: String,
    pub sub: String,
    pub auth_time: u64,
}
