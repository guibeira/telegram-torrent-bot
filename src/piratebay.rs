use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub info_hash: String,
    pub leechers: String,
    pub seeders: String,
    pub num_files: String,
    pub size: String,
    pub username: String,
    pub added: String,
    pub status: String,
    pub category: String,
    pub imdb: String,
}

#[derive(Debug)]
pub struct Torrent {
    pub name: String,
    pub magnet_link: String,
    pub seeders: Option<u32>,
    pub leechers: Option<u32>,
}

pub async fn search(query: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
    let body = reqwest::get(format!("https://apibay.org/q.php?q={}", query))
        .await?
        .text()
        .await?;
    parse_piratebay(&body)
}

fn parse_piratebay(content: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
    let entries: Vec<Entry> = serde_json::from_str(content)?;

    let results = entries
        .iter()
        .filter(|entry| {
            entry.id != "0"
                && entry.name != "No results returned"
                && entry.info_hash != "0000000000000000000000000000000000000000"
        })
        .map(|entry| Torrent {
            name: entry.name.clone(),
            magnet_link: format!("magnet:?xt=urn:btih:{}", entry.info_hash),
            seeders: entry.seeders.parse().ok(),
            leechers: entry.leechers.parse().ok(),
        })
        .collect();
    Ok(results)
}
