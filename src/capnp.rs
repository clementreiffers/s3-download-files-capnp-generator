use std::collections::HashMap;

fn get_parent_directory(file_path: &str) -> String {
    let mut components: Vec<&str> = file_path.split('/').collect();
    components.pop(); // Remove the file name or last component

    components.join("/")
}

pub fn sort_files(list: Vec<String>) -> Vec<Vec<String>> {
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
