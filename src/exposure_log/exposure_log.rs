
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ExposureLog {
    id: String,
    site_id: String,
    obs_id: String,
    instrument: String,
    day_obs: usize,
    seq_num: usize,
    message_text: String,
    level: usize,
    tags: Vec<String>,
    urls: Vec<String>,
    user_id: String,
    user_agent: String,
    is_human: bool,
    is_valid: bool,
    exposure_flag: String,
    date_added: Option<String>,
    date_invalidated: Option<String>,
    parent_id: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize() {
        let exposure_log_json = r#"{"id":"000f68b2-e560-40ce-bdbc-a57b3363e1e9","site_id":"summit","obs_id":"AT_O_20220608_000168","instrument":"LATISS","day_obs":20220608,"seq_num":168,"message_text":"","level":20,"tags":[],"urls":[],"user_id":"slimleashma","user_agent":"notebook:nublado","is_human":true,"is_valid":true,"exposure_flag":"junk","date_added":"2022-06-08T23:19:38.906593","date_invalidated":null,"parent_id":null}"#;

        let exposure_log: ExposureLog = serde_json::from_str(exposure_log_json).unwrap();

        assert_eq!(exposure_log.id, "000f68b2-e560-40ce-bdbc-a57b3363e1e9");
        assert_eq!(exposure_log.site_id, "summit");
        assert_eq!(exposure_log.obs_id, "AT_O_20220608_000168");
        assert_eq!(exposure_log.instrument, "LATISS");
        assert_eq!(exposure_log.day_obs, 20220608);
        assert_eq!(exposure_log.seq_num, 168);
        assert_eq!(exposure_log.message_text, "");
        assert_eq!(exposure_log.level, 20);
        assert!(exposure_log.tags.is_empty());
        assert!(exposure_log.urls.is_empty());
        assert_eq!(exposure_log.user_id, "slimleashma");
        assert_eq!(exposure_log.user_agent, "notebook:nublado");
        assert_eq!(exposure_log.is_human, true);
        assert_eq!(exposure_log.is_valid, true);
        assert_eq!(exposure_log.exposure_flag, "junk");
        assert_eq!(
            exposure_log.date_added.unwrap(),
            "2022-06-08T23:19:38.906593"
        );
        assert_eq!(exposure_log.date_invalidated, None);
        assert_eq!(exposure_log.parent_id, None);
    }
}
