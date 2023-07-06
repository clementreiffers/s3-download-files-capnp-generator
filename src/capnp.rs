use regex::Regex;
use std::collections::HashMap;
use std::fs;

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
    format!("( name = \"entrypoint\", esModule = embed \"{}\" )", path)
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
fn format_modules(modules: String) -> String {
    format!(
        "modules = [ {} ], compatibilityDate = \"2023-02-28\"",
        modules
    )
}
pub fn create_workers(source: &str) -> Vec<String> {
    let files = sort_files(list_files(source));
    let mut workers: Vec<String> = Vec::new();
    for file in files {
        let mut modules = format!("");
        for path in file {
            let module_part = manage_worker_module(&path);
            let regex = Regex::new(r"entrypoint").unwrap();
            modules = match regex.is_match(&module_part) {
                true => format!("{},{}", module_part, modules),
                false => format!("{}{}", modules, module_part),
            }
        }
        let modules = format_modules(modules);
        workers.push(modules);
    }
    workers
}

pub fn create_config(source: &str) -> String {
    let mut workers = create_workers(source);

    let (services, sockets): (Vec<String>, Vec<String>) = workers
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let worker_name = format!("w{}", index);
            let service = format!("(name = \"{}\", worker = .{})", worker_name, worker_name);
            let socket = format!(
                "( name= \"http\", address = \"*:{}\", http = (), service = \"{}\" )",
                8080 + index,
                worker_name
            );
            (service, socket)
        })
        .unzip();

    let total_worker = workers
        .iter()
        .enumerate()
        .map(|(index, worker)| {
            let worker_name = format!("w{}", index);
            format!("const {} :Workerd.Worker = ( {} );", worker_name, worker)
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        "using Workerd = import \"/workerd/workerd.capnp\"; const config :Workerd.Config = ( services = [{}], sockets = [{}], ); {}",
        services.join(","),
        sockets.join(","),
        total_worker
    )
}
