use reqwest::Client;
use scraper::{Html, Selector};
use colored::*;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_url = "https://www.rust-lang.org";
    
    // Thread-safe set to track visited URLs
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let client = Client::new();
    
    // Channel for communication between tasks
    let (tx, mut rx) = mpsc::channel(100);

    // Seed the crawler
    tx.send(start_url.to_string()).await?;

    println!("{} Starting Oxicrawler...", "🦀".bright_green());

    let mut count = 0;
    let max_pages = 10; // Limit for safety

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
                    
                    // Create a list for links outside the scope
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
                    // --- SCOPE END: 'document' and 'selector' are dropped here ---

                    // Now 'document' is gone, so we can safely .await
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