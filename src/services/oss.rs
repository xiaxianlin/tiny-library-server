use crate::utils::{get_oss_client, get_oss_dir};
use aliyun_oss_client::file::Files;

pub async fn upload(content: Vec<u8>, name: String) -> String {
    let client = get_oss_client();
    let dir = get_oss_dir();
    let file_name = format!("{}/{}", dir, name);
    let result = String::from(&file_name);

    let data = client
        .put_content(content, file_name, |_| Some("image/jpeg"))
        .await
        .expect("msg");
    println!("[OSS UPLOAD] result: {}", data);
    result
}

pub async fn upload_by_url(url: String, name: String) -> String {
    println!("[OSS UPLOAD] url: {}", url);
    let file_ext = url.split(".").last().unwrap();
    let file_name = format!("{}.{}", name, file_ext);

    let res = reqwest::get(&url)
        .await
        .expect("读取 url 失败")
        .bytes()
        .await
        .expect("解析 url 内容失败");

    upload(res.to_vec(), file_name).await
}
