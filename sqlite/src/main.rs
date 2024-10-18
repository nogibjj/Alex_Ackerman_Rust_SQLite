//this will be the CLI portion of the project where we accept
//user defined arguments and call lib.rs logic to handle them
use clap::{Parser, Subcommand};

//Here we define a struct (or object) to hold our CLI arguments

//for #[STUFF HERE] syntax, these are called attributes. Dont worry about them
//for now, they define behavior for elements in rust.

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
//Think of a struct as a class which makes objects in python
//This is designed to generate an object out of the CLI inputs
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

//An enum is a type in rust that can have multiple exauhstive and mutually exclusive options
//We also know that the command can be 1 of 4 (really 3) options
//Create, Read and Update (query), Delete

#[derive(Debug, Subcommand)]
//By separating out the commands as enum types we can easily match what the user is
//trying to do in main
enum Commands {
    ///Pass a table name to create a table
    #[command(alias = "c", short_flag = 'c')]
    Create { table_name: String },
    ///Pass a query string to execute Read or Update operations
    #[command(alias = "q", short_flag = 'q')]
    Query { query: String },
    ///Pass a sql formatted delete query
    #[command(alias = "d", short_flag = 'd')]
    Delete { delete_query: String },
}

fn main() {
    //Here we parse the CLI arguments and store them in the args object
    let args = Cli::parse();
    //Here we can match the behavior on the subcommand and call our lib logic
    match args.command {
        Commands::Create { table_name } => {
            println!("Creating Table {}", table_name);
        }
        Commands::Query { query } => {
            println!("Query: {}", query);
        }
        Commands::Delete { delete_query } => {
            println!("Deleting: {}", delete_query);
        }
    }
}