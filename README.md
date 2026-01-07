# 🦀 Oxi-Crawler: High-Performance Rust Web Scraper

A blazing-fast, concurrent web crawler built with **Rust** and the **Tokio** runtime. This tool demonstrates the power of memory safety and fearlessness concurrency by crawling multiple pages simultaneously using asynchronous channels.

## 🚀 Performance Features
- **Asynchronous Engine:** Utilizes `tokio` for non-blocking I/O.
- **Concurrent Workers:** Uses `mpsc` channels and `tokio::spawn` to visit multiple URLs in parallel.
- **Thread-Safe State:** Implements `Arc<Mutex<HashSet>>` to track visited URLs across threads without data races.
- **Smart Filtering:** Parses HTML using `scraper` to extract and validate unique outbound links.

## 🛠 Tech Stack
- **Language:** Rust (Latest Stable)
- **Runtime:** `tokio` (Asynchronous)
- **Network:** `reqwest`
- **Parsing:** `scraper` (CSS Selectors)
- **UI:** `colored` (Terminal UI)

## 🏁 How to Run
1. Ensure you have the Rust toolchain installed.
2. Clone the repository:
   ```bash
   git clone [https://github.com/your-username/oxi-crawler.git](https://github.com/your-username/oxi-crawler.git)