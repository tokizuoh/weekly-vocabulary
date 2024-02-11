use api::vocabulary::{PartOfSpeech, Vocabulary};
use dotenv;
use mysql::prelude::*;
use mysql::*;
use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::new(url.as_str());

    let mut conn = pool?.get_conn()?;

    conn.query_drop("TRUNCATE TABLE vocabulary")?;

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
        r"INSERT INTO vocabulary (spelling, meaning, part_of_speech)
        VALUES(:spelling, :meaning, :part_of_speech)",
        vocabulary_list.iter().map(|v| {
            params! {
                "spelling" => v.spelling.clone(),
                "meaning" => v.meaning.clone(),
                "part_of_speech" => v.part_of_speech.text()
            }
        }),
    )?;

    Ok(())
}
