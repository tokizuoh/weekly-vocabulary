mod vocabulary;

pub async fn run() -> String {
    let latest_vocabulary = vocabulary::get_latest_vocabulary().await;

    latest_vocabulary.spelling.clone()
}
