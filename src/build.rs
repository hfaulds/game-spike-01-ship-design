use crate::prelude::*;

pub struct BuildPlugin;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum BuildAction {
    DeselectTool,
    SelectWallTool,
}

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<BuildAction>::default());
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(PlayerState::Building).with_system(tool_select_system),
        );
        app.add_system_set(
            SystemSet::on_update(BuildState::WallPlace).with_system(wall_place_system),
        );
    }
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::new([
        (KeyCode::Key1, BuildAction::SelectWallTool),
        (KeyCode::Key0, BuildAction::DeselectTool),
    ]);
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<BuildAction>::default());
}

fn tool_select_system(
    mut buildstate: ResMut<State<BuildState>>,
    action_state: Res<ActionState<BuildAction>>,
) {
    if action_state.just_pressed(BuildAction::DeselectTool) {
        buildstate.set(BuildState::None).unwrap();
    }
    if action_state.just_pressed(BuildAction::SelectWallTool) {
        buildstate.set(BuildState::WallPlace).unwrap();
    }
}

fn wall_place_system() {}
