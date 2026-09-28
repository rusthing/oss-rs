use crate::api_client::oss_file_api_client::OssFileApiClient;
use robotech::macros::api_client;

#[api_client]
pub struct OssApiClient {
    pub file_client: OssFileApiClient,
}