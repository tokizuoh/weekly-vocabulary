pub struct Vocabulary {
    pub spelling: String,
    pub meaning: String,
}

pub async fn get_latest_vocabulary() -> Vocabulary {
    // TODO: get from DB
    Vocabulary {
        spelling: "transcribe".to_string(),
        meaning: "to write down something ecactly as it was said".to_string(),
    }
}
