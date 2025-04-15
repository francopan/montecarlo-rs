use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SprintRecord {
    // pub sprint_name: String,
    pub completed_story_points: u16,
}
