#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      





#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetRecentGet200Response {
    #[serde(rename = "part_of_speech")]
    pub part_of_speech: String,

    #[serde(rename = "spelling")]
    pub spelling: String,

    #[serde(rename = "meaning")]
    pub meaning: String,

}


impl GetRecentGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(part_of_speech: String, spelling: String, meaning: String, ) -> GetRecentGet200Response {
        GetRecentGet200Response {
            part_of_speech,
            spelling,
            meaning,
        }
    }
}

/// Converts the GetRecentGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetRecentGet200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("part_of_speech".to_string()),
            Some(self.part_of_speech.to_string()),


            Some("spelling".to_string()),
            Some(self.spelling.to_string()),


            Some("meaning".to_string()),
            Some(self.meaning.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetRecentGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetRecentGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub part_of_speech: Vec<String>,
            pub spelling: Vec<String>,
            pub meaning: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetRecentGet200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "part_of_speech" => intermediate_rep.part_of_speech.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "spelling" => intermediate_rep.spelling.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "meaning" => intermediate_rep.meaning.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetRecentGet200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetRecentGet200Response {
            part_of_speech: intermediate_rep.part_of_speech.into_iter().next().ok_or_else(|| "part_of_speech missing in GetRecentGet200Response".to_string())?,
            spelling: intermediate_rep.spelling.into_iter().next().ok_or_else(|| "spelling missing in GetRecentGet200Response".to_string())?,
            meaning: intermediate_rep.meaning.into_iter().next().ok_or_else(|| "meaning missing in GetRecentGet200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetRecentGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GetRecentGet200Response>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetRecentGet200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetRecentGet200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<GetRecentGet200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetRecentGet200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetRecentGet200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



