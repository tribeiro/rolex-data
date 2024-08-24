mod exposure_log;
mod fault_log;
mod narrative_log;

#[macro_use]
extern crate serde_derive;

use rolex::block_log::block_log::BlockLog;
use rolex::night_plan::night_plan::NightPlan;
use std::{collections::HashMap, error::Error};
use url::Url;

use crate::{
    exposure_log::exposure_log::ExposureLog, fault_log::fault_log::FaultLog,
    narrative_log::narrative_log::NarrativeLog,
};
use chrono;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /*
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

    // let base_url: &str = "https://api.zephyrscale.smartbear.com/v2/";
    // let test_cycle_key: &str = "BLOCK-R19";

    // let night_plan = NightPlan::retrieve(base_url, test_cycle_key).await?;
    // println!("NightPlan = {night_plan:?}");

    // let night_plan_status = night_plan.get_status().await?;
    // println!("{night_plan_status}");

    // let night_plan_owner = night_plan.get_owner().await;
    // println!("{night_plan_owner:?}");

    // let night_plan_links = night_plan.get_links().await?;
    // println!("{night_plan_links}");
    */
    let parse_from_str = chrono::NaiveDateTime::parse_from_str;

    let date_start = parse_from_str("2024-08-13T12:00:00", "%Y-%m-%dT%H:%M:%S")?;
    let date_end = date_start + chrono::Duration::days(1);
    println!("{date_start:?} {date_end:?}");

    let fault_logs = FaultLog::retrieve(&date_start, &date_end).await;

    println!("{fault_logs:?}");

    let block_logs = BlockLog::retrieve("summit_efd", &date_start, &date_end).await;

    println!("{block_logs:?}");
    Ok(())
}
