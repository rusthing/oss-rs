use crate::api_client::oss_bucket_api_client::OssBucketApiClient;
use crate::api_client::oss_file_api_client::OssFileApiClient;
use crate::api_client::oss_obj_api_client::OssObjApiClient;
use crate::api_client::oss_obj_ref_api_client::OssObjRefApiClient;
use robotech::macros::api_client;

#[api_client]
pub struct OssApiClient {
    pub file_client: OssFileApiClient,
    pub bucket_client: OssBucketApiClient,
    pub obj_client: OssObjApiClient,
    pub obj_ref_client: OssObjRefApiClient,
}
