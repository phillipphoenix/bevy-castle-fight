use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkFields;
use bevy_ecs_ldtk::EntityInstance;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Reflect, Hash)]
pub enum Team {
    TeamRed,
    TeamBlue,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Team::TeamRed => write!(f, "RED"),
            Team::TeamBlue => write!(f, "BLUE"),
        }
    }
}

#[derive(Component, Debug, Reflect)]
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
                "RED" => Team::TeamRed,
                "BLUE" => Team::TeamBlue,
                _ => {
                    panic!("Team {:?} doesn't exist!", team_field);
                }
            },
        }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Health {
    pub max_health: f32,
    pub health: f32,
}

/// From how far away can an entity spot for instance opponents.
#[derive(Component)]
pub struct VisionRange(f32);

#[derive(Component)]
pub struct AttackTarget(pub Entity); // TODO: Make it contain a list of targets.
                                     // The reason for the above to do is that when a target is destroyed or out of sight
                                     // new potential targets that are already inside the sensor collider range will not be
                                     // be used as new targets. We therefore need to store all targets within the vision range.
                                     // NOTE: Perhaps we could have the list on another component (AttackTargetsInVision)
                                     // This component would always create a new AttackTarget component with the next target, if none exist.
