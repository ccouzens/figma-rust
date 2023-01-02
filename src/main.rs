mod design_tokens;
mod figma_api;

use anyhow::{Context, Result};
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

    let file: figma_api::File = serde_json::from_reader(std::io::stdin())
        .context("Failed to parse Figma API file from stdin")?;

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
