use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path; // Correct import for Path

use csv::Reader;
use llm_chain::{executor, parameters, prompt, step::Step};

// Correctly use dotenvy crate
use dotenvy::dotenv;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize dotenv
    dotenv().ok();

    // Initialize the Tokio runtime; note that we have to conditionally compile the correct runtime initialization based on the Tokio version.
    #[cfg(feature = "full")]
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    
    #[cfg(not(feature = "full"))]
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(async {
        let exec = executor!().expect("Failed to create executor");

        // Open 'Salary_Data.csv' in the current working directory
        let path = Path::new("Salary_Data.csv");

        let file = File::open(&path)
            .expect(&format!("couldn't open {}", path.display()));
        
        let mut reader = Reader::from_reader(file);

        let mut csv_data = String::new();
        for result in reader.records() {
            let record = result?;
            csv_data.push_str(&record.iter().collect::<Vec<_>>().join(","));
            csv_data.push('\n');
        }

        loop {
            println!("Enter your prompt (or 'quit' to exit):");
            io::stdout().flush()?;

            let mut user_prompt = String::new();
            io::stdin().read_line(&mut user_prompt)?;
            let user_prompt = user_prompt.trim(); // No need to convert back to String

            if user_prompt.eq_ignore_ascii_case("quit") {
                break;
            }

            let prompt_string = format!(
                "You are a data analyst tasked with analyzing a CSV file containing information about individuals, including their Age, Gender, Education Level, Job Title, Years of Experience, Salary. Your goal is to provide clear and concise answers to the given questions based on the data provided.\n\nQuestion: {}\n\nCSV Data:\n{}",
                user_prompt, csv_data
            );

            let step = Step::for_prompt_template(prompt!("{}", &prompt_string));

            let res = step.run(&parameters!(), &exec).await?;
            println!("{}", res.to_immediate().await?.as_content());
        }

        Ok(())
    })
}
