pub enum PartOfSpeech {
    Noun,
    Pronoun,
    Adjective,
    Verb,
    Adverb,
    Preposition,
    Conjunction,
    Interjection,
}

pub struct Vocabulary {
    pub part_of_speech: PartOfSpeech,
    pub spelling: String,
    pub meaning: String,
}

pub async fn get_latest_vocabulary() -> Vocabulary {
    // TODO: get from DB
    Vocabulary {
        part_of_speech: PartOfSpeech::Verb,
        spelling: "transcribe".to_string(),
        meaning: "to write down something ecactly as it was said".to_string(),
    }
}
