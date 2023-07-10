# s3-download-files-capnp-generator

First download files from S3, and then generate the `Cap'n'proto` file.
Needed to use the `workerd` runtime from cloudflare.


## Overview

1. [Install it](#install-it)
2. [Build it](#build-it)

## Install it

run : 
```terminal 
cargo run -- 
    --s3-bucket-name stage-cf-worker 
    --s3-endpoint 'https://s3.fr-par.scw.cloud' 
    --s3-region fr-par 
    --s3-object-key 398803b74bcdb1b454434669bc634190 
    --destination ./download
```
- the field `--s3-bucket-name` concerns the name of the S3 Bucket you want.
- the field `--s3-endpoint` concerns the endpoint of your S3 Bucket.
- the field `--s3-region` concerns the region in which your S3 Bucket is host.
- the field `--s3-object-key` concerns the path of the object you want to dowload
- the field `destination` concerns where your file will be downloaded and where the `Cap'n'proto` will be generated.

> **NOTE**
> you can write different S3 links separated by a commas if you want multiple files from it.

> **NOTE**
> you maybe need to configure the `awscli` by running `aws configure`

## Build it
if you want to build on your specific OS, run :
```terminal
cargo build
```

Or you can download executables from [this link](https://github.com/clementreiffers/s3-download-files-capnp-generator/releases)
