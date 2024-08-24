use std::{env, error::Error};

use url::Url;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Project {
    id: usize,
    #[serde(rename = "self")]
    url: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Status {
    id: usize,
    #[serde(rename = "self")]
    url: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Owner {
    #[serde(rename = "accountId")]
    id: String,
    #[serde(rename = "self")]
    url: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Links {
    #[serde(rename = "self")]
    url: String,
    issues: Vec<String>,
    #[serde(rename = "webLinks")]
    web_links: Vec<String>,
    #[serde(rename = "testPlans")]
    test_plans: Vec<TestPlan>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TestPlan {
    id: usize,
    #[serde(rename = "self")]
    url: String,
    #[serde(rename = "testPlanId")]
    test_plan_id: usize,
    #[serde(rename = "type")]
    test_plan_type: String,
    target: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CustomFields {
    #[serde(rename = "End of Night - TMA El position")]
    tma_elevation_position: String,
    #[serde(rename = "End of Night - TMA Az Position")]
    tma_azimuth_position: String,
    #[serde(rename = "TMA walk around - performed by")]
    tma_walk_around_performed_by: String,
    #[serde(rename = "TMA walk around - comments")]
    tma_walk_around_comments: String,
    #[serde(rename = "TMA walk around done")]
    tma_walk_around_done: bool,
    #[serde(rename = "TMA ready for use?")]
    tma_ready: bool,
    #[serde(rename = "End of Night - Power Supply Status")]
    end_of_night_power_supply: String,
    #[serde(rename = "End of Night - OSS Power Status")]
    end_of_night_oss: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct NightPlan {
    id: usize,
    key: String,
    name: String,
    project: Project,
    #[serde(rename = "jiraProjectVersion")]
    jira_project_version: Option<String>,
    status: Status,
    folder: Option<String>,
    description: Option<String>,
    #[serde(rename = "plannedStartDate")]
    planned_start_date: String,
    #[serde(rename = "plannedEndDate")]
    planned_end_date: String,
    owner: Owner,
    #[serde(rename = "customFields")]
    custom_fields: CustomFields,
    links: Links,
}

// const BASE_URL: &str = "https://api.zephyrscale.smartbear.com/v2/";

impl Status {}

impl NightPlan {
    pub async fn retrieve(
        base_url: &str,
        test_cycle_key: &str,
    ) -> Result<NightPlan, Box<dyn Error>> {
        let token = env::var("ZEPHYR_API_TOKEN")?;
        let client = reqwest::Client::new();
        let endpoint = format!("testcycles/{test_cycle_key}");

        let url = Url::parse(base_url)?.join(&endpoint)?;

        let response = client
            .get(url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response_text = response.text().await?;

        println!("{response_text}");
        let night_plan = serde_json::from_str(&response_text)?;

        Ok(night_plan)
    }

    pub async fn get_status(&self) -> Result<String, Box<dyn Error>> {
        let token = env::var("ZEPHYR_API_TOKEN")?;
        let client = reqwest::Client::new();

        let response = client
            .get(&self.status.url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response_text = response.text().await?;

        Ok(response_text)
    }

    pub async fn get_owner(&self) -> Result<String, Box<dyn Error>> {
        let token = env::var("JIRA_CLOUD_API_TOKEN")?;
        let client = reqwest::Client::new();

        let response = client
            .get(&self.owner.url)
            .header("Authorization", format!("Basic {token}"))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response_text = response.text().await?;

        Ok(response_text)
    }

    pub async fn get_links(&self) -> Result<String, Box<dyn Error>> {
        let token = env::var("ZEPHYR_API_TOKEN")?;
        let client = reqwest::Client::new();

        let response = client
            .get(&self.links.url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response_text = response.text().await?;

        Ok(response_text)
    }
}
