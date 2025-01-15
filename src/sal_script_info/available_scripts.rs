use chrono::NaiveDateTime;
use lsst_efd_client::EfdAuth;
use reqwest::Client;
use std::{collections::HashMap, error::Error as StdError};
use thiserror::Error;

use crate::efd_utils::efd_utils::QueryResult;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{0}")]
struct ErrorRetrievingAvailableScripts(String);

#[derive(Debug, Deserialize, Serialize, Default)]
struct Series {
    name: String,
    columns: Vec<String>,
    values: Vec<(String, u32, String)>,
}

impl Series {
    pub fn into_available_scripts(
        &self,
        script_state: &HashMap<u32, ScriptState>,
    ) -> Vec<AvailableScript> {
        self.values
            .iter()
            .map(|(timestamp, sal_index, classname)| AvailableScript {
                sal_index: *sal_index,
                state: script_state
                    .get(sal_index)
                    .unwrap_or(&ScriptState::Unknown)
                    .clone(),
                class_name: classname.to_string(),
                timestamp: timestamp.to_string(),
            })
            .collect()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScriptState {
    Unknown,
    Unconfigured,
    Configured,
    Running,
    Paused,
    Ending,
    Stopping,
    Failing,
    Done,
    Stopped,
    Failed,
    ConfigureFailed,
}

impl ScriptState {
    fn from_u32(val: &u32) -> ScriptState {
        match val {
            1 => ScriptState::Unconfigured,
            2 => ScriptState::Configured,
            3 => ScriptState::Running,
            4 => ScriptState::Paused,
            5 => ScriptState::Ending,
            6 => ScriptState::Stopping,
            7 => ScriptState::Failing,
            8 => ScriptState::Done,
            9 => ScriptState::Stopped,
            10 => ScriptState::Failed,
            11 => ScriptState::ConfigureFailed,
            _ => ScriptState::Unknown,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct ScriptStateSeries {
    name: String,
    columns: Vec<String>,
    values: Vec<(String, u32, u32)>,
}

impl ScriptStateSeries {
    pub fn into_script_state(&self) -> HashMap<u32, ScriptState> {
        self.values
            .iter()
            .map(|(_, sal_index, state_value)| (*sal_index, ScriptState::from_u32(state_value)))
            .collect()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvailableScript {
    pub sal_index: u32,
    pub class_name: String,
    pub state: ScriptState,
    pub timestamp: String,
}

impl AvailableScript {
    pub async fn retrieve(
        efd_name: &str,
        date_start: &NaiveDateTime,
        date_end: &NaiveDateTime,
    ) -> Result<Vec<AvailableScript>, Box<dyn StdError>> {
        let efd_auth = EfdAuth::new(efd_name).await?;

        let influxdb_url = format!(
            "https://{}:{}/influxdb/query",
            efd_auth.get_host(),
            efd_auth.get_port(),
        );
        // Create a reqwest client
        let client = Client::new();

        let query = format!(
            r#"SELECT "salIndex", "classname" FROM "efd"."autogen"."lsst.sal.Script.logevent_description" WHERE time > '{date_start}' AND time < '{date_end}'"#
        );

        // Construct the full URL with query parameters
        let response = client
            .get(&influxdb_url)
            .basic_auth(efd_auth.get_username(), Some(efd_auth.get_password()))
            .query(&[("db", "efd"), ("q", &query)])
            .send()
            .await?; // Check the status code

        if response.status().is_success() {
            // Parse the response JSON
            let text = response.text().await?;
            let query_result: QueryResult<Series> = serde_json::from_str(&text)?;

            let query = format!(
                r#"SELECT "salIndex", "state" FROM "efd"."autogen"."lsst.sal.Script.logevent_state" WHERE time > '{date_start}' AND time < '{date_end}'"#
            );
            let response = client
                .get(&influxdb_url)
                .basic_auth(efd_auth.get_username(), Some(efd_auth.get_password()))
                .query(&[("db", "efd"), ("q", &query)])
                .send()
                .await?; // Check the status code

            let script_state = {
                if response.status().is_success() {
                    let text = response.text().await?;
                    let query_result: QueryResult<ScriptStateSeries> = serde_json::from_str(&text)?;
                    query_result.results[0].series[0].into_script_state()
                } else {
                    HashMap::new()
                }
            };
            let available_scripts =
                query_result.results[0].series[0].into_available_scripts(&script_state);
            Ok(available_scripts)
        } else {
            println!("{response:?}");
            Err(Box::new(ErrorRetrievingAvailableScripts(format!(
                "Error: {:?}",
                response
            ))))
        }
    }
}
