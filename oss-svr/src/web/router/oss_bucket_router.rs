use robotech_macros::router;
#[router(all, routes[
    ("/oss/bucket/cascade/{id}", delete(del_cascade)),  // 级联删除
])]
struct OssBucketRouter;
