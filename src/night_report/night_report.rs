use std::{collections::HashMap, error::Error};

pub struct NightReport {
    pub id: String,
    pub site_id: String,
    pub telescope: String,
    pub day_obs: u32,
    pub summary: String,
    pub telescope_status: String,
    pub confluence_url: String,
    pub user_id: String,
    pub user_agent: String,
    pub date_added: String,
    pub date_sent: String,
    pub is_valid: bool,
    pub date_invalidated: String,
    pub parent_id: String,
    pub observers_crew: Vec<String>,
}

impl NightReport {
    pub async fn retrieve(
        url: &str,
        params: &Option<HashMap<String, String>>,
    ) -> Result<Vec<NightReport>, Box<dyn Error>> {
        Ok(vec![])
    }
}
