use std::{
    env,
    fs::File,
    fs::{read_to_string, write},
    path::Path,
    process::Command,
};
use walkdir::WalkDir;

fn main() {
    let out = env::var("OUT_DIR").expect("out dir variable");
    let out_dir = Path::new(&out);
    let schema = out_dir.join("openapi.yml");

    let mut res = reqwest::blocking::get(
        "https://raw.githubusercontent.com/tonkeeper/opentonapi/master/api/openapi.yml".to_owned(),
    )
    .expect("fetch openapi.yml from github");

    let mut new_file = File::create(&schema).expect("create onenapi.yml file");
    res.copy_to(&mut new_file).expect("write openapi content");

    let _ = Command::new("node")
        .arg("-e")
        .arg(format!(r#"require('child_process').execSync('npx @openapitools/openapi-generator-cli generate -i {} -g rust -o {}')"#, schema.to_string_lossy(), out))
        .status();

    let src_dir = out_dir.join("src");

    // Adding prefix for using generated code inside 'codegen' module
    for entry in WalkDir::new(&src_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let content = read_to_string(entry.path()).expect("read some rs file");
            let updated_content = content.replace("crate::", "crate::codegen::");
            write(entry.path(), updated_content).expect("write changes");
        }
    }

    // Removing all "extern crate"
    let lib_file = src_dir.join("lib.rs");
    let contents = read_to_string(&lib_file).expect("read lib.rs");
    let mut new_contents = String::new();
    for line in contents.lines() {
        if line.trim_start().starts_with("extern crate") || line.trim() == "#[macro_use]" {
            continue;
        }
        new_contents.push_str(line);
        new_contents.push('\n');
    }
    write(&lib_file, new_contents).expect("update content");
}
