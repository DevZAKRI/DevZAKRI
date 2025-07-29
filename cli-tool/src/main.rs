// CLI Tool Example for JavaScript Developers
// This demonstrates file operations and JSON processing in Rust
// Run with: cargo run -- --help

use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Command line interface definition (like commander.js in Node.js)
#[derive(Parser)]
#[command(name = "dev-tool")]
#[command(about = "A CLI tool by DevZAKRI - demonstrating Rust for JS developers")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new JSON file with sample data
    Create {
        /// Name of the file to create
        #[arg(short, long)]
        filename: String,
    },
    /// Read and display a JSON file
    Read {
        /// Name of the file to read
        #[arg(short, long)]
        filename: String,
    },
    /// List all JSON files in current directory
    List,
    /// Convert JSON to pretty formatted output
    Pretty {
        /// Name of the file to format
        #[arg(short, long)]
        filename: String,
    },
    /// Analyze a JSON file and show statistics
    Analyze {
        /// Name of the file to analyze
        #[arg(short, long)]
        filename: String,
    },
}

// Data structures (like TypeScript interfaces)
#[derive(Serialize, Deserialize, Debug)]
struct ProjectInfo {
    name: String,
    version: String,
    author: String,
    description: String,
    technologies: Vec<String>,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileStats {
    filename: String,
    size_bytes: u64,
    line_count: usize,
    technologies_count: usize,
}

fn main() {
    println!("{}", "🦀 DevZAKRI's Rust CLI Tool".bright_blue().bold());
    println!("{}", "Demonstrating file operations for JS developers".cyan());
    println!();

    let cli = Cli::parse();

    match cli.command {
        Commands::Create { filename } => create_sample_file(&filename),
        Commands::Read { filename } => read_json_file(&filename),
        Commands::List => list_json_files(),
        Commands::Pretty { filename } => pretty_print_json(&filename),
        Commands::Analyze { filename } => analyze_json_file(&filename),
    }
}

fn create_sample_file(filename: &str) {
    let sample_data = ProjectInfo {
        name: "Rust Learning Project".to_string(),
        version: "1.0.0".to_string(),
        author: "DevZAKRI".to_string(),
        description: "Learning Rust coming from JavaScript background".to_string(),
        technologies: vec![
            "Rust".to_string(),
            "Axum".to_string(),
            "Tokio".to_string(),
            "Serde".to_string(),
            "Clap".to_string(),
        ],
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    match serde_json::to_string_pretty(&sample_data) {
        Ok(json_string) => {
            match fs::write(&filename, json_string) {
                Ok(_) => {
                    println!("{} {}", "✅ Successfully created:".green(), filename.bright_white());
                    println!("{}", "File contains sample project data".dimmed());
                }
                Err(e) => {
                    println!("{} {}", "❌ Error creating file:".red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} {}", "❌ Error serializing data:".red(), e);
        }
    }
}

fn read_json_file(filename: &str) {
    if !Path::new(filename).exists() {
        println!("{} File '{}' not found", "❌".red(), filename.bright_white());
        return;
    }

            match fs::read_to_string(filename) {
        Ok(content) => {
            println!("{} {}", "📖 Reading file:".blue(), filename.bright_white());
            println!("{}", "─".repeat(50).dimmed());
            
            // Try to parse as ProjectInfo first
            match serde_json::from_str::<ProjectInfo>(&content) {
                Ok(project) => {
                    println!("{} {}", "Project Name:".cyan(), project.name.bright_white());
                    println!("{} {}", "Version:".cyan(), project.version.bright_white());
                    println!("{} {}", "Author:".cyan(), project.author.bright_white());
                    println!("{} {}", "Description:".cyan(), project.description.bright_white());
                    println!("{}", "Technologies:".cyan());
                    for tech in &project.technologies {
                        println!("  • {}", tech.bright_green());
                    }
                    println!("{} {}", "Created:".cyan(), project.created_at.bright_white());
                }
                Err(_) => {
                    // If it's not a ProjectInfo, just print the raw JSON
                    println!("{}", content);
                }
            }
        }
        Err(e) => {
            println!("{} {}", "❌ Error reading file:".red(), e);
        }
    }
}

fn list_json_files() {
    println!("{}", "📁 JSON files in current directory:".blue());
    println!("{}", "─".repeat(40).dimmed());

    match fs::read_dir(".") {
        Ok(entries) => {
            let mut json_files = Vec::new();
            
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "json" {
                            if let Some(filename) = path.file_name() {
                                if let Some(filename_str) = filename.to_str() {
                                    json_files.push(filename_str.to_string());
                                }
                            }
                        }
                    }
                }
            }

            if json_files.is_empty() {
                println!("{}", "No JSON files found".dimmed());
            } else {
                for (index, file) in json_files.iter().enumerate() {
                    println!("{}. {}", (index + 1).to_string().green(), file.bright_white());
                }
                println!();
                println!("{} {} files found", "📊".blue(), json_files.len().to_string().bright_green());
            }
        }
        Err(e) => {
            println!("{} {}", "❌ Error reading directory:".red(), e);
        }
    }
}

fn pretty_print_json(filename: &str) {
    if !Path::new(filename).exists() {
        println!("{} File '{}' not found", "❌".red(), filename.bright_white());
        return;
    }

    match fs::read_to_string(filename) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(json_value) => {
                    match serde_json::to_string_pretty(&json_value) {
                        Ok(pretty_json) => {
                            println!("{} {}", "✨ Pretty JSON for:".blue(), filename.bright_white());
                            println!("{}", "─".repeat(50).dimmed());
                            println!("{}", pretty_json);
                        }
                        Err(e) => {
                            println!("{} {}", "❌ Error formatting JSON:".red(), e);
                        }
                    }
                }
                Err(e) => {
                    println!("{} Invalid JSON format: {}", "❌".red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} {}", "❌ Error reading file:".red(), e);
        }
    }
}

fn analyze_json_file(filename: &str) {
    if !Path::new(filename).exists() {
        println!("{} File '{}' not found", "❌".red(), filename.bright_white());
        return;
    }

    match fs::read_to_string(filename) {
        Ok(content) => {
            // Calculate file statistics
            let file_size = content.len() as u64;
            let line_count = content.lines().count();
            
            // Try to parse as ProjectInfo to get tech count
            let tech_count = match serde_json::from_str::<ProjectInfo>(&content) {
                Ok(project) => project.technologies.len(),
                Err(_) => 0,
            };

            let stats = FileStats {
                filename: filename.to_string(),
                size_bytes: file_size,
                line_count,
                technologies_count: tech_count,
            };

            println!("{} {}", "📊 File Analysis:".blue(), filename.bright_white());
            println!("{}", "─".repeat(40).dimmed());
            println!("{} {} bytes", "Size:".cyan(), stats.size_bytes.to_string().bright_green());
            println!("{} {} lines", "Lines:".cyan(), stats.line_count.to_string().bright_green());
            
            if stats.technologies_count > 0 {
                println!("{} {} technologies", "Technologies:".cyan(), stats.technologies_count.to_string().bright_green());
            }

            // Validate JSON structure
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(_) => {
                    println!("{} {}", "JSON Status:".cyan(), "✅ Valid".bright_green());
                }
                Err(e) => {
                    println!("{} {}", "JSON Status:".cyan(), format!("❌ Invalid: {}", e).bright_red());
                }
            }
        }
        Err(e) => {
            println!("{} {}", "❌ Error reading file:".red(), e);
        }
    }
}
