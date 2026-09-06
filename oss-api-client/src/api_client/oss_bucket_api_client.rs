use crate::dto::*;
use crate::vo::{OssBucketExVo, OssBucketVo};
use robotech::macros::feign;
use robotech::micro_svc::FeignApiClient;

#[feign]
pub struct OssBucketApiClient {
    client: FeignApiClient,
}

impl OssBucketApiClient {
    pub fn new(client: FeignApiClient) -> Self {
        Self { client }
    }
}