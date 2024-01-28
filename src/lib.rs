mod vocabulary;

pub async fn run() -> String {
    vocabulary::get_vocabulary().await
}
