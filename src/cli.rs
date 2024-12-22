use crate::blockchain::blockchain::Blockchain;
use clap::{arg, ArgMatches, Command};
use std::{fs, process::exit};

const COMMANDS_NAME: [&str; 3] = ["printchain", "clear", "addblocks"];

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli, Box<dyn std::error::Error>> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let matches = Command::new("Blockchain-rust-demo")
            .version("0.1")
            .author("guerra.anthony122@gmail.com")
            .about("blockchain in rust - a simple blockchain for learning")
            .subcommand(Command::new(COMMANDS_NAME[0]).about("Print all blocks"))
            .subcommand(Command::new(COMMANDS_NAME[1]).about("Clear the blockchain"))
            .subcommand(
                Command::new(COMMANDS_NAME[2])
                    .about("Add a block to the blockchain")
                    .arg(arg!(<DATA> "The block data")),
            )
            .get_matches();

        self.handle_commands(matches)?;
        Ok(())
    }

    fn handle_commands(&mut self, matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(_) = matches.subcommand_matches(COMMANDS_NAME[0]) {
            self.cmd_print_chain()?;
        } else if let Some(_) = matches.subcommand_matches(COMMANDS_NAME[1]) {
            self.clear_blockchain()?;
        } else if let Some(matches) = matches.subcommand_matches(COMMANDS_NAME[2]) {
            if let Some(data) = matches.get_one::<String>("DATA") {
                self.add_block(data.clone())?;
            } else {
                eprintln!("Missing data for the block");
                exit(1);
            }
        } else {
            eprintln!("Unknown command");
            exit(1);
        }
        Ok(())
    }

    fn cmd_print_chain(&self) -> Result<(), Box<dyn std::error::Error>> {
        for block in self.bc.iter() {
            println!("{:?}", block);
        }
        Ok(())
    }

    fn clear_blockchain(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let db_path = "data/blocks";

        if fs::remove_dir_all(db_path).is_ok() {
            println!("Blockchain cleared.");
        } else {
            eprintln!("Failed to clear blockchain. Directory might not exist.");
        }

        // Reinitialize the database
        self.bc = Blockchain::new()?;
        Ok(())
    }

    fn add_block(&mut self, data: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("Adding block with data: {:?}", data);
        self.bc.add_block(data)?;
        Ok(())
    }
}
