use std::{env, ffi::OsStr, fs, io, path::Path};

fn main() {
    let var = "my_path";
    match env::var_os(var) {
        Some(path) => match path.to_str() {
            Some(path_string) => match copy_dir_all("scaffolding", path_string.to_string()) {
                Ok(()) => println!("cargo:rerun-if-changed=build.rs"),
                Err(e) => {
                    println!("There was an error in scaffolding process.");
                    println!("{}", e);
                }
            },
            None => println!("Variable {} could not be parsed.", var),
        },
        None => {
            println!("Variable {} not found.", var);
            println!("Scaffolding couldn't be carried out.")
        }
    }
}
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), io::Error> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            let path = entry.path();
            let file = path.file_name().unwrap();
            let file_name = file.to_str().unwrap().to_string();
            let rust_file_name = if path.extension().and_then(OsStr::to_str) == Some("bak") {
                &file_name[..file_name.rfind(".bak").unwrap()]
            } else {
                &file_name.as_str()
            };
            fs::copy(entry.path(), dst.as_ref().join(rust_file_name))?;
        }
    }
    Ok(())
}
