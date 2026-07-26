use std::{env, process::Command};

type DynError = Box<dyn std::error::Error>;
const TARGET: &str = "xtensa-esp32s3-none-elf";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    let target = env::args().nth(2);

    if task.is_none() || target.is_none() {
        print_help();
    }

    let task = task.unwrap();
    let target = target.unwrap();

    match task.as_str() {
        "build" => build(target)?,
        _ => print_help(),
    }
    Ok(())
}

fn build(application: String) -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release", "--target", TARGET])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }
}

fn print_help() {
    eprintln!(
        "Tasks:

build <application>           builds specified application and removes LLVM sections
"
    )
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
