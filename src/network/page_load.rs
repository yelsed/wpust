use color_eyre::eyre::{eyre, Result};
use crossterm::{cursor, execute, terminal};
use serde::Deserialize;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::config::load_config;
use crate::utils::extract_hostname;

// --- Serde structs for PageSpeed Insights API v5 ---

#[derive(Deserialize)]
struct PageSpeedResponse {
    #[serde(rename = "lighthouseResult")]
    lighthouse_result: LighthouseResult,
}

#[derive(Deserialize)]
struct LighthouseResult {
    categories: Categories,
    audits: Audits,
}

#[derive(Deserialize)]
struct Categories {
    performance: CategoryEntry,
}

#[derive(Deserialize)]
struct CategoryEntry {
    score: Option<f64>,
}

#[derive(Deserialize)]
struct Audits {
    #[serde(rename = "first-contentful-paint")]
    first_contentful_paint: AuditEntry,
    #[serde(rename = "largest-contentful-paint")]
    largest_contentful_paint: AuditEntry,
    #[serde(rename = "total-blocking-time")]
    total_blocking_time: AuditEntry,
    #[serde(rename = "cumulative-layout-shift")]
    cumulative_layout_shift: AuditEntry,
    #[serde(rename = "speed-index")]
    speed_index: AuditEntry,
    #[serde(rename = "interactive")]
    time_to_interactive: AuditEntry,
}

#[derive(Deserialize)]
struct AuditEntry {
    score: Option<f64>,
    #[serde(rename = "displayValue")]
    display_value: Option<String>,
}

// --- Rating system ---

enum Rating {
    Good,
    NeedsImprovement,
    Poor,
}

impl Rating {
    fn from_score(score: f64) -> Self {
        if score >= 0.9 {
            Rating::Good
        } else if score >= 0.5 {
            Rating::NeedsImprovement
        } else {
            Rating::Poor
        }
    }

    fn emoji(&self) -> &str {
        match self {
            Rating::Good => "✅",
            Rating::NeedsImprovement => "⚠️ ",
            Rating::Poor => "❌",
        }
    }

    fn label(&self) -> &str {
        match self {
            Rating::Good => "Good",
            Rating::NeedsImprovement => "Needs Improvement",
            Rating::Poor => "Poor",
        }
    }
}

// --- Spinner ---

struct Spinner {
    stop: Arc<AtomicBool>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Spinner {
    fn start(message: &str) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = stop.clone();
        let msg = message.to_string();

        let handle = std::thread::spawn(move || {
            let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let mut i = 0;
            while !stop_clone.load(Ordering::Relaxed) {
                print!("\r{} {}", frames[i % frames.len()], msg);
                let _ = io::stdout().flush();
                i += 1;
                std::thread::sleep(Duration::from_millis(80));
            }
            // Clear the spinner line
            let _ = execute!(
                io::stdout(),
                cursor::MoveToColumn(0),
                terminal::Clear(terminal::ClearType::CurrentLine)
            );
        });

        Spinner {
            stop,
            handle: Some(handle),
        }
    }

    fn stop(self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle {
            let _ = handle.join();
        }
    }
}

// --- Main function ---

pub fn page_load(site: String, strategy: String, api_key: Option<String>) -> Result<()> {
    let strategy = strategy.to_lowercase();
    if strategy != "mobile" && strategy != "desktop" {
        return Err(eyre!(
            "Invalid strategy '{}'. Use 'mobile' or 'desktop'.",
            strategy
        ));
    }

    // Resolve API key: --key flag / env var > config file
    let api_key = api_key.or_else(|| {
        load_config().ok().and_then(|c| c.pagespeed_api_key)
    });

    let hostname = extract_hostname(&site)?;
    let url = format!("https://{}", hostname);

    let spinner = Spinner::start(&format!("Analysing {} ({})...", hostname, strategy));

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()?;

    let mut params = vec![("url", url.as_str()), ("strategy", strategy.as_str())];
    let key_val;
    if let Some(ref key) = api_key {
        key_val = key.clone();
        params.push(("key", key_val.as_str()));
    }

    let response = client
        .get("https://www.googleapis.com/pagespeedonline/v5/runPagespeed")
        .query(&params)
        .send();

    spinner.stop();

    let response = response.map_err(|e| eyre!("API request failed: {}", e))?;

    if response.status() == 429 {
        let mut msg = "Rate limited by Google PageSpeed Insights API.".to_string();
        if api_key.is_none() {
            msg.push_str(
                "\n\nA free API key is required (Google changed the unauthenticated quota to 0).\n\n\
                 Get a key at:\n  \
                 https://developers.google.com/speed/docs/insights/v5/get-started#key\n\n\
                 Then provide it via one of:\n  \
                 wpust page-load example.com --key YOUR_KEY\n  \
                 export PAGESPEED_API_KEY=YOUR_KEY\n  \
                 wpust config  (saves key to config file)"
            );
        } else {
            msg.push_str("\nPlease wait a moment and try again.");
        }
        return Err(eyre!("{}", msg));
    }

    if !response.status().is_success() {
        return Err(eyre!(
            "PageSpeed Insights API returned HTTP {}",
            response.status()
        ));
    }

    let data: PageSpeedResponse = response
        .json()
        .map_err(|e| eyre!("Failed to parse API response: {}", e))?;

    // Performance score
    let perf_score = data
        .lighthouse_result
        .categories
        .performance
        .score
        .unwrap_or(0.0);
    let score_int = (perf_score * 100.0).round() as u32;
    let rating = Rating::from_score(perf_score);

    println!(
        "PageSpeed Insights for: {} ({})\n",
        hostname, strategy
    );
    println!(
        "Performance Score: {} / 100  {}  {}\n",
        score_int,
        rating.emoji(),
        rating.label()
    );
    println!("Core Web Vitals:\n");

    // Print each metric
    let audits = &data.lighthouse_result.audits;
    let metrics: [(&str, &AuditEntry); 6] = [
        ("First Contentful Paint (FCP)", &audits.first_contentful_paint),
        ("Largest Contentful Paint (LCP)", &audits.largest_contentful_paint),
        ("Total Blocking Time (TBT)", &audits.total_blocking_time),
        ("Cumulative Layout Shift (CLS)", &audits.cumulative_layout_shift),
        ("Speed Index", &audits.speed_index),
        ("Time to Interactive (TTI)", &audits.time_to_interactive),
    ];

    for (name, audit) in &metrics {
        let value = audit
            .display_value
            .as_deref()
            .unwrap_or("N/A");
        let metric_rating = audit
            .score
            .map(Rating::from_score)
            .unwrap_or(Rating::Poor);

        println!(
            "  {:<42} {:>10}  {}  {}",
            name,
            value,
            metric_rating.emoji(),
            metric_rating.label()
        );
    }

    println!();
    println!(
        "Full report: https://pagespeed.web.dev/analysis?url={}&strategy={}",
        url, strategy
    );
    println!();

    Ok(())
}
