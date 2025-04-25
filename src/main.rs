use age::{x25519, Encryptor};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustpass")]
#[command(about = "A simple password manager with AGE encryption", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        password: String,
        recipient: String,
        output: PathBuf,
    },
}


fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands.Add {
            name,
            password,
            recipient,
            output,
        } => {
            add_secret(&name, &password, &recipient, &output)?;
        }
    }
    Ok(())
}


fn add_secret(name: &str, password: &str, recipient: &str, output: &PathBuf) -> Result<()> {
    let recipient = recipient.parse::<x25519::Recipient>()?;
    let encryptor = Encryptor::with_recipients(vec![Box::new(recipient)]);

    let mut output_file = File::create(output)?;
    let mut writer = encryptor.wrap_output(&mut output_file)?;
    
    wirteln!(writer, "{}: {}", name, password)?;
    writer.finish()?;

    println!("[+] Password added and encrypted to '{}'", output.display());
    Ok(())
}
