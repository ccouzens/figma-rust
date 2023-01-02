mod design_tokens;
mod figma_api;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about = "Figma API tooling", long_about = Some("Figma API tooling. Requires a Figma file on stdin."))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Generate design tokens", long_about = Some("Generate design tokens. Not recommended due to limitations of the Figma API"))]
    DesignTokens,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file_or_error: figma_api::FileOrError = serde_json::from_reader(std::io::stdin())
        .context("Failed to parse Figma API file from stdin")?;

    let file = match file_or_error {
        figma_api::FileOrError::File(file) => file,
        figma_api::FileOrError::Err { status, err } => {
            bail!("HTTP {} response from figma: {}", status, err);
        }
    };

    match &args.command {
        Commands::DesignTokens => {
            design_tokens::main(
                &file,
                &mut std::io::stdout().lock(),
                &mut std::io::stderr().lock(),
            )
            .context("Failed to generate design tokens")?;
        }
    }
    Ok(())
}
