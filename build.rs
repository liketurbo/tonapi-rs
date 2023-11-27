use std::{fs::File, path::Path, process::Command};

fn main() {
    let dest = Path::new("generated-tonapi").join("openapi.yml");

    if let Some(par) = dest.parent() {
        if !par.exists() {
            std::fs::create_dir(par).expect("create generated-tonapi dir");
        }
    }

    if !dest.is_file() {
        let mut res = reqwest::blocking::get(
            "https://raw.githubusercontent.com/tonkeeper/opentonapi/master/api/openapi.yml"
                .to_owned(),
        )
        .expect("fetch openapi.yml from github");

        let dest = Path::new("generated-tonapi").join("openapi.yml");

        if let Some(par) = dest.parent() {
            if !par.exists() {
                std::fs::create_dir(par).expect("create generated-tonapi dir");
            }
        }

        let mut file = File::create(dest).expect("create onenapi.yml file");
        res.copy_to(&mut file).expect("write openapi content");
    }

    let cargo_f = Path::new("generated-tonapi").join("Cargo.toml");
    if !cargo_f.is_file() {
        let _ = Command::new("node")
        .arg("-e")
        .arg(r#"require('child_process').execSync('npx @openapitools/openapi-generator-cli generate -i generated-tonapi/openapi.yml -g rust -o generated-tonapi')"#)
        .status();
    }
}
