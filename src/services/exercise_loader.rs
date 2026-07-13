use crate::models::exercise::Exercise;
use anyhow::Result;

pub fn load_exercises() -> Result<Vec<Exercise>> {
    let data = include_str!("../../assets/exercises.jsonc");

    let exercises: Vec<Exercise> = json5::from_str(data)?;

    Ok(exercises)
}
