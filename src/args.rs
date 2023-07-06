use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[arg(long)]
    pub(crate) s3_bucket_name: String,
    #[arg(long)]
    pub(crate) s3_endpoint: String,
    #[arg(long)]
    pub(crate) s3_access_key: String,
    #[arg(long)]
    pub(crate) s3_secret_key: String,
    #[arg(long)]
    pub(crate) s3_region: String,
    #[arg(long)]
    pub(crate) s3_object_key: String,
    #[arg(long)]
    pub(crate) capnp_destination: String,
}

#[derive(Debug)]
pub struct S3Params<'a> {
    pub(crate) s3_bucket_name: &'a str,
    pub(crate) s3_endpoint: &'a str,
    pub(crate) s3_access_key: &'a str,
    pub(crate) s3_secret_key: &'a str,
    pub(crate) s3_region: &'a str,
    pub(crate) s3_object_key: &'a str,
}
