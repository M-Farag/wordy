use clap::Parser;

#[derive(Parser,Debug)]
#[command(
    name = "Wordy",
    version = "0.0.1",
    about = "A simple command line tool to replace a string in a file"
)]
pub struct Args {
    
    #[arg(short='f', long)]
    file_path: String,

    #[arg(short='o', long)]
    origin:String,

    #[arg(short='r', long)]
    replacement:String
}


impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}