use askama::Template;
use std::{collections::HashMap, error::Error};
use url::Url;

#[derive(Debug, Deserialize, Serialize, Default, Template)]
#[template(path = "log_entry.html", ext = "html")]
pub struct NarrativeLog {
    id: String,
    site_id: String,
    message_text: String,
    level: usize,
    tags: Vec<String>,
    urls: Vec<String>,
    time_lost: f32,
    date_begin: String,
    user_id: String,
    user_agent: String,
    is_human: bool,
    is_valid: bool,
    date_added: String,
    date_invalidated: Option<String>,
    parent_id: Option<String>,
    systems: Vec<String>,
    subsystems: Vec<String>,
    cscs: Vec<String>,
    date_end: String,
    components: Option<String>,
    primary_software_components: Option<String>,
    primary_hardware_components: Option<String>,
}

impl NarrativeLog {
    pub async fn retrieve(
        url: &str,
        params: &Option<HashMap<String, String>>,
    ) -> Result<Vec<NarrativeLog>, Box<dyn Error>> {
        let url = {
            let mut url = Url::parse(url)?;

            if let Some(params) = params {
                for (key, value) in params {
                    url.query_pairs_mut().append_pair(key, value);
                }
            }
            url
        };

        let response = reqwest::get(&url.to_string()).await?;

        let response_text = response.text().await?;

        let narrative_logs: Vec<NarrativeLog> = serde_json::from_str(&response_text)?;

        Ok(narrative_logs)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize() {
        let narrative_log_json = r#"{"id":"04be0aef-e22a-4742-a5c0-0dab847ec237","site_id":"summit","message_text":"LOVE OLE test from upper panel","level":0,"tags":["observatorysoftwaretools","love"],"urls":[],"time_lost":24.01,"date_begin":"2023-02-19T17:17:09.794000","user_id":"admin@love02.cp.lsst.org","user_agent":"LOVE","is_human":true,"is_valid":true,"date_added":"2023-02-20T17:20:19.169017","date_invalidated":null,"parent_id":null,"systems":["ObservatorySoftwareTools"],"subsystems":["LOVE"],"cscs":[],"date_end":"2023-02-20T17:17:46.794000","components":null,"primary_software_components":null,"primary_hardware_components":null}"#;

        let narrative_log: NarrativeLog = serde_json::from_str(narrative_log_json).unwrap();

        assert_eq!(narrative_log.id, "04be0aef-e22a-4742-a5c0-0dab847ec237");
        assert_eq!(narrative_log.site_id, "summit");
        assert_eq!(narrative_log.message_text, "LOVE OLE test from upper panel");
        assert_eq!(narrative_log.level, 0);

        assert_eq!(narrative_log.tags.len(), 2);
        let val1 = "observatorysoftwaretools".to_owned();
        let val2 = "love".to_owned();
        assert!(narrative_log.tags.contains(&val1));
        assert!(narrative_log.tags.contains(&val2));
        assert!(narrative_log.urls.is_empty());

        assert_eq!(narrative_log.time_lost, 24.01);
        assert_eq!(narrative_log.date_begin, "2023-02-19T17:17:09.794000");
        assert_eq!(narrative_log.user_id, "admin@love02.cp.lsst.org");
        assert_eq!(narrative_log.user_agent, "LOVE");
        assert_eq!(narrative_log.is_human, true);
        assert_eq!(narrative_log.is_valid, true);
        assert_eq!(narrative_log.date_added, "2023-02-20T17:20:19.169017");
        assert_eq!(narrative_log.date_invalidated, None);
        assert_eq!(narrative_log.parent_id, None);
        assert_eq!(narrative_log.date_end, "2023-02-20T17:17:46.794000");
        assert_eq!(narrative_log.components, None);
        assert_eq!(narrative_log.primary_software_components, None);
        assert_eq!(narrative_log.primary_hardware_components, None);
    }

    #[test]
    fn test_template() {
        let narrative_log_json = r#"{"id":"04be0aef-e22a-4742-a5c0-0dab847ec237","site_id":"summit","message_text":"LOVE OLE test from upper panel","level":0,"tags":["observatorysoftwaretools","love"],"urls":[],"time_lost":24.01,"date_begin":"2023-02-19T17:17:09.794000","user_id":"admin@love02.cp.lsst.org","user_agent":"LOVE","is_human":true,"is_valid":true,"date_added":"2023-02-20T17:20:19.169017","date_invalidated":null,"parent_id":null,"systems":["ObservatorySoftwareTools"],"subsystems":["LOVE"],"cscs":[],"date_end":"2023-02-20T17:17:46.794000","components":null,"primary_software_components":null,"primary_hardware_components":null}"#;

        let narrative_log: NarrativeLog = serde_json::from_str(narrative_log_json).unwrap();

        let template = narrative_log.render().unwrap();

        assert_eq!(template, "\n\t<span class=\"score\">\n\t\t<p>\n      \\u{1F4CE}\n\t\t</p>\n\t</span>\n\t<span class=\"title\">\n\t\t\t<a>\n        LOVE OLE test from upper panel\n\t\t\t</a>\n\t</span>\n\t<span class=\"meta\">\n    <a>\n      admin@love02.cp.lsst.org 2023-02-20T17:20:19.169017\n    </a>\n\t</span>")
    }
}
