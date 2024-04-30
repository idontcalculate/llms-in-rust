use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use csv::Reader;
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    println!("Environment variables loaded.");

    let _api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");
    println!("API key retrieved successfully.");

    let path = Path::new("src/Salary_Data.csv");
    let file = File::open(&path).expect(&format!("Couldn't open {}", path.display()));
    println!("CSV file opened successfully.");

    let mut reader = Reader::from_reader(file);
    let mut csv_data = String::new();

    for result in reader.records() {
        let record = result?;
        csv_data.push_str(&record.iter().collect::<Vec<_>>().join(","));
        csv_data.push('\n');
    }

    println!("CSV data loaded successfully. First line: {}", csv_data.lines().next().unwrap_or("No data"));

    while let true = prompt_user().await? {
        // continue prompting user
    }

    Ok(())
}

async fn prompt_user() -> Result<bool, Box<dyn Error>> {
    println!("Enter your prompt (or 'quit' to exit):");
    io::stdout().flush()?;

    let mut user_prompt = String::new();
    io::stdin().read_line(&mut user_prompt)?;
    let user_prompt = user_prompt.trim();

    if user_prompt.eq_ignore_ascii_case("quit") {
        println!("Quit command received, exiting loop.");
        return Ok(false);
    }

    println!("You entered: {}", user_prompt);
    Ok(true)
}
