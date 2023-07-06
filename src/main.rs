mod args;
mod capnp;
mod s3;

use crate::args::{Args, S3Params};
use std::collections::HashMap;

use crate::s3::{create_s3_client, download_dir};
use clap::Parser;
use regex::Regex;
use rusoto_s3::S3Client;
use std::fmt::Error;
use std::fs;
use tokio;

fn get_parent_directory(file_path: &str) -> String {
    let mut components: Vec<&str> = file_path.split('/').collect();
    components.pop(); // Remove the file name or last component

    components.join("/")
}

fn get_filename(file_path: &str) -> &str {
    file_path.split("/").last().expect("failed to get filename")
}

pub(crate) fn sort_files(list: Vec<String>) -> Vec<Vec<String>> {
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();

    for file_path in list.iter() {
        let parent_directory = get_parent_directory(file_path);
        hashmap
            .entry(parent_directory)
            .or_insert(Vec::new())
            .push(file_path.to_owned());
    }

    hashmap.into_iter().map(|(_, v)| v).collect()
}

fn list_files(path: &str) -> Vec<String> {
    let entries = fs::read_dir(path).expect("failed to read dir");
    let mut files = Vec::new();

    for entry in entries {
        let path = entry.expect("failed to get entry").path();

        if path.is_dir() {
            files.extend(list_files(&path.to_string_lossy()));
        } else {
            files.push(path.display().to_string())
        }
    }
    files
}

fn set_wasm_module(path: &str) -> String {
    format!(
        "( name = \"./{}\", wasm = embed \"{}\" )",
        get_filename(&path),
        path
    )
}

fn set_js_module(path: &str) -> String {
    format!("( name = \"entrypoint\", wasm = embed \"{}\" )", path)
}

fn is_wasm_file(path: &str) -> bool {
    let regex = Regex::new(r"\.wasm$").unwrap();
    regex.is_match(&path)
}

fn manage_worker_module(path: &str) -> String {
    match is_wasm_file(&path) {
        true => set_wasm_module(&path),
        false => set_js_module(&path),
    }
}
fn format_modules(modules: Vec<String>) -> String {
    format!(
        "modules = [ {} ], compatibilityDate = \"2023-02-28\"",
        modules.join(",")
    )
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
        s3_object_key: &*args.s3_object_key,
    };

    let client: S3Client = create_s3_client(&s3_params);

    let links = s3_params.s3_object_key.split(",");

    // for link in links {
    //     download_dir(&client, link, &s3_params).await;
    // }

    let folder_path = "download";

    let sorted_files = sort_files(list_files(folder_path));

    let mut workers: Vec<String> = Vec::new();
    for file in sorted_files {
        let mut modules = Vec::new();
        for path in file {
            modules.push(manage_worker_module(&path));
        }
        let modules = format_modules(modules);
        workers.push(modules);
    }
    let mut services = Vec::new();
    let mut sockets = Vec::new();
    let mut total_worker = format!("");
    for (index, worker) in workers.iter().enumerate() {
        let worker_name = format!("w{}", index);
        total_worker =
            total_worker + &format!("const {} :Workerd.Worker = ( {} );", worker_name, worker);
        services.push(format!(
            "(name = \"{}\", worker=.{})",
            worker_name, worker_name
        ));
        sockets.push(format!(
            "(name=\"http\", address = \"*:{}\", http = (), service = \"{}\")",
            8080 + index,
            worker_name
        ));
    }
    let config =
        format!("using Workerd = import \"workerd/workerd.capnp\"; const config :Workerd.Config = ( services = [{}], sockets = [{}], ); {}", services.join(","), sockets.join(","), total_worker);
    std::fs::write("config.capnp", config.as_bytes()).unwrap();

    Ok(())
}
