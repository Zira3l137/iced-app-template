use std::{env, fs, path::PathBuf};

#[cfg(windows)]
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Cargo places build artifacts in .../target/{profile}/build/{pkg}/out
    // The executable itself is one level higher: ../../../../{profile}
    let exe_dir = out_dir.ancestors().nth(4).expect("Failed to locate target directory");

    let exe_profile_dir = exe_dir.join(env::var("PROFILE").unwrap());
    let resources_src =
        PathBuf::from("../../resources").canonicalize().expect("Failed to canonicalize resources path");
    let resources_dst = exe_profile_dir.join("resources");

    if let Err(e) = copy_dir_all(&resources_src, &resources_dst) {
        println!("cargo:warning=Failed to copy resources: {e}");
    }

    let mut res = winres::WindowsResource::new();
    res.set_icon(resources_src.join("icon.ico").to_str().unwrap_or("icon.ico"));
    if let Err(e) = res.compile() {
        println!("cargo:warning=Failed to compile resource file: {e}");
    };
}

#[cfg(not(windows))]
fn main() {}

fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(&entry.path(), &dest_path)?;
        }
    }
    Ok(())
}
