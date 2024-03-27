use bevy::prelude::{Color, Component, Reflect};
use bevy_ecs_ldtk::prelude::LdtkFields;
use bevy_ecs_ldtk::EntityInstance;
use std::fmt;
use std::fmt::Formatter;

// --- Enums ---

#[derive(Clone, Copy, Eq, PartialEq, Debug, Reflect, Hash, Default)]
pub enum Team {
    #[default]
    Gaia,
    Red,
    Blue,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Team::Gaia => write!(f, "GAIA"),
            Team::Red => write!(f, "RED"),
            Team::Blue => write!(f, "BLUE"),
        }
    }
}

impl Team {
    pub fn get_color(&self) -> Color {
        match self {
            Team::Gaia => Color::GRAY,
            Team::Red => Color::RED,
            Team::Blue => Color::BLUE,
        }
    }
}

// --- Components ---

#[derive(Default, Component, Debug, Reflect)]
pub struct TeamEntity {
    pub team: Team,
}

impl TeamEntity {
    pub fn from_field(entity_instance: &EntityInstance) -> TeamEntity {
        let team_field = entity_instance
            .get_enum_field("team")
            .expect("Team enum wasn't found on the LDTK entity...");
        TeamEntity {
            team: match team_field.as_str() {
                "GAIA" => Team::Gaia,
                "RED" => Team::Red,
                "BLUE" => Team::Blue,
                _ => {
                    panic!("Team {:?} doesn't exist!", team_field);
                }
            },
        }
    }
}
