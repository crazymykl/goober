use entity::{Entity, EntityType};

#[derive(Debug, Clone, PartialEq)]
pub enum VictoryState {
    Win,
    Loss
}

pub struct WinOrLoseSystem;

impl WinOrLoseSystem {
    pub fn win_or_loss(goobs: &Vec<Entity>, objects: &Vec<Entity>) -> Option<VictoryState> {
        match any_goobs_or_goals_left(goobs.clone(), objects.clone()) {
            (false, true) => Some(VictoryState::Loss),
            (_, false)    => Some(VictoryState::Win),
            _             => None
        }
    }

}

fn any_goobs_or_goals_left(goobs: Vec<Entity>, mut objects: Vec<Entity>) -> (bool, bool) {
    objects.retain(|object| object.entity_type == EntityType::Goal);
    let any_goals = !objects.is_empty();
    let any_goobs = !goobs.is_empty();
    (any_goobs, any_goals)
}
