use std::{
    fs::File,
    io::BufReader,
    process::{Command, Output},
};

fn main() {
    // Listen for files that may change and rebuild when the files change
    println!("cargo:rerun-if-changed=build.rs");
    // If the directory where the static file is located does not exist, install it directly
    let static_dir = std::env::current_dir()
        .expect("Failed to get the current directory")
        .join("static");
    if !static_dir.exists() {
        println!("cargo:warning=The static dir of scalar does not exist.");
        install_and_compress();
        return;
    }
    // If the static file does not exist, install it directly
    let static_js = static_dir.join("scalar-api-reference.js");
    if !static_js.exists() {
        println!("cargo:warning=The static file of scalar does not exist.");
        install_and_compress();
        return;
    }
    // The latest version of @scalar-api-reference was obtained via pnpm
    let Some(scalar_version) = get_scalar_version_by_pnpm() else {
        return;
    };
    // Read package.json via package.json
    let Some(scalar_version_another) = get_scalar_version_from_package_json() else {
        return;
    };
    if scalar_version_another != scalar_version {
        println!("cargo:warning=The scalar has a new version.");
        install_and_compress();
    }
}

fn get_scalar_version_from_package_json() -> Option<String> {
    let package_json = std::env::current_dir()
        .expect("Failed to get the current directory")
        .join("package.json");
    let package_json_file = File::open(package_json).expect("Failed to open package.json");
    let reader = BufReader::new(package_json_file);
    let json = serde_json::from_reader::<_, serde_json::Value>(reader)
        .expect("Failed to parse package.json");
    if let Some(dev_dependencies) = json.get("devDependencies")
        && let Some(scalar) = dev_dependencies.get("@scalar/api-reference")
        && let Some(version) = scalar.as_str()
    {
        Some(version.replace("^", ""))
    } else {
        println!("cargo:error=Failed to get scalar version from package.json");
        None
    }
}

fn get_scalar_version_by_pnpm() -> Option<String> {
    let output = Command::new("pnpm")
        .arg("info")
        .arg("@scalar/api-reference")
        .arg("version")
        .output()
        .expect("Failed to execute pnpm update");
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        println!(
            "cargo:error=Failed to get scalar version from npm, err is {}, {}",
            String::from_utf8_lossy(&output.stderr),
            &output.status
        );
        None
    }
}

fn install_and_compress() {
    // Update front-end dependencies
    let output = Command::new("pnpm")
        .arg("update")
        .output()
        .expect("Failed to execute pnpm update");
    print_output("pnpm update", output);
    // Install front-end dependencies
    let output = Command::new("pnpm")
        .arg("install")
        .output()
        .expect("Failed to execute pnpm install");
    print_output("pnpm install", output);
    // Compress scala-api-reference.js
    let output = Command::new("pnpm")
        .arg("run")
        .arg("build:compress")
        .output()
        .expect("Failed to execute pnpm run build:compress");
    print_output("pnpm run build:compress", output);
}

fn print_output(
    msg: &str,
    Output {
        status,
        stdout,
        stderr,
    }: Output,
) {
    if status.success() {
        let out = if stdout.is_empty() {
            "".to_owned()
        } else {
            format!(", out is {}", String::from_utf8_lossy(&stdout))
        };
        println!("cargo:warning={msg}{out}");
    } else {
        let err = if stderr.is_empty() {
            format!(", {status}")
        } else {
            format!(", err is {}, {status}", String::from_utf8_lossy(&stderr))
        };
        println!("cargo:error={msg}{err}",);
    }
}
