use reqwest::Client;
use scraper::{Html, Selector};
use colored::*;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use tokio::sync::mpsc;
use std::fs::OpenOptions;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
let start_url = "https://en.wikipedia.org/wiki/Timeline_of_quantum_computing_and_communication";

    // Thread-safe set to track visited URLs
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    // Channel for communication between tasks
    let (tx, mut rx) = mpsc::channel(100);

    // Seed the crawler
    tx.send(start_url.to_string()).await?;

    println!("{} Starting Oxicrawler...", "🦀".bright_green());

    let mut count = 0;
    let max_pages = 10;

    while let Some(url) = rx.recv().await {
        if count >= max_pages { break; }

        let visited_clone = Arc::clone(&visited);
        let tx_clone = tx.clone();
        let client_clone = client.clone();

        // Check if visited
        {
            let mut v = visited_clone.lock().unwrap();
            if v.contains(&url) { continue; }
            v.insert(url.clone());
        }

        count += 1;

// SPAWN A CONCURRENT TASK
    tokio::spawn(async move {
        println!("{} Crawling: {}", "🔍".yellow(), url.cyan());

        if let Ok(response) = client_clone.get(&url).send().await {
            if let Ok(body) = response.text().await {
                
                let mut found_links = Vec::new();

                // --- SCOPE START ---
                {
                    let document = Html::parse_document(&body);
                    let selector = Selector::parse("a").unwrap();

                    for element in document.select(&selector) {
                        if let Some(link) = element.value().attr("href") {
                            if link.starts_with("http") {
                                found_links.push(link.to_string());
                            }
                        }
                    }
                } 
                // --- SCOPE END ---

                // 1. Write to the file (Inside the if-let block where found_links is alive!)
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("results.log")
                    .expect("Could not open file");

                for link in &found_links {
                    if let Err(e) = writeln!(file, "{}", link) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }

                // 2. Send links to the channel
                for link in found_links {
                    let _ = tx_clone.send(link).await;
                }
            }
        }
    });
    }

    println!("\n{} Finished crawling {} pages.", "✅".green(), count);
    Ok(())
}