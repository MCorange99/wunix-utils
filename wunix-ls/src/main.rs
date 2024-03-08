use std::fs;

fn main() {
    if let Ok(entries) = fs::read_dir(".") {
        let files: Vec<String> = entries
            .filter_map(|entry| {
                entry.ok().map(|e| {
                    let file_name = e.file_name().to_string_lossy().to_string();
                    if e.path().is_dir() {
                        format!("{}/", file_name)
                    } else {
                        file_name
                    }
                })
            })
            .collect();
        println!("{}", files.join(" "));
    } else {
        eprintln!("error: can't process directory");
    }
}
