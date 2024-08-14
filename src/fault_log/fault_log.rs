use askama::Template;
use chrono::{DateTime, NaiveDateTime, Utc};
use lsst_efd_client::EfdAuth;
use reqwest::{Client, Error, Url};
use std::error::Error as StdError;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize, Default, Template)]
#[template(path = "fault_log.html", ext = "html")]
pub struct FaultLog {
    component: String,
    error_code: i32,
    report: String,
    traceback: String,
    timestamp: String,
}

impl FaultLog {
    pub async fn retrieve(
        component_name: &str,
        date_start: &NaiveDateTime,
        date_end: &NaiveDateTime,
    ) -> Result<FaultLog, Box<dyn StdError>> {
        let efd_auth = EfdAuth::new("tucson_teststand_efd").await?;

        let influxdb_url = format!(
            "https://{}:{}/influxdb/query",
            efd_auth.get_host(),
            efd_auth.get_port(),
        );
        // Create a reqwest client
        let client = Client::new();

        let query = r#"SELECT * FROM "efd"."autogen"."lsst.sal.ATMCS.logevent_errorCode" WHERE errorCode > 0 ORDER BY DESC LIMIT 1"#;
        // Construct the full URL with query parameters
        let response = client
            .get(influxdb_url)
            .basic_auth(efd_auth.get_username(), Some(efd_auth.get_password()))
            .query(&[("db", "efd"), ("q", query)])
            .send()
            .await?; // Check the status code
                     //
        if response.status().is_success() {
            // Parse the response JSON
            let text = response.text().await?;
            println!("Response text: {}", text);
        } else {
            eprintln!("Error: {:?}", response);
        }
        // println!("{ping:?}");
        Ok(FaultLog::default())
    }
}
