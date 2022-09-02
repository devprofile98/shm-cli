use clap::Parser;

mod static_data;
mod utils;

use utils::utility::{build_project, create_new_project};
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
            create_new_project(&project_name, "OpenGL");
        }
        Commands::Build => {
            println!("Building the Project, please wait ...");
            if !build_project() {
                println!("Failed to find SHM library!");
                println!("Please install the shm first, see the installation guide at:\nhttps://github.com/devprofile98/shm.git");
            } else {
                println!("You can run the project from -- build/bin/ --");
            }
        }
    }
}
