use std::io::Write;
use std::{env, fs::File, io, path::Path};

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("can't figure target dir 😕");
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut constants = File::create(dest_path)?;

    let user_agent = format!(
        "{}@{}",
        env!("CARGO_PKG_REPOSITORY")
            .split("/")
            .last()
            .unwrap_or(env!("CARGO_PKG_NAME")),
        env!("CARGO_PKG_VERSION")
    );

    write!(
        &mut constants,
        r#"pub const USER_AGENT: &str = "{}";"#,
        user_agent
    )?;

    Ok(())
}
