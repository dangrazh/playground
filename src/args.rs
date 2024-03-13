pub use clap::{Args, Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author = "Daniel Grass <dani.grass@bluewin.ch>")]
#[command(version = "0.1.0")]
#[command(about, long_about = None)]
/// Minimal Parser for ISO 20022 camt 053 documents
pub struct Arguments {
    #[command(subcommand)]
    pub action: ActionType,
    #[arg(short, long = "file")]
    /// The input file name
    pub filename: PathBuf,
    #[arg(short, long)]
    /// The Path to the file
    pub path: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Extract values of provided tag names from source file into a generated csv file
    Extract(ExtractCommand),
    /// Create a new xml file and filter 'Ntry' tags based on provided tag names and values
    Filter(FilterCommand),
}

#[derive(Debug, Args)]
pub struct ExtractCommand {
    #[arg(required = true, short, long, num_args(1..))]
    /// The tag(s) to be extracted
    pub tags: Vec<String>,
}

#[derive(Debug, Args)]
pub struct FilterCommand {
    /// The tag(s) to be filtered on provided as "Tag=Value" pair(s)
    #[arg(required = true, short, long, num_args(1..), value_parser = parse_key_val::<String, String>)]
    pub criteria: Vec<(String, String)>,
    /// If more than one tag criteria is provided, specify the relation between the tags as "and" or "or"
    /// e.g. `filter -c "Tag1=Value1" "Tag2=Value2" -r and` would mean an 'Ntry' is only kept if both
    /// tag / value pairs are present in the respective 'Ntry'
    #[arg(short, long, default_value = "or")]
    pub relation: Option<String>,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
