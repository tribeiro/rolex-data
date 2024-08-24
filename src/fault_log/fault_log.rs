use askama::Template;
use chrono::NaiveDateTime;
use lsst_efd_client::EfdAuth;
use reqwest::Client;
use std::error::Error as StdError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{0}")]
struct ErrorRetrievingFaultLog(String);

#[derive(Debug, Deserialize, Serialize, Default, Template)]
#[template(path = "fault_log.html", ext = "html")]
pub struct FaultLog {
    name: String,
    severity: usize,
    reason: String,
    time: String,
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
    values: Vec<(String, String, String, usize)>,
}

impl Series {
    fn into_fault_log(&self) -> Vec<FaultLog> {
        self.values
            .iter()
            .map(|(time, name, reason, severity)| FaultLog {
                name: name.to_string(),
                severity: *severity,
                reason: reason.to_string(),
                time: time.to_string(),
            })
            .collect()
    }
}

impl FaultLog {
    pub async fn retrieve(
        date_start: &NaiveDateTime,
        date_end: &NaiveDateTime,
    ) -> Result<Vec<FaultLog>, Box<dyn StdError>> {
        let efd_auth = EfdAuth::new("base_efd").await?;

        let influxdb_url = format!(
            "https://{}:{}/influxdb/query",
            efd_auth.get_host(),
            efd_auth.get_port(),
        );
        // Create a reqwest client
        let client = Client::new();

        let query = format!(
            r#"SELECT "time","name","reason","severity" FROM "efd"."autogen"."lsst.sal.Watcher.logevent_alarm" WHERE time > '{date_start}' AND time < '{date_end}'"#
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
