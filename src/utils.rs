pub mod utility {

    use crate::static_data::{
        cmake_file_content, load_model_header, load_model_source, main_cpp, shm_minimal_setting,
    };
    use std::fs::{create_dir, File};
    use std::io::prelude::*;

    pub fn create_new_project(name: &str, api: &str) -> bool {
        if let Ok(mut file) = File::create("CMakeLists.txt") {
            file.write(
                cmake_file_content
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
        if let Ok(mut file) = File::create("sources/main.cpp") {
            file.write(main_cpp.replace("#PROJECT_NAME#", &name).as_bytes())
                .expect("Failed to generate main.cpp file");
        } else {
            println!("Failed to generate  main.cpp file");
            return false;
        }
        if let Ok(mut file) = File::create("sources/firstmodel.cpp") {
            file.write(load_model_source.as_bytes())
                .expect("Failed to write into firstmodel.cpp file");
        } else {
            println!("Failed to generate  main.cpp file");
            return false;
        }
        if let Ok(mut file) = File::create("includes/firstmodel.hpp") {
            file.write(load_model_header.as_bytes())
                .expect("Failed to write into firstmodel.hpp file");
        } else {
            println!("Failed to generate  firstmodel.hpp file");
            return false;
        }
        if let Ok(mut file) = File::create("sources/game.cpp") {
            file.write(shm_minimal_setting.as_bytes())
                .expect("Failed to write into game.cpp file");
        } else {
            println!("Failed to generate  game.cpp file");
            return false;
        }
        true
    }
}
