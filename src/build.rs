use crate::prelude::*;

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(BuildState::WallPlace).with_system(start_place_wall_system),
        );
    }
}

fn start_place_wall_system() {}
