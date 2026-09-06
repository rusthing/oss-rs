use crate::dto::*;
use crate::vo::{OssObjRefExVo, OssObjRefVo};
use robotech::macros::feign;
use robotech::micro_svc::FeignApiClient;

#[feign]
pub struct OssObjRefApiClient {
    client: FeignApiClient,
}

impl OssObjRefApiClient {
    pub fn new(client: FeignApiClient) -> Self {
        Self { client }
    }
}