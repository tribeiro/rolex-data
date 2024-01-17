mod exposure_log;
mod narrative_log;

#[macro_use]
extern crate serde_derive;

use reqwest::Error;
use url::Url;

use crate::{exposure_log::exposure_log::ExposureLog, narrative_log::narrative_log::NarrativeLog};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let base_url = "http://summit-lsp.lsst.codes/exposurelog/messages";
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

    // let exposure_logs: Vec<ExposureLog> = ;
    println!("Got {} entries: {:?}", exposure_logs.len(), exposure_logs);

    let base_url = "http://summit-lsp.lsst.codes/narrativelog/messages";
    let mut url = Url::parse(base_url).unwrap();

    // Add query parameters
    url.query_pairs_mut()
        .append_pair("limit", "2");
    println!("{}", url);
    let response = reqwest::get(&url.to_string()).await?;

    let response_text = response.text().await.unwrap();

    println!("{response_text}");
    let narrative_logs: Vec<NarrativeLog> = serde_json::from_str(&response_text).unwrap();

    // let exposure_logs: Vec<ExposureLog> = ;
    println!("Got {} entries: {:?}", narrative_logs.len(), narrative_logs);

    Ok(())
}
