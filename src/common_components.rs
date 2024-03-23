use bevy::prelude::*;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Component, Debug)]
pub struct TeamEntity {
    pub team: Team,
}

#[derive(Component, Debug)]
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
