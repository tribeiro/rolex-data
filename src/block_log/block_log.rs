use askama::Template;
use chrono::{DateTime, NaiveDateTime};
use lsst_efd_client::EfdAuth;
use reqwest::Client;
use std::error::Error as StdError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{0}")]
struct ErrorRetrievingFaultLog(String);

#[derive(Debug, Deserialize, Serialize, Default, Template)]
#[template(path = "block_log.html", ext = "html")]
pub struct BlockLog {
    time: String,
    id: String,
    status: String,
    hash: String,
    sal_index: usize,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct QueryResult {
    results: Vec<Payload>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Payload {
    statement_id: usize,
    series: Vec<Series>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Series {
    name: String,
    columns: Vec<String>,
    values: Vec<(String, String, String, String, usize)>,
}

impl Series {
    fn into_fault_log(&self) -> Vec<BlockLog> {
        self.values
            .iter()
            .map(|(time, id, status, hash, sal_index)| BlockLog {
                time: time.to_string(),
                id: id.to_string(),
                status: status.to_string(),
                hash: hash.to_string(),
                sal_index: *sal_index,
            })
            .collect()
    }
}

impl BlockLog {
    pub fn get_date_added(&self) -> &str {
        &self.time
    }

    pub fn get_index_label(&self) -> String {
        match self.sal_index {
            1 => "Maintel".to_owned(),
            2 => "AuxTel".to_owned(),
            3 => "OCS".to_owned(),
            _ => format!("Unknown[{}]", self.sal_index),
        }
    }

    pub async fn retrieve(
        efd_name: &str,
        date_start: &NaiveDateTime,
        date_end: &NaiveDateTime,
    ) -> Result<Vec<BlockLog>, Box<dyn StdError>> {
        let efd_auth = EfdAuth::new(efd_name).await?;

        let influxdb_url = format!(
            "https://{}:{}/influxdb/query",
            efd_auth.get_host(),
            efd_auth.get_port(),
        );
        // Create a reqwest client
        let client = Client::new();

        let query = format!(
            r#"SELECT "id", "status", "hash", "salIndex" FROM "efd"."autogen"."lsst.sal.Scheduler.logevent_blockStatus" WHERE time > '{date_start}' AND time < '{date_end}'"#
        );
        println!("{query}");
        // Construct the full URL with query parameters
        let response = client
            .get(influxdb_url)
            .basic_auth(efd_auth.get_username(), Some(efd_auth.get_password()))
            .query(&[("db", "efd"), ("q", &query)])
            .send()
            .await?; // Check the status code
        if response.status().is_success() {
            // Parse the response JSON
            let text = response.text().await?;
            let query_result: QueryResult = serde_json::from_str(&text)?;
            Ok(query_result.results[0].series[0].into_fault_log())
        } else {
            println!("{response:?}");
            Err(Box::new(ErrorRetrievingFaultLog(format!(
                "Error: {:?}",
                response
            ))))
        }
        // println!("{ping:?}");
    }
}
