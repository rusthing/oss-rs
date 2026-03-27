use axum::extract::DefaultBodyLimit;
use robotech_macros::router;

#[router(none, routes[
    ("/oss/file/upload/{bucket}", post(upload).layer(DefaultBodyLimit::disable())), // 上传文件
    ("/oss/file/download/{obj_id}", get(download)),                                 // 下载文件
    ("/oss/file/preview/{obj_id}", get(preview)),                                   // 预览文件
])]
struct OssFileRouter;
