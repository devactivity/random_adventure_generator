use crate::ai_client;
use crate::game_state::AdventureSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adventure {
    pub location: String,
    pub characters: Vec<String>,
    pub objective: String,
    pub challenges: Vec<String>,
}

impl Adventure {
    pub async fn generate(settings: &AdventureSettings) -> color_eyre::Result<Self> {
        let genre = format!("{:?}", settings.genre);
        let difficulty = format!("{:?}", settings.difficulty);
        let length = format!("{:?}", settings.length);

        let ai_response = ai_client::generate_adventure(&genre, &difficulty, &length).await?;

        let mut lines = ai_response.lines();
        let location = lines
            .next()
            .unwrap_or("Unknown")
            .trim_start_matches("Location: ")
            .to_string();
        let characters = lines
            .next()
            .unwrap_or("Unknown")
            .trim_start_matches("Characters: ")
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        let objective = lines
            .next()
            .unwrap_or("Unknown")
            .trim_start_matches("Objective: ")
            .to_string();
        let challenges = lines
            .next()
            .unwrap_or("Unknown")
            .trim_start_matches("Challenges: ")
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Adventure {
            location,
            characters,
            objective,
            challenges,
        })
    }
}

impl std::fmt::Display for Adventure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Location: {}", self.location)?;
        writeln!(f, "\nCharacters:")?;
        for character in &self.characters {
            writeln!(f, "- {}", character)?;
        }

        writeln!(f, "\nObjective: {}", self.objective)?;

        writeln!(f, "\nChallenges:")?;
        for challenge in &self.challenges {
            writeln!(f, "- {}", challenge)?;
        }
        Ok(())
    }
}
