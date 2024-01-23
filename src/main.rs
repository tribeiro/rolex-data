mod exposure_log;
mod narrative_log;

#[macro_use]
extern crate serde_derive;

use std::{collections::HashMap, error::Error};
use url::Url;

use crate::{exposure_log::exposure_log::ExposureLog, narrative_log::narrative_log::NarrativeLog};
use chrono;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_url = "https://tucson-teststand.lsst.codes/exposurelog/messages";
    let mut url = Url::parse(base_url).unwrap();

    // Add query parameters
    url.query_pairs_mut()
        .append_pair("min_date_added", "2023-09-13T00:00:00.000000")
        .append_pair("limit", "2");
    println!("{}", url);
    let response = reqwest::get(&url.to_string()).await?;

    let response_text = response.text().await.unwrap();

    println!("{response_text}");
    let exposure_logs: Vec<ExposureLog> = serde_json::from_str(&response_text).unwrap();

    println!("Got {} entries: {:?}", exposure_logs.len(), exposure_logs);

    let params = Some(HashMap::from([
        (
            "min_date_added".to_string(),
            "2023-09-13T00:00:00.000000".to_string(),
        ),
        ("limit".to_string(), "2".to_string()),
    ]));
    let exposure_logs = ExposureLog::retrieve(&base_url, &params).await?;

    println!("Got {} entries: {:?}", exposure_logs.len(), exposure_logs);

    let base_url = "https://tucson-teststand.lsst.codes/narrativelog/messages";
    let mut url = Url::parse(base_url).unwrap();

    // Add query parameters
    url.query_pairs_mut().append_pair("limit", "2");
    println!("{}", url);
    let response = reqwest::get(&url.to_string()).await?;

    let response_text = response.text().await.unwrap();

    println!("{response_text}");
    let narrative_logs: Vec<NarrativeLog> = serde_json::from_str(&response_text).unwrap();

    println!("Got {} entries: {:?}", narrative_logs.len(), narrative_logs);

    let params = Some(HashMap::from([("limit".to_string(), "2".to_string())]));
    let narrative_logs = NarrativeLog::retrieve(base_url, &params).await?;
    println!("Got {} entries: {:?}", narrative_logs.len(), narrative_logs);

    let parse_from_str = chrono::NaiveDateTime::parse_from_str;

    let date_start = parse_from_str("2024-01-18T12:00:00", "%Y-%m-%dT%H:%M:%S")?;
    let date_end = date_start + chrono::Duration::days(1);
    println!("{date_start:?} {date_end:?}");
    Ok(())
}
