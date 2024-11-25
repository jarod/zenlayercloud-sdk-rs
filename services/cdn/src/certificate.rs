#[derive(Debug, Deserialize)]
pub struct CertificateInfo {
    #[serde(rename = "certificateId")]
    pub certificate_id: Option<String>,
    #[serde(rename = "certificateLabel")]
    pub certificate_label: String,
    pub common: String,
    pub fingerprint: String,
    pub issuer: String,
    pub sans: Vec<String>,
    pub algorithm: String,
    #[serde(rename = "createTime")]
    pub create_time: DateTime<Local>,
    #[serde(rename = "startTime")]
    pub start_time: DateTime<Local>,
    #[serde(rename = "endTime")]
    pub end_time: DateTime<Local>,
    pub expired: bool,
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: Option<String>,
}

#[derive(Debug, Serialize, bon::Builder)]
pub struct DescribeCertificatesRequest {
    #[serde(rename = "certificateIds")]
    pub certificate_ids: Option<Vec<String>>,
    #[serde(rename = "certificateLabel")]
    pub certificate_label: Option<String>,
    pub san: Option<String>,
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: Option<String>,
    pub expired: Option<bool>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u16>,
    #[serde(rename = "pageNum")]
    pub page_num: Option<u16>,
}
#[derive(Debug, Deserialize)]
pub struct DescribeCertificatesResponse {
    #[serde(rename = "totalCount")]
    pub total_count: u16,
    #[serde(rename = "dataSet")]
    pub data_set: Vec<CertificateInfo>,
}

#[derive(Debug, Serialize, bon::Builder)]
pub struct CreateCertificateRequest {
    #[serde(rename = "certificateContent")]
    pub certificate_content: String,
    #[serde(rename = "certificateKey")]
    pub certificate_key: String,
    #[serde(rename = "certificateLabel")]
    pub certificate_label: Option<String>,
    pub san: Option<String>,
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct CreateCertificateResponse {
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
}

#[derive(Debug, Serialize, bon::Builder)]
pub struct ModifyCertificateRequest {
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
    #[serde(rename = "certificateContent")]
    pub certificate_content: String,
    #[serde(rename = "certificateKey")]
    pub certificate_key: String,
}
#[derive(Debug, Deserialize)]
pub struct ModifyCertificateResponse {
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
}

#[derive(Debug, Serialize, bon::Builder)]
pub struct DeleteCertificateRequest {
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
}
#[derive(Debug, Deserialize)]
pub struct DeleteCertificateResponse {
    #[serde(rename = "requestId")]
    pub request_id: String,
}

/// CDN / [Certificate API](https://docs.console.zenlayer.com/api-reference/content-delivery-network/certificate)
impl Client {
    pub async fn describe_certificates(
        &self,
        payload: &DescribeCertificatesRequest,
    ) -> Result<Response<DescribeCertificatesResponse>, Error> {
        self.client
            .call_api(self.create_request("DescribeCertificates", payload))
            .await
    }

    pub async fn create_certificate(
        &self,
        payload: &CreateCertificateRequest,
    ) -> Result<Response<CreateCertificateResponse>, Error> {
        self.client
            .call_api(self.create_request("CreateCertificate", payload))
            .await
    }

    pub async fn modify_certificate(
        &self,
        payload: &ModifyCertificateRequest,
    ) -> Result<Response<ModifyCertificateResponse>, Error> {
        self.client
            .call_api(self.create_request("ModifyCertificate", payload))
            .await
    }

    pub async fn delete_certificate(
        &self,
        payload: &DeleteCertificateRequest,
    ) -> Result<Response<DeleteCertificateResponse>, Error> {
        self.client
            .call_api(self.create_request("DeleteCertificate", payload))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn describe_certificates() -> anyhow::Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();

        let credential = AccessKeyCredential::from_env()?;
        let c = Client::new(credential);
        let payload = DescribeCertificatesRequest::builder().build();
        let res = c.describe_certificates(&payload).await;
        println!("{:?}", res);
        Ok(())
    }
}
