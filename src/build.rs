use crate::prelude::*;

pub struct BuildPlugin;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum BuildAction {
    DeselectTool,
    SelectWallTool,
}

#[derive(Resource, Debug, Default)]
struct WallTool {
    start: Option<Vec2>,
}

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<BuildAction>::default());
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(PlayerState::Building).with_system(tool_select_system),
        );
        app.add_system_set(
            SystemSet::on_update(BuildState::WallTool).with_system(wall_place_system),
        );
    }
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::new([
        (KeyCode::Key1, BuildAction::SelectWallTool),
        (KeyCode::Key0, BuildAction::DeselectTool),
    ]);
    commands.insert_resource(input_map);
    commands.init_resource::<WallTool>();
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
        buildstate.set(BuildState::WallTool).unwrap();
    }
}

fn wall_place_system(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut wall_tool: ResMut<WallTool>,
    buttons: Res<Input<MouseButton>>,
    mut ship_paths: Query<&mut Path, With<Ship>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let pos = round_to_grid(get_cursor_position(windows, camera), 20.0);
        match wall_tool.start {
            None => {
                wall_tool.start = Some(pos);
            }
            Some(start) => {
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(start);
                path_builder.line_to(pos);
                let line = path_builder.build();

                let path_builder = { ShapePath::new().add(ship_paths.single()).add(&line) };
                let mut ship_path = ship_paths.single_mut();
                *ship_path = path_builder.build();
                wall_tool.start = None;
            }
        }
    }
}
