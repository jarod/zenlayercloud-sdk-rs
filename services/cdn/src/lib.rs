use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use zenlayercloud_sdk::{
    self as zc_sdk, credentials::AccessKeyCredential, Error, Request, Response,
};

const API_VERSION: &str = "2024-02-29";
const SERVICE: &str = "cdn";

pub struct Client {
    client: zc_sdk::Client,
}

impl Client {
    pub fn new(credential: AccessKeyCredential) -> Self {
        let sdk_client = zc_sdk::Client::new(credential);
        Client { client: sdk_client }
    }

    fn create_request<T>(&self, action: &str, payload: &T) -> Request
    where
        T: Serialize + ?Sized,
    {
        self.client
            .create_request(SERVICE, API_VERSION, action, payload)
    }
}

include!("certificate.rs");
