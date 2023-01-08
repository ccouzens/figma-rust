mod design_tokens;
mod figma_api;
mod typescript_props;

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
    #[command(name = "typescript-props", about = "Generate TypeScript props for the components", long_about = None)]
    TypeScriptProps,
    #[command(about = "Echo the JSON back", long_about = None)]
    Echo,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file_or_error: serde_json::Value = serde_json::from_reader(std::io::stdin())
        .context("Failed to parse Figma API JSON from stdin")?;
    if let (Some(err), Some(status)) = (
        file_or_error.get("err").and_then(|e| e.as_str()),
        file_or_error.get("status").and_then(|s| s.as_u64()),
    ) {
        bail!("HTTP {} response from figma: {}", status, err);
    }

    match file_or_error
        .get("schemaVersion")
        .and_then(serde_json::Value::as_i64)
    {
        Some(0) => {}
        _ => {
            bail!(
                r#"Compatible with "schemaVersion": 0, got {:?}"#,
                file_or_error.get("schemaVersion")
            );
        }
    }

    let file: figma_api::File =
        serde_json::from_value(file_or_error).context("Failed to parse JSON as Figma API")?;

    match &args.command {
        Commands::DesignTokens => {
            design_tokens::main(
                &file,
                &mut std::io::stdout().lock(),
                &mut std::io::stderr().lock(),
            )
            .context("Failed to generate design tokens")?;
        }
        Commands::TypeScriptProps => {
            typescript_props::main(
                &file,
                &mut std::io::stdout().lock(),
                &mut std::io::stderr().lock(),
            )
            .context("Failed to generate TypeScript props")?;
        }
        Commands::Echo => {
            serde_json::to_writer_pretty(std::io::stdout().lock(), &file)
                .context("Failed to echo JSON")?;
        }
    }
    Ok(())
}
