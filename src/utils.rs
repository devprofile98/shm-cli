pub mod utility {

    use crate::static_data::{
        CMAKE_FILE_CONTENT, LOAD_MODEL_HEADER, LOAD_MODEL_SOURCE, MAIN_CPP, SHM_MINIMUM_SETTING,
    };
    use std::fs::{create_dir, File};
    use std::io::prelude::*;
    use std::process::Command;

    pub fn create_new_project(name: &str, api: &str) -> bool {
        if let Ok(mut file) = File::create("CMakeLists.txt") {
            file.write(
                CMAKE_FILE_CONTENT
                    .replace("#PROJECT_NAME#", name)
                    .as_bytes(),
            )
            .expect("Error in creating project");
        } else {
            println!("Cannot create necessary Files and Directories!");
            return false;
        }
        for dir_name in ["sources", "includes", "build", "Engine"] {
            if let Err(_) = create_dir(dir_name) {
                println!("Failed to create Project files");
                return false;
            }
        }
        let files_to_create = [
            MAIN_CPP,
            LOAD_MODEL_HEADER,
            LOAD_MODEL_SOURCE,
            SHM_MINIMUM_SETTING,
        ];
        for f in files_to_create {
            if let Ok(mut file) = File::create("sources/main.cpp") {
                file.write(f.replace("#PROJECT_NAME#", &name).as_bytes())
                    .expect("Failed to generate project files");
            } else {
                println!("Failed to generate project files!");
                return false;
            }
        }
        true
    }

    pub fn check_for_engine_library() -> bool {
        if let Ok(ldconfig_cache) = Command::new("/sbin/ldconfig").arg("-p").output() {
            if ldconfig_cache.status.success() {
                let cmd_output = String::from_utf8(ldconfig_cache.stdout)
                    .expect("Failed in checking for SHM library existance!");
                if cmd_output.contains("libshm-engine.so") {
                    return true;
                } else {
                    println!("Cannot open /sbin/ldconfig -p {}", cmd_output);
                    return false;
                }
            } else {
                println!("Cannot check for library, failed to open `ldconfig`");
                return false;
            }
        }
        false
    }

    pub fn build_project() -> bool {
        if !check_for_engine_library() {
            return false;
        }
        // change directory to build
        // run cmake ..
        Command::new("cmake")
            .arg("..")
            .current_dir("build/")
            .output()
            .expect("Failed to build the project!");

        // run cmake --build
        Command::new("cmake")
            .args(["--build", "."])
            .current_dir("build/")
            .output()
            .expect("Failed to build the project!");
        println!("Building the project compeleted");
        true
    }
}
