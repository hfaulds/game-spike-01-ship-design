use crate::prelude::*;

pub struct BuildPlugin;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum BuildAction {
    DeselectTool,
    SelectWallTool,
}

#[derive(Component, Debug, Default)]
struct WallTool {}

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<BuildAction>::default());
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(PlayerState::Building).with_system(tool_select_system),
        );
        app.add_system_set(
            SystemSet::on_update(BuildState::WallTool).with_system(wall_tool_system),
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
    commands.spawn((
        WallTool::default(),
        GeometryBuilder::build_as(
            &ShapePath::new().build(),
            DrawMode::Stroke(StrokeMode::new(Color::BLUE, 5.0)),
            Transform::default(),
        ),
    ));
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

fn wall_tool_system(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut wall_tools: Query<&mut Path, With<WallTool>>,
    mut ship_paths: Query<&mut Path, (With<Ship>, Without<WallTool>)>,
    buttons: Res<Input<MouseButton>>,
) {
    let cursor = round_to_grid(get_cursor_position(windows, camera), 20.0);

    if buttons.just_pressed(MouseButton::Left) {
        let wall_tool_path = wall_tools.single();
        let new_wall_path = if wall_tool_path.0.first_endpoint().is_none() {
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(cursor);
            path_builder.build()
        } else {
            let new_ship_path = ShapePath::new()
                .add(ship_paths.single())
                .add(wall_tool_path)
                .build();
            let mut ship_path = ship_paths.single_mut();
            *ship_path = new_ship_path;
            ShapePath::new().build()
        };
        let mut wall_tool_path = wall_tools.single_mut();
        *wall_tool_path = new_wall_path;
    }

    if let Ok(mut wall_tool_path) = wall_tools.get_single_mut() {
        if let Some(point) = wall_tool_path.0.first_endpoint() {
            let start = Vec2::new(point.0.x, point.0.y);

            let mut path_builder = PathBuilder::new();
            path_builder.move_to(start);
            path_builder.line_to(cursor);

            *wall_tool_path = path_builder.build();
        }
    }
}
