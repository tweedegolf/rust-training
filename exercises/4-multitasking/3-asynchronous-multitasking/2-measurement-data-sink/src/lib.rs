#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Measurement {
    pub node_id: usize,
    pub room_id: usize,
    pub temperature: f64,
    pub rel_humidity: f64,
    pub fires: u64,
    pub mood: Mood,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Mood {
    Good,
    CouldBeBetter,
    Meh,
    Bad(String),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KeepAlive {
    pub everything_is_fine: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_datasheet_example() {
        let example = Measurement {
            node_id: 42,
            room_id: 1337,
            temperature: 20.0,
            rel_humidity: 0.42,
            fires: 1,
            mood: Mood::Bad("battery low".to_string()),
        };

        let input = r#"{"node_id":42,"room_id":1337,"temperature":20.0,"rel_humidity":0.42,"fires":1,"mood":{"Bad":"battery low"}}"#;
        let parsed: Measurement = serde_json::from_str(input).unwrap();
        assert_eq!(parsed, example);
    }

    #[test]
    fn can_generate_keepalive() {
        let msg = KeepAlive {
            everything_is_fine: true,
        };

        assert_eq!(
            serde_json::to_string(&msg).unwrap(),
            r#"{"everything_is_fine":true}"#
        );
    }
}
