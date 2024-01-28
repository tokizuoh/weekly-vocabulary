use mysql::prelude::*;
use mysql::*;
use std::error::Error;
use std::result::Result;
use weekly_vocabulary::vocabulary::{PartOfSpeech, Vocabulary};

fn main() -> Result<(), Box<dyn Error>> {
    let url = "mysql://user:password@localhost:3306/db";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    conn.query_drop("TRUNCATE TABLE vocabulary_list")?;

    let vocabulary_list = vec![
        Vocabulary {
            part_of_speech: PartOfSpeech::Noun,
            spelling: "apple".to_string(),
            meaning: "a round fruit with firm, white flesh and a green or red skin".to_string(),
        },
        Vocabulary {
            part_of_speech: PartOfSpeech::Noun,
            spelling: "banana".to_string(),
            meaning: "a long fruit with yello skin and soft".to_string(),
        },
        Vocabulary {
            part_of_speech: PartOfSpeech::Noun,
            spelling: "citrus".to_string(),
            meaning: "any of a group plants that produce acidic fruits".to_string(),
        },
        Vocabulary {
            part_of_speech: PartOfSpeech::Noun,
            spelling: "dragon fruit".to_string(),
            meaning: "a fruit with red or yellow skin and white or red flesh with many black seeds"
                .to_string(),
        },
        Vocabulary {
            part_of_speech: PartOfSpeech::Noun,
            spelling: "eggplant".to_string(),
            meaning: "a vegetable with oval, purple skin and white flesh".to_string(),
        },
    ];

    conn.exec_batch(
        r"INSERT INTO vocabulary_list (spelling, meaning, part_of_speech)
        VALUES(:spelling, :meaning, :part_of_speech)",
        vocabulary_list.iter().map(|v| {
            params! {
                "spelling" => v.spelling.clone(),
                "meaning" => v.meaning.clone(),
                "part_of_speech" => match v.part_of_speech {
                    PartOfSpeech::Noun => {"noun"},
                    PartOfSpeech::Pronoun => {"pronoun"},
                    PartOfSpeech::Adjective => {"adjectiv"},
                    PartOfSpeech::Verb => {"verb"},
                    PartOfSpeech::Adverb => {"adverb"},
                    PartOfSpeech::Preposition => {"preposition"},
                    PartOfSpeech::Conjunction => {"conjunction"},
                    PartOfSpeech::Interjection => {"interjection"},
                }
            }
        }),
    )?;

    Ok(())
}
