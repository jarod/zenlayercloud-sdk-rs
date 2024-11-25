use log::debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    credentials::AccessKeyCredential,
    signer::{Signer, Zc2HS256Signer},
    Error, API_DOMAIN, SDK_VERSION,
};

#[derive(Debug)]
pub struct Request {
    rb: reqwest::RequestBuilder,
}

impl Request {
    pub fn build(self) -> reqwest::Result<reqwest::Request> {
        self.rb.build()
    }
}

/// [Responses](https://docs.console.zenlayer.com/api-reference/api-introduction/instruction/response)
#[derive(Debug, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// Success response. Content are API specific.
    pub response: Option<T>,
    /// Failure response.
    pub code: Option<String>,
    pub message: Option<String>,
}

pub struct Client {
    hc: reqwest::Client,
    signer: Zc2HS256Signer,
}

impl Client {
    pub fn new(credential: AccessKeyCredential) -> Self {
        let hc = reqwest::Client::new();
        let signer = Zc2HS256Signer::new(credential);
        Client { hc, signer }
    }

    pub fn create_request<T>(
        &self,
        service: &str,
        api_version: &str,
        action: &str,
        payload: &T,
    ) -> Request
    where
        T: Serialize + ?Sized,
    {
        let host = API_DOMAIN;
        let url = format!("https://{}/api/v2/{}", host, service);

        let timestamp = format!("{}", chrono::Local::now().timestamp());
        let rb = self
            .hc
            .post(&url)
            .header("Host", host)
            .header("x-zc-version", api_version)
            .header("x-zc-action", action)
            .header("x-zc-timestamp", timestamp)
            .header("x-zc-sdk-version", SDK_VERSION)
            .header("x-zc-sdk-lang", "rust")
            .json(payload); // set Content-Type: application/json
        Request { rb }
    }

    pub async fn call_api<R>(&self, req: Request) -> Result<Response<R>, Error>
    where
        R: DeserializeOwned,
    {
        let mut http_req = req.build()?;
        self.signer.sign_request(&mut http_req)?;
        let http_res = self.hc.execute(http_req).await?;
        debug!("http_res={:?}", http_res);
        let status = http_res.status();
        let res: Response<R> = http_res.json().await?;
        if status.is_success() {
            Ok(res)
        } else {
            Err(Error::APIError {
                request_id: res.request_id,
                http_status: status.as_u16(),
                code: res.code.unwrap_or_default(),
                message: res.message.unwrap_or_default(),
            })
        }
    }
}
