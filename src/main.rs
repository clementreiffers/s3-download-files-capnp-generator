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
use std::fmt::{format, Error};
use std::fs;
use std::process::exit;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let s3_params = S3Params {
        s3_endpoint: &*args.s3_endpoint,
        s3_bucket_name: &*args.s3_bucket_name,
        s3_object_key: &*args.s3_object_key,
        s3_region: &*args.s3_region,
    };

    let client: S3Client = create_s3_client(&s3_params);

    let links = s3_params.s3_object_key.split(",");

    for link in links {
        download_dir(&args.destination, &client, link, &s3_params).await;
    }
    println!("files downloaded !");

    let config: String = create_config(&args.destination);

    let capnp_destination = format!("{}/config.capnp", &args.destination);
    fs::write(capnp_destination, config.as_bytes()).expect("failed to write file");

    println!("capnp generated!");

    Ok(())
}
