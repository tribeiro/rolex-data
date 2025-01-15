use chrono::NaiveDateTime;
use lsst_efd_client::EfdAuth;
use reqwest::Client;
use std::error::Error as StdError;
use thiserror::Error;

use crate::{
    efd_utils::efd_utils::QueryResult, sal_script_info::available_scripts::AvailableScript,
};

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{0}")]
struct ErrorRetrievingSalScriptInfo(String);

#[derive(Debug)]
pub struct LogMessage {
    level: u32,
    timestamp: String,
    message: String,
    traceback: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct LogMessagesSeries {
    name: String,
    columns: Vec<String>,
    values: Vec<(String, u32, String, String)>,
}

impl LogMessagesSeries {
    pub fn into_log_messages(&self) -> Vec<LogMessage> {
        self.values
            .iter()
            .map(|(timestamp, level, message, traceback)| LogMessage {
                timestamp: timestamp.to_string(),
                level: *level,
                message: message.to_string(),
                traceback: traceback.to_string(),
            })
            .collect()
    }
}
#[derive(Debug)]
/// Store summary information about a single SAL Script.
pub struct SalScriptInfo {
    log_messages: Vec<LogMessage>,
}

impl SalScriptInfo {
    pub async fn retrieve(
        efd_name: &str,
        script: &AvailableScript,
    ) -> Result<SalScriptInfo, Box<dyn StdError>> {
        let efd_auth = EfdAuth::new(efd_name).await?;

        let influxdb_url = format!(
            "https://{}:{}/influxdb/query",
            efd_auth.get_host(),
            efd_auth.get_port(),
        );
        // Create a reqwest client
        let client = Client::new();

        let sal_index = script.sal_index;
        let parse_from_str = chrono::NaiveDateTime::parse_from_str;

        let date_start = parse_from_str(&script.timestamp, "%Y-%m-%dT%H:%M:%S%.fZ")?;
        let date_end = date_start + chrono::Duration::hours(1);
        let query = format!(
            r#"SELECT "level", "message", "traceback" FROM "efd"."autogen"."lsst.sal.Script.logevent_logMessage" WHERE time > '{date_start}' AND time < '{date_end}' AND salIndex = {sal_index}"#
        );

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
            let query_result: QueryResult<LogMessagesSeries> = serde_json::from_str(&text)?;
            Ok(SalScriptInfo {
                log_messages: query_result.results[0].series[0].into_log_messages(),
            })
        } else {
            println!("{response:?}");
            Err(Box::new(ErrorRetrievingSalScriptInfo(format!(
                "Error: {:?}",
                response
            ))))
        }
    }
}
