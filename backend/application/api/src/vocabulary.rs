use generated::models::Vocabulary;

pub trait Validatable {
    fn validate(&self) -> bool;
}

impl Validatable for Vocabulary {
    fn validate(&self) -> bool {
        match self.part_of_speech.to_lowercase().as_str() {
            "noun" | "pronoun" | "adjective" | "verb" | "adverb" | "preposition"
            | "conjunction" | "interjection" => true,
            _ => false,
        }
    }
}
