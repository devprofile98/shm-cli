use clap::{Parser, Subcommand};
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::string;

static cmake_file_content: &str = "
cmake_minimum_required(VERSION 3.14)

project(#PROJECT_NAME#)
set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

add_executable(
	#PROJECT_NAME# 
	sources/main.cpp 
	sources/game.cpp
	sources/gamemodel.cpp
	)
target_include_directories(#PROJECT_NAME# PUBLIC includes/ Engine/include/ Engine/openGLRenderer/ shm-physics/include/cyclon/ cyclone-physics/include/cyclone)
target_compile_options(#PROJECT_NAME# PUBLIC -g)

if (WIN32)
	target_link_libraries(#PROJECT_NAME# \"F:/project/SHM/Engine/include/GLFW/glfw3.lib\")
	target_link_libraries(#PROJECT_NAME# \"F:/project/SHM/Engine/include/assimp/assimp-vc142-mtd.lib\")

elseif(UNIX)
	target_link_libraries(#PROJECT_NAME# ${CMAKE_DL_LIBS})
	target_link_libraries(#PROJECT_NAME# Threads::Threads)
	target_link_libraries(#PROJECT_NAME# glfw3)
	target_link_libraries(#PROJECT_NAME# assimp)
endif()
target_link_libraries(#PROJECT_NAME# shm-engine)

# copy the asset folder to generated bin file
# so the asset folder can be used in the game
# and the path can be relative
add_custom_command(
	TARGET #PROJECT_NAME# 
	POST_BUILD
	COMMAND ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/assets ${CMAKE_BINARY_DIR}/bin/assets
)
message(${CMAKE_CURRENT_SOURCE_DIR}/assets)

";

#[derive(Parser)]
#[clap(author="ahmad mansoori", version="0.0.1", about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Adds files to myapp
    New {
        #[clap(value_parser)]
        name: Option<String>,
        api: Option<String>,
    },
    Build,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, api } => {
            let project_name = name
                .as_ref()
                .expect("You must provide a Name for your new project");
            // create CMakeLists.txt file and put necessary data to it
            if let Ok(mut file) = File::create("CMakeLists.txt") {
                file.write(
                    cmake_file_content
                        .replace("#PROJECT_NAME#", &project_name)
                        .as_bytes(),
                )
                .expect("Error creating project");
            } else {
                println!("Cannot create project file! Quiting.");
            }

            // create necessary directory for project
            for dir_name in ["sources", "includes", "build", "Engine"] {
                if let Ok(_) = create_dir(dir_name) {
                } else {
                    println!("Failed to create Project files")
                }
            }
        }
        Commands::Build => {
            println!("Building the Project, please wait ...");
        }
    }
}
