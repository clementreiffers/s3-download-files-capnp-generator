mod args;
mod capnp;
mod s3;

use crate::args::{Args, S3Params};
use std::collections::HashMap;

use crate::capnp::{create_config, create_workers};
use crate::s3::{create_s3_client, download_dir};
use clap::Parser;
use regex::Regex;
use rusoto_s3::S3Client;
use std::fmt::Error;
use std::fs;
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

    let links = s3_params.s3_object_key.split(",");

    for link in links {
        download_dir(&client, link, &s3_params).await;
    }
    let config: String = create_config(&args.capnp_destination);
    std::fs::write("config.capnp", config.as_bytes()).expect("failed to write file");

    Ok(())
}
