#[derive(Debug, Deserialize, Serialize, Default)]
pub struct QueryResult<T> {
    pub results: Vec<Payload<T>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Payload<T> {
    statement_id: usize,
    pub series: Vec<T>,
}
