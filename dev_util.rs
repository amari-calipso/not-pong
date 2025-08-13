use std::{collections::HashMap, env, fs, io::Error, path::PathBuf, process::Command};

// UNIX-like: rustc dev_util.rs -o dev_util && ./dev_util <command>
// Windows: rustc dev_util.rs -o dev_util.exe && dev_util <command>

const WASM_TARGET: &str = "wasm32-unknown-emscripten";
const WASM_SERVER_PORT: &str = "8080";

fn get_filename(target: &Option<String>) -> &str {
    if let Some(target) = &target {
        if target.contains("-windows-") {
            "univ.exe"
        } else {
            "univ"
        }
    } else {
        if cfg!(windows) {
            "univ.exe"
        } else {
            "univ"
        }
    }
}

fn release(args: &mut Vec<String>, args_map: &mut HashMap<String, usize>) -> Result<(), Error> {
    let target = {
        if let Some(idx) = args_map.get("--target") {
            args.remove(*idx);

            if *idx >= args.len() {
                return Err(Error::other("Target argument was supplied but no target was specified"));
            }

            Some(args.remove(*idx))
        } else {
            None
        }
    };

    let mut command = Command::new("cargo");
    command.args(["build", "--release"]);

    if let Some(target) = &target {
        command.args(["--target", target]);
    }

    println!("Calling cargo");
    if !command.status()?.success() {
        return Err(Error::other("Build failed"));
    }

    let dist = PathBuf::from("dist");
    if dist.exists() {
        println!("\"dist\" folder exists. Deleting");
        fs::remove_dir_all(&dist)?;
    }

    println!("Creating \"dist\" folder");
    fs::create_dir(&dist)?;

    println!("Copying executable to \"dist\" folder");
    let mut executable = PathBuf::from("target").join("release");

    if let Some(target) = &target {
        executable.push(target);
    }

    let filename = get_filename(&target);
    executable.push(&filename);
    fs::copy(&executable, dist.join(&filename))?;

    println!("Copying license to \"dist\" folder");
    fs::copy("LICENSE", &dist.join("LICENSE"))?;
    Ok(())
}

fn copy_file_from(base_dir: &PathBuf, filename: &str, dist: &PathBuf) -> Result<(), Error> {
    fs::copy(&base_dir.join(filename), dist.join(&filename))?;
    Ok(())
}

fn release_wasm() -> Result<PathBuf, Error> {
    unsafe {
        env::set_var(
            "EMCC_CFLAGS", 
            concat!(
                "-O3 -sUSE_GLFW=3 -sFORCE_FILESYSTEM=1 -sALLOW_MEMORY_GROWTH=1 -flto ",
                "-sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1 --no-entry ",
                "-s EXPORTED_RUNTIME_METHODS=HEAPF32"
            )
        );
    }

    let mut command = Command::new("cargo");
    command.args([
        "build", "--release",
        "--target", WASM_TARGET
    ]);

    println!("Calling cargo");
    if !command.status()?.success() {
        return Err(Error::other("Build failed"));
    }

    let dist = PathBuf::from("dist");
    if dist.exists() {
        println!("\"dist\" folder exists. Deleting");
        fs::remove_dir_all(&dist)?;
    }

    println!("Creating \"dist\" folder");
    fs::create_dir(&dist)?;

    let base_dir = PathBuf::from("target").join(WASM_TARGET).join("release");

    println!("Copying WASM to \"dist\" folder");
    copy_file_from(&base_dir, "not_pong.wasm", &dist)?;
    println!("Copying JS to \"dist\" folder");
    copy_file_from(&base_dir, "not-pong.js", &dist)?;
    println!("Copying HTML to \"dist\" folder");
    copy_file_from(&PathBuf::from("resources"), "index.html", &dist)?;
    
    println!("Copying license to \"dist\" folder");
    fs::copy("LICENSE", &dist.join("LICENSE"))?;
    Ok(dist)
}

fn run_wasm() -> Result<(), Error> {
    let dist = release_wasm()?;

    println!("Looking for cargo-server...");
    let mut command = Command::new("cargo");
    command.args(["server", "--help"]);

    if !command.output()?.status.success() {
        println!("Couldn't run cargo-server. Trying to install it...");
        let mut command = Command::new("cargo");
        command.args(["install", "cargo-server"]);

        if !command.status()?.success() {
            return Err(Error::other("An error occurred while installing cargo-server"));
        }
    }

    println!("Running...");
    let mut command = Command::new("cargo");
    command.args([
        "server", "--open",
        "--port", WASM_SERVER_PORT,
        "--path", dist.to_str().unwrap()
    ]);

    if !command.status()?.success() {
        return Err(Error::other("An error occurred while running server"));
    }

    Ok(())
}

macro_rules! commands {
    (
        ($args: ident, $args_map: ident);

        $($name: literal -> $action: expr)+
    ) => {
        if false { unreachable!() }
        $(
            else if let Some(idx) = $args_map.get($name) {
                $args.remove(*idx);
                $args_map.remove($name);
                $action;
            }
        )+
        else {
            println!("No option specified. Use {}", stringify!($($name,)+));
            return Err(Error::other("No option specified"));
        }
    };
}

#[allow(dead_code)] // shuts up rust-analyzer (it thinks this is a module for build.rs)
fn main() -> Result<(), Error> {
    let mut args: Vec<String> = env::args().collect();
    let mut args_map: HashMap<String, usize> = args.iter().enumerate().map(|(i, x)| (x.clone(), i)).collect();

    commands! {
        (args, args_map);
        
        "--release"      -> release(&mut args, &mut args_map)?
        "--release-wasm" -> release_wasm()?
        "--run-wasm"     -> run_wasm()?
    };

    println!("Done!");
    Ok(())
}