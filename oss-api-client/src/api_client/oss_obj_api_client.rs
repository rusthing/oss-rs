use crate::dto::*;
use crate::vo::{OssObjExVo, OssObjVo};
use robotech::macros::feign;
use robotech::micro_svc::FeignApiClient;

#[feign]
pub struct OssObjApiClient {
    client: FeignApiClient,
}

impl OssObjApiClient {
    pub fn new(client: FeignApiClient) -> Self {
        Self { client }
    }
}