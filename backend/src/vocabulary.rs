use serde::Serialize;

#[derive(Debug, Serialize)]
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

impl PartOfSpeech {
    pub fn text(&self) -> &str {
        match self {
            PartOfSpeech::Noun => "noun",
            PartOfSpeech::Pronoun => "pronoun",
            PartOfSpeech::Adjective => "adjectiv",
            PartOfSpeech::Verb => "verb",
            PartOfSpeech::Adverb => "adverb",
            PartOfSpeech::Preposition => "preposition",
            PartOfSpeech::Conjunction => "conjunction",
            PartOfSpeech::Interjection => "interjection",
        }
    }
}

impl PartOfSpeech {
    pub fn from_string(s: String) -> std::prelude::v1::Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "noun" => Ok(PartOfSpeech::Noun),
            "pronoun" => Ok(PartOfSpeech::Pronoun),
            "adjective" => Ok(PartOfSpeech::Adjective),
            "verb" => Ok(PartOfSpeech::Verb),
            "adverb" => Ok(PartOfSpeech::Adverb),
            "preposition" => Ok(PartOfSpeech::Preposition),
            "conjunction" => Ok(PartOfSpeech::Conjunction),
            "interjection" => Ok(PartOfSpeech::Interjection),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Vocabulary {
    pub part_of_speech: PartOfSpeech,
    pub spelling: String,
    pub meaning: String,
}
