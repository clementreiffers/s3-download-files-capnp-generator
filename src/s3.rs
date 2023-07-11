use crate::args::S3Params;
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_core::{Client, HttpClient, Region};
use rusoto_s3::{GetObjectRequest, ListObjectsV2Output, ListObjectsV2Request, S3Client, S3};
use tokio::io::AsyncReadExt;

fn get_parent_directory(path: &String) -> String {
    path.trim_end_matches("/")
        .rsplitn(2, "/")
        .last()
        .expect("failed to get parent directories")
        .to_string()
}

pub fn create_s3_client(s3_params: &S3Params) -> S3Client {
    let region: Region = Region::Custom {
        name: s3_params.s3_region.parse().unwrap(),
        endpoint: s3_params.s3_endpoint.parse().unwrap(),
    };
    S3Client::new(region)
}

pub async fn download_files<'a>(
    destination: &str,
    client: &S3Client,
    object_key: &String,
    s3_params: &S3Params<'a>,
) -> String {
    // Download each file individually using its object key
    let get_request = GetObjectRequest {
        bucket: s3_params.s3_bucket_name.to_string(),
        key: object_key.to_string(),
        ..Default::default()
    };

    let get_response = client.get_object(get_request).await.unwrap();

    let output = get_response.body.expect("failed to get body");
    let mut buf = vec![];
    output
        .into_async_read()
        .read_to_end(&mut buf)
        .await
        .unwrap();
    let destination = format!("{}/{}", destination, object_key);
    std::fs::create_dir_all(get_parent_directory(&destination)).expect("failed to create all dir");
    std::fs::write(&destination, buf).unwrap();
    destination
}

pub async fn list_files<'a>(
    client: &S3Client,
    object_key: &str,
    s3_params: &S3Params<'a>,
) -> ListObjectsV2Output {
    let list_request = ListObjectsV2Request {
        bucket: s3_params.s3_bucket_name.parse().unwrap(),
        prefix: Some(object_key.to_string()),
        ..Default::default()
    };

    client.list_objects_v2(list_request).await.unwrap()
}

pub async fn download_dir<'a>(
    destination: &str,
    client: &S3Client,
    link: &str,
    s3_params: &S3Params<'a>,
) -> Vec<String> {
    let mut files_downloaded: Vec<String> = Vec::new();
    let contents = list_files(&client, link, &s3_params)
        .await
        .contents
        .expect("failed to get contents");

    for object in contents {
        files_downloaded.push(
            download_files(
                destination,
                &client,
                &object.key.as_ref().unwrap(),
                &s3_params,
            )
            .await,
        );
    }
    files_downloaded
}
