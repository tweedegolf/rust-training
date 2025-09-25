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
