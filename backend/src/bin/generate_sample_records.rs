use mysql::prelude::*;
use mysql::*;
use weekly_vocabulary::vocabulary::{PartOfSpeech, Vocabulary};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://user:password@localhost:3306/db";
    let pool = Pool::new(url)?;

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

    let mut conn = pool.get_conn()?;
    conn.exec_batch(
        r"INSERT INTO vocabulary_list (spelling, meaning)
        VALUES(:spelling, :meaning)",
        vocabulary_list.iter().map(|v| {
            params! {
                "spelling" => v.spelling.clone(),
                "meaning" => v.meaning.clone(),
            }
        }),
    )?;

    Ok(())
}
