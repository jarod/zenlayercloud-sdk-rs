## Introduction
Async Zenlayer Cloud [openapi](https://docs.console.zenlayer.com/api-reference) wrapper.

## Usage

Cargo.toml:
```toml
zenlayercloud-sdk = "*"
# service api to use
zenlayercloud-sdk-cdn = "*"
```

rust:
```rust
use zenlayercloud_sdk::credentials::AccessKeyCredential;
use zenlayercloud_sdk_cdn::*;

async fn async_function() -> Result<()> {
    // read credential from environment variable ZENLAYER_CLOUD_ACCESS_KEY_ID & ZENLAYER_CLOUD_ACCESS_KEY_PASSWORD
    let credential = AccessKeyCredential::from_env()?;
    let c = Client::new(credential);
    let payload = DescribeCertificatesRequest::builder().build();
    let res = c.describe_certificates(&payload).await;
    Ok(())
}
```
