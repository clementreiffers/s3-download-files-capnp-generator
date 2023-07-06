mod args;
mod s3;

use crate::args::{Args, S3Params};

use crate::s3::{create_s3_client, download_files, list_files};
use clap::Parser;
use rusoto_s3::S3Client;
use std::fmt::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let s3_params = S3Params {
        s3_endpoint: &*args.s3_endpoint,
        s3_secret_key: &*args.s3_secret_key,
        s3_access_key: &*args.s3_access_key,
        s3_region: &*args.s3_region,
        s3_bucket_name: &*args.s3_bucket_name,
        s3_object_key: &*args.s3_object_key,
    };

    let client: S3Client = create_s3_client(&s3_params);
    let object_key = &s3_params.s3_object_key;
    if let Some(contents) = list_files(&client, &object_key, &s3_params).await.contents {
        for object in contents {
            download_files(&client, &object.key.as_ref().unwrap(), &s3_params).await;
        }
    }
    Ok(())
}
