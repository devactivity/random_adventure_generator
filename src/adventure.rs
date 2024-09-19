use crate::game_state::{AdventureSettings, Difficulty, Genre, Length};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adventure {
    pub location: String,
    pub characters: Vec<String>,
    pub objective: String,
    pub challenges: Vec<String>,
}

impl Adventure {
    pub fn generate(settings: &AdventureSettings) -> Self {
        let location = generate_location(&settings.genre);
        let characters = generate_characters(&settings.genre, &settings.length);
        let objective = generate_objective(&settings.genre, &settings.difficulty);
        let challenges =
            generate_challenges(&settings.genre, &settings.difficulty, &settings.length);

        Adventure {
            location,
            characters,
            objective,
            challenges,
        }
    }
}

fn generate_location(genre: &Genre) -> String {
    let locations = match genre {
        Genre::Fantasy => vec![
            "Ancient Elven Forest",
            "Dwarven Mountain Stronghold",
            "Mystical Floating Islands",
            "Cursed Swamplands",
            "Dragon's Lair Caverns",
            "Enchanted Crystal Caves",
        ],
        Genre::SciFi => vec![
            "Abandoned Space Station",
            "Alien Megacity",
            "Terraformed Mars Colony",
            "Interdimensional Nexus",
            "Quantum Realm Laboratory",
            "Cybernetic Underworld",
        ],
        Genre::Horror => vec![
            "Decrepit Victorian Mansion",
            "Fog-Shrouded Graveyard",
            "Abandoned Asylum",
            "Cursed Ancient Tomb",
            "Nightmarish Dreamscape",
            "Haunted Ghost Ship",
        ],
    };
    locations
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

fn generate_characters(genre: &Genre, length: &Length) -> Vec<String> {
    let character_count = match length {
        Length::Short => 2,
        Length::Medium => 3,
        Length::Long => 4,
    };

    let mut characters = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..character_count {
        let character = match genre {
            Genre::Fantasy => {
                let archetypes = [
                    "Wise Wizard",
                    "Brave Knight",
                    "Cunning Rogue",
                    "Nature-attuned Druid",
                    "Holy Paladin",
                    "Mysterious Seer",
                    "Dwarven Blacksmith",
                    "Elven Archer",
                ];
                archetypes.choose(&mut rng).unwrap().to_string()
            }
            Genre::SciFi => {
                let archetypes = [
                    "AI Construct",
                    "Cyborg Mercenary",
                    "Alien Diplomat",
                    "Rogue Android",
                    "Psychic Navigator",
                    "Quantum Engineer",
                    "Xenobiologist",
                    "Space Pirate",
                ];
                archetypes.choose(&mut rng).unwrap().to_string()
            }
            Genre::Horror => {
                let archetypes = [
                    "Paranormal Investigator",
                    "Exorcist Priest",
                    "Haunted Medium",
                    "Skeptical Scientist",
                    "Cursed Antique Dealer",
                    "Amnesiac Survivor",
                    "Occult Scholar",
                    "Possessed Child",
                ];
                archetypes.choose(&mut rng).unwrap().to_string()
            }
        };
        characters.push(character);
    }
    characters
}

fn generate_objective(genre: &Genre, difficulty: &Difficulty) -> String {
    let objectives = match genre {
        Genre::Fantasy => vec![
            "Recover the lost artifact of immense power",
            "Break the curse afflicting the royal bloodline",
            "Prevent an ancient evil from awakening",
            "Unite warring factions to face a common threat",
            "Discover the secret of immortality hidden by the gods",
        ],
        Genre::SciFi => vec![
            "Prevent a rogue AI from taking over the galaxy",
            "Establish first contact with a newly discovered alien species",
            "Retrieve crucial data from a black hole's event horizon",
            "Stop a time-traveling assassin from altering history",
            "Cure a deadly virus spreading across multiple planets",
        ],
        Genre::Horror => vec![
            "Banish an ancient evil entity back to its realm",
            "Solve a series of gruesome murders linked to the supernatural",
            "Escape a nightmarish realm created by a malevolent being",
            "Break a generations-old family curse before it claims another victim",
            "Uncover the truth behind a town's dark and bloody history",
        ],
    };

    let base_objective = objectives
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    match difficulty {
        Difficulty::Easy => format!("{} (with clear guidance and support)", base_objective),
        Difficulty::Medium => base_objective,
        Difficulty::Hard => format!(
            "{} (while facing numerous obstacles and betrayals)",
            base_objective
        ),
    }
}

fn generate_challenges(genre: &Genre, difficulty: &Difficulty, length: &Length) -> Vec<String> {
    let challenge_count = match length {
        Length::Short => 2,
        Length::Medium => 3,
        Length::Long => 4,
    };

    let mut challenges = Vec::new();
    let mut rng = rand::thread_rng();

    let base_challenges = match genre {
        Genre::Fantasy => vec![
            "Navigate treacherous terrain",
            "Solve ancient riddles",
            "Negotiate with temperamental magical creatures",
            "Overcome a powerful magical barrier",
            "Survive in harsh, uncharted wilderness",
            "Infiltrate a heavily guarded fortress",
        ],
        Genre::SciFi => vec![
            "Hack into a secure alien database",
            "Survive in zero-gravity environments",
            "Repair a malfunctioning spacecraft mid-flight",
            "Navigate through a dangerous asteroid field",
            "Decipher an unknown alien language",
            "Neutralize a hostile artificial intelligence",
        ],
        Genre::Horror => vec![
            "Maintain sanity while facing eldritch horrors",
            "Survive a night in a haunted location",
            "Perform a complex exorcism ritual",
            "Escape from a labyrinth of nightmares",
            "Resist the influence of a corrupting artifact",
            "Uncover the truth hidden in fragmented memories",
        ],
    };

    for _ in 0..challenge_count {
        let mut challenge = base_challenges.choose(&mut rng).unwrap().to_string();

        challenge = match difficulty {
            Difficulty::Easy => format!("{} (with helpful clues)", challenge),
            Difficulty::Medium => challenge,
            Difficulty::Hard => {
                let complications = [
                    "under extreme time pressure",
                    "while being hunted",
                    "with limited resources",
                    "in complete darkness",
                    "while protecting vulnerable allies",
                ];
                format!(
                    "{} ({})",
                    challenge,
                    complications.choose(&mut rng).unwrap()
                )
            }
        };

        challenges.push(challenge);
    }

    challenges
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
