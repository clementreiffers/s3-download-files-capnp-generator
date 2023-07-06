use clap::{arg, Parser};
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_core::{HttpClient, Region};
use rusoto_s3::{GetObjectRequest, ListObjectsV2Output, ListObjectsV2Request, S3Client, S3};
use std::fmt::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::{fs, io};
use tokio;
use tokio::io::AsyncReadExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(long)]
    s3_bucket_name: String,
    #[arg(long)]
    s3_endpoint: String,
    #[arg(long)]
    s3_access_key: String,
    #[arg(long)]
    s3_secret_key: String,
    #[arg(long)]
    s3_region: String,
}

#[derive(Debug)]
struct S3Params<'a> {
    s3_bucket_name: &'a str,
    s3_endpoint: &'a str,
    s3_access_key: &'a str,
    s3_secret_key: &'a str,
    s3_region: &'a str,
}

fn get_parent_directory(path: &String) -> String {
    path.trim_end_matches("/")
        .rsplitn(2, "/")
        .last()
        .expect("failed to get parent directories")
        .to_string()
}

fn create_s3_client(s3_params: &S3Params) -> S3Client {
    let credentials: AwsCredentials =
        AwsCredentials::new(s3_params.s3_access_key, s3_params.s3_secret_key, None, None);
    let provider: StaticProvider = StaticProvider::from(credentials);

    let region: Region = Region::Custom {
        name: s3_params.s3_region.parse().unwrap(),
        endpoint: s3_params.s3_endpoint.parse().unwrap(),
    };
    let dispatcher = HttpClient::new().expect("Failed to create request dispatcher");

    S3Client::new_with(dispatcher, provider, region)
}

async fn download_files<'a>(client: &S3Client, object_key: &String, s3_params: &S3Params<'a>) {
    // Download each file individually using its object key
    let get_request = GetObjectRequest {
        bucket: s3_params.s3_bucket_name.to_string(),
        key: object_key.to_string(),
        ..Default::default()
    };

    let get_response = client.get_object(get_request).await.unwrap();

    if let Some(output) = get_response.body {
        let mut buf = vec![];
        output
            .into_async_read()
            .read_to_end(&mut buf)
            .await
            .unwrap();
        let destination = format!("download/{}", object_key);
        std::fs::create_dir_all(get_parent_directory(&destination))
            .expect("failed to create all dir");
        std::fs::write(destination, buf).unwrap();
    }
}

async fn list_files<'a>(
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let s3_params = S3Params {
        s3_endpoint: &*args.s3_endpoint,
        s3_secret_key: &*args.s3_secret_key,
        s3_access_key: &*args.s3_access_key,
        s3_region: &*args.s3_region,
        s3_bucket_name: &*args.s3_bucket_name,
    };

    let object_key = "398803b74bcdb1b454434669bc634190/wasm-worker/1688628480";
    let client: S3Client = create_s3_client(&s3_params);

    if let Some(contents) = list_files(&client, &object_key, &s3_params).await.contents {
        for object in contents {
            download_files(&client, &object.key.as_ref().unwrap(), &s3_params).await;
        }
    }
    Ok(())
}
