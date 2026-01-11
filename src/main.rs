use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

/// HealthcareText – Healthcare text processing CLI
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Input text file
    #[arg(short, long)]
    input: PathBuf,

    /// Output text file
    #[arg(short, long)]
    output: PathBuf,

    /// Enable medical abbreviation expansion
    #[arg(long)]
    expand_abbrev: bool,

    /// Enable redaction of sensitive data
    #[arg(long)]
    redact: bool,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(&args.input)
        .expect("Failed to read input file");

    let mut processed = content;

    if args.expand_abbrev {
        processed = expand_abbreviations(&processed);
    }

    if args.redact {
        processed = redact_sensitive_data(&processed);
    }

    processed = normalize_formatting(&processed);

    fs::write(&args.output, processed)
        .expect("Failed to write output file");

    println!("✔ Processing complete.");
}

fn expand_abbreviations(text: &str) -> String {
    let replacements = [
        (r"\bbp\b", "blood pressure"),
        (r"\bhr\b", "heart rate"),
        (r"\bpt\b", "patient"),
    ];

    let mut result = text.to_string();
    for (pattern, replacement) in replacements {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    result
}

fn redact_sensitive_data(text: &str) -> String {
    let date_re = Regex::new(r"\b\d{2}/\d{2}/\d{4}\b").unwrap();
    let name_re = Regex::new(r"\b[A-Z][a-z]+ [A-Z][a-z]+\b").unwrap();

    let text = date_re.replace_all(text, "[REDACTED DATE]");
    let text = name_re.replace_all(&text, "[REDACTED NAME]");

    text.to_string()
}

fn normalize_formatting(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
