use anyhow::Context;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use crate::Error;

use crate::credentials::AccessKeyCredential;

type HS256 = Hmac<Sha256>;

pub trait Signer {
    fn sign_request(&self, req: &mut reqwest::Request) -> Result<(), Error>;
}

/// [ZC2-HMAC-SHA256](https://docs.console.zenlayer.com/api-reference/api-introduction/instruction/sign) Signer implementation
pub struct Zc2HS256Signer {
    algorithm: &'static str,
    credential: AccessKeyCredential,
}

impl Zc2HS256Signer {
    pub fn new(credential: AccessKeyCredential) -> Self {
        Zc2HS256Signer {
            algorithm: "ZC2-HMAC-SHA256",
            credential,
        }
    }
}

impl Signer for Zc2HS256Signer {
    fn sign_request(&self, req: &mut reqwest::Request) -> Result<(), Error> {
        use reqwest::header::HeaderValue;

        let canonical_uri = "/";
        let canonical_query_string = "";
        let canonical_headers = format!(
            "content-type:{}\nhost:{}\n",
            req.headers()["Content-Type"]
                .to_str()
                .context("header Content-Type")?,
            req.headers()["Host"].to_str().context("header Host")?,
        );

        let signed_headers = "content-type;host";

        let body_bytes = req
            .body()
            .context("payload not set")?
            .as_bytes()
            .context("failed to read payload")?;
        let hashed_request_payload = hex_sha256(body_bytes);

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            req.method(),
            canonical_uri,
            canonical_query_string,
            canonical_headers,
            signed_headers,
            &hashed_request_payload,
        );

        let timestamp = req.headers()["x-zc-timestamp"].to_str().context("header x-zc-timestamp")?;
        let string_to_sign = format!(
            "{}\n{}\n{}",
            self.algorithm,
            &timestamp,
            hex_sha256(canonical_request.as_bytes()),
        );

        let mut hs256 =
            HS256::new_from_slice(&self.credential.access_key_password.as_bytes()).unwrap();
        hs256.update(string_to_sign.as_bytes());
        let signature = format!("{:x}", hs256.finalize().into_bytes());

        let authorization = format!(
            "{} Credential={}, SignedHeaders={}, Signature={}",
            self.algorithm, self.credential.access_key_id, signed_headers, signature,
        );
        req.headers_mut()
            .insert("Authorization", HeaderValue::from_str(&authorization).context("set header Authorization")?);
        req.headers_mut().insert(
            "X-ZC-Signature-Method",
            HeaderValue::from_str(self.algorithm).context("set header X-ZC-Signature-Method")?,
        );

        Ok(())
    }
}

fn hex_sha256(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}
