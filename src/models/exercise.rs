use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Exercise {
    pub name: String,

    #[serde(rename = "type")]
    pub exercise_type: ExerciseType,
    pub value: u32,
    pub minimum_seconds: u32,
    pub image: String,
    pub body_parts: Vec<String>,
    pub goals: Vec<String>,
    pub difficulty: u8,
    pub load: u8,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExerciseType {
    Reps,
    Time,
}
