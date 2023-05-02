use std::error::Error;
use std::fs::{self,File};
use std::io::{BufReader, BufWriter, BufRead, Write};
use regex::Regex;

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
    replacement:String,
    
    #[arg(short='s', long)]
    safe: bool
}


impl Args {
    fn get() -> Result<Self,Box<dyn Error>> {
        let args = Self::parse();
        Ok(args)
    }
}

pub struct Wordy {
    pub file_path: String,
    pub origin: String,
    pub replacement: String,
    pub safe: bool
}

impl Wordy {
    pub fn new() -> Result<Self,Box<dyn Error>> {
        let args = Args::get()?;
        Ok(Self {
            file_path: args.file_path,
            origin: args.origin,
            replacement: args.replacement,
            safe: args.safe
        })
    }

    pub fn process(&self) -> Result<(),Box<dyn Error>>
    {
        let file_in = File::open(&self.file_path).unwrap();
        let file_buf_read = BufReader::new(file_in);

        let file_out = File::create("tmp.txt").unwrap();
        let mut file_buf_write = BufWriter::new(file_out);

        let pattern = Regex::new(&self.origin).unwrap();
        
        for line in file_buf_read.lines()
        {
            let line = line.unwrap();
            let replaced = pattern.replace_all(&line, &self.replacement);

            writeln!(file_buf_write,"{}",replaced).unwrap();
        }

        if ! self.safe {
            fs::rename("tmp.txt", &self.file_path).unwrap();
        }

        Ok(())
    }
}