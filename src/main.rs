use clap::Parser;

mod static_data;
mod utils;

use utils::utility::create_new_project;
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
    new_model {
        #[clap(value_parser)]
        name: Option<String>,
    },
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
        }

        Commands::new_model { name } => {}
    }
}
