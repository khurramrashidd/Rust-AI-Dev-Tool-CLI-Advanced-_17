use clap::{Parser, Subcommand};
use colored::*;
use reqwest::blocking::Client;
use serde_json::json;
use std::{env, error::Error, fs, time::Duration};
use dotenv::dotenv;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "AI Dev Tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        #[arg(short, long)]
        path: String,
    },
    Summary {
        #[arg(short, long)]
        path: String,
    },
    Fix {
        #[arg(short, long)]
        path: String,

        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let cli = Cli::parse();

    let api_key = env::var("GEMINI_API_KEY")
        .expect("API key not found in .env");

    match cli.command {
        Commands::Analyze { path } => run(&path, &api_key, "analyze", None)?,
        Commands::Summary { path } => run(&path, &api_key, "summary", None)?,
        Commands::Fix { path, output } => run(&path, &api_key, "fix", output)?,
    }

    Ok(())
}

fn run(
    path: &str,
    api_key: &str,
    mode: &str,
    output: Option<String>,
) -> Result<(), Box<dyn Error>> {
    println!("{}", "🚀 Running AI Dev Tool...".green());

    let mut combined_code = String::new();

    let allowed = ["rs", "py", "js", "ts", "java", "cpp", "c"];

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let file_path = entry.path();

        if file_path.to_string_lossy().contains("target")
            || file_path.to_string_lossy().contains(".git")
        {
            continue;
        }

        if file_path.is_file() {
            if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                if allowed.contains(&ext) {
                    if let Ok(content) = fs::read_to_string(file_path) {
                        combined_code.push_str(&format!(
                            "\n// FILE: {}\n{}\n",
                            file_path.display(),
                            content
                        ));
                    }
                }
            }
        }
    }

    if combined_code.is_empty() {
        println!("{}", "❌ No valid files found.".red());
        return Ok(());
    }

    let trimmed = if combined_code.len() > 15000 {
        &combined_code[..15000]
    } else {
        &combined_code
    };

    let prompt = match mode {
        "summary" => format!("Summarize this project:\n\n{}", trimmed),
        "fix" => format!(
            "Fix bugs and improve this code. Return improved code:\n\n{}",
            trimmed
        ),
        _ => format!(
            "Analyze this project and provide summary, issues, improvements:\n\n{}",
            trimmed
        ),
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent?key={}",
        api_key
    );

    let body = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let res = client.post(&url).json(&body).send();

    let res = match res {
        Ok(r) => r,
        Err(e) => {
            println!("{}", format!("❌ Error: {}", e).red());
            return Ok(());
        }
    };

    let json: serde_json::Value = res.json()?;

    let reply = json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("No response");

    println!("\n{}", "🧠 AI Output:".blue());
    println!("{}", reply);

    // 💾 Save output if provided
    if let Some(file) = output {
        fs::write(&file, reply)?;
        println!("{}", format!("💾 Saved to {}", file).yellow());
    }

    Ok(())
}