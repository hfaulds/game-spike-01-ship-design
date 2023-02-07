use crate::prelude::*;

pub struct BuildPlugin;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum BuildAction {
    DeselectTool,
    SelectWallTool,
    SelectEngineTool,
}

#[derive(Component, Debug, Default)]
struct WallTool {}

#[derive(Component, Debug, Default)]
struct EngineTool {}

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
        app.add_system_set(
            SystemSet::on_update(BuildState::EngineTool).with_system(engine_tool_system),
        );
    }
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::new([
        (KeyCode::Key0, BuildAction::DeselectTool),
        (KeyCode::Key1, BuildAction::SelectWallTool),
        (KeyCode::Key2, BuildAction::SelectEngineTool),
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
    if buildstate.current() != &BuildState::None
        && action_state.just_pressed(BuildAction::DeselectTool)
    {
        buildstate.set(BuildState::None).unwrap();
    }
    if buildstate.current() != &BuildState::WallTool
        && action_state.just_pressed(BuildAction::SelectWallTool)
    {
        buildstate.set(BuildState::WallTool).unwrap();
    }
    if buildstate.current() != &BuildState::EngineTool
        && action_state.just_pressed(BuildAction::SelectEngineTool)
    {
        buildstate.set(BuildState::EngineTool).unwrap();
    }
}

fn wall_tool_system(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    ship_transforms: Query<&GlobalTransform, With<Ship>>,
    mut wall_tools: Query<&mut Path, With<WallTool>>,
    mut wall_paths: Query<&mut Path, (With<ShipWalls>, Without<WallTool>)>,
    buttons: Res<Input<MouseButton>>,
) {
    if let Some(cursor_global) = get_cursor_position(windows, camera) {
        let ship_transform = ship_transforms.single();
        let cursor_local = point_relative_to_transform(cursor_global, ship_transform).truncate();
        let cursor = round_to_grid(cursor_local, 20.0);
        if buttons.just_pressed(MouseButton::Left) {
            let wall_tool_path = wall_tools.single();
            let new_wall_tool_path = if wall_tool_path.0.first_endpoint().is_none() {
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(cursor);
                path_builder.build()
            } else {
                let new_wall_path = ShapePath::new()
                    .add(wall_paths.single())
                    .add(wall_tool_path)
                    .build();
                let mut wall_path = wall_paths.single_mut();
                *wall_path = new_wall_path;
                ShapePath::new().build()
            };
            let mut wall_tool_path = wall_tools.single_mut();
            *wall_tool_path = new_wall_tool_path;
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
}

fn engine_tool_system(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    ships: Query<(Entity, &GlobalTransform), With<Ship>>,
    buttons: Res<Input<MouseButton>>,
) {
    if let Some(cursor_global) = get_cursor_position(windows, camera) {
        let (ship_entity, ship_transform) = ships.single();
        let cursor_local = point_relative_to_transform(cursor_global, ship_transform).truncate();
        let cursor = round_to_grid(cursor_local, 20.0);
        if buttons.just_pressed(MouseButton::Left) {
            let engines = RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::Radius(5.0),
                ..shapes::RegularPolygon::default()
            };

            commands
                .get_entity(ship_entity)
                .unwrap()
                .add_children(|parent| {
                    parent.spawn((
                        ShipEngine {},
                        GeometryBuilder::build_as(
                            &ShapePath::build_as(&engines),
                            DrawMode::Fill(FillMode::color(Color::RED)),
                            Transform {
                                translation: Vec3::new(cursor.x, cursor.y, 0.0),
                                ..Default::default()
                            },
                        ),
                    ));
                });
        }
    }
}
