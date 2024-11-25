use std::env;

#[derive(Debug)]
pub struct AccessKeyCredential {
    pub access_key_id: String,
    pub access_key_password: String,
}

impl AccessKeyCredential {
    pub fn from_env() -> Result<Self, env::VarError> {
        let id = env::var("ZENLAYER_CLOUD_ACCESS_KEY_ID")?;
        let password = env::var("ZENLAYER_CLOUD_ACCESS_KEY_PASSWORD")?;
        Ok(AccessKeyCredential {
            access_key_id: id.to_owned(),
            access_key_password: password.to_owned(),
        })
    }
}
