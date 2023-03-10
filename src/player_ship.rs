use crate::prelude::*;

pub const START_LIFE: u32 = 3;

// Actions are divided in two enums
// One for pure Player Ship actions, during effective gameplay, added on the player entity itself.
// One for Menu actions, added as a global resource
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Forward,
    RotateLeft,
    RotateRight,
    Fire,

    ToggleBuild,
}

#[derive(Component)]
pub struct Ship {
    /// Ship rotation speed in rad/s
    pub rotation_speed: f32,
    /// Ship thrust N
    pub thrust: f32,
    /// Ship life points
    pub life: u32,
    /// Cannon auto-fire timer
    pub cannon_timer: Timer,
    /// Id of the controlling player. 1 or 2
    pub player_id: u32,
}

#[derive(Component)]
pub struct ShipWalls {}

#[derive(Component)]
pub struct ShipEngine {}

#[derive(Component, Clone, Copy)]
pub struct Damage {
    pub value: u32,
}

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default());
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_ship))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(build_toggle_system)
                    .with_system(ship_input_system)
                    .with_system(ship_dampening_system)
                    .with_system(ship_timers_system),
            );
    }
}

// Tag component to update the exhaust particle effect with speed.
#[derive(Component)]
pub struct ExhaustEffect;

fn spawn_ship(mut commands: Commands) {
    // For player actions, allow keyboard WASD/ Arrows/ Gamepag to control the ship
    let mut input_map = InputMap::new([
        (KeyCode::W, PlayerAction::Forward),
        (KeyCode::Up, PlayerAction::Forward),
        (KeyCode::A, PlayerAction::RotateLeft),
        (KeyCode::Left, PlayerAction::RotateLeft),
        (KeyCode::D, PlayerAction::RotateRight),
        (KeyCode::Right, PlayerAction::RotateRight),
        (KeyCode::Space, PlayerAction::Fire),
        (KeyCode::F, PlayerAction::ToggleBuild),
    ]);
    input_map.insert(GamepadButtonType::South, PlayerAction::Fire);
    input_map.insert(
        SingleAxis::positive_only(GamepadAxisType::LeftStickY, 0.4),
        PlayerAction::Forward,
    );
    input_map.insert(
        SingleAxis::negative_only(GamepadAxisType::LeftStickY, -0.4),
        PlayerAction::Forward,
    );
    input_map.insert(
        SingleAxis::positive_only(GamepadAxisType::LeftStickX, 0.4),
        PlayerAction::RotateRight,
    );
    input_map.insert(
        SingleAxis::negative_only(GamepadAxisType::LeftStickX, -0.4),
        PlayerAction::RotateLeft,
    );

    commands
        .spawn((
            Ship {
                rotation_speed: 3.0,
                thrust: 60.0,
                life: START_LIFE,
                cannon_timer: Timer::from_seconds(0.2, TimerMode::Once),
                player_id: 1,
            },
            ForState {
                states: vec![AppState::Game],
            },
            RigidBody::Dynamic,
            Collider::ball(13.5),
            ExternalImpulse::default(),
            Velocity::linear(Vec2::ZERO),
            ActiveEvents::COLLISION_EVENTS,
            InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map,
            },
            GeometryBuilder::build_as(
                &PathBuilder::new().build(),
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 5.0)),
                Transform::default(),
            ),
        ))
        .with_children(|parent| {
            let mut walls_path = PathBuilder::new();
            walls_path.move_to(Vec2::new(-5.0, -5.0));
            walls_path.line_to(Vec2::new(-5.0, 5.0));
            walls_path.line_to(Vec2::new(5.0, 5.0));
            walls_path.line_to(Vec2::new(5.0, -5.0));
            walls_path.line_to(Vec2::new(-5.0, -5.0));

            parent.spawn((
                ShipWalls {},
                GeometryBuilder::build_as(
                    &walls_path.build(),
                    DrawMode::Stroke(StrokeMode::new(Color::WHITE, 5.0)),
                    Transform::default(),
                ),
            ));

            let engines = RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::Radius(5.0),
                ..shapes::RegularPolygon::default()
            };

            parent.spawn((
                ShipEngine {},
                GeometryBuilder::build_as(
                    &ShapePath::build_as(&engines),
                    DrawMode::Stroke(StrokeMode::new(Color::RED, 5.0)),
                    Transform {
                        translation: Vec3::new(0.0, -5.0, 0.0),
                        ..Default::default()
                    },
                ),
            ));
        });
}

fn ship_dampening_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Ship>>) {
    for mut velocity in query.iter_mut() {
        let elapsed = time.delta_seconds();
        velocity.angvel *= 0.1f32.powf(elapsed);
        velocity.linvel *= 0.4f32.powf(elapsed);
    }
}

fn build_toggle_system(
    mut playerstate: ResMut<State<PlayerState>>,
    query: Query<&ActionState<PlayerAction>>,
) {
    for action_state in query.iter() {
        if action_state.just_pressed(PlayerAction::ToggleBuild) {
            let newstate = match playerstate.current() {
                PlayerState::Building => PlayerState::Flying,
                PlayerState::Flying => PlayerState::Building,
            };
            playerstate.set(newstate).unwrap();
        }
    }
}

fn ship_input_system(
    gamestate: Res<State<AppGameState>>,
    playerstate: ResMut<State<PlayerState>>,
    mut laser_spawn_events: EventWriter<LaserSpawnEvent>,
    mut query: Query<(
        &ActionState<PlayerAction>,
        &mut ExternalImpulse,
        &mut Velocity,
        &Transform,
        &mut Ship,
    )>,
) {
    if gamestate.current() == &AppGameState::Game && playerstate.current() == &PlayerState::Flying {
        for (action_state, mut impulse, mut velocity, transform, mut ship) in query.iter_mut() {
            let thrust = if action_state.pressed(PlayerAction::Forward) {
                1.0
            } else {
                0.0
            };
            let rotation = if action_state.pressed(PlayerAction::RotateLeft) {
                1
            } else if action_state.pressed(PlayerAction::RotateRight) {
                -1
            } else {
                0
            };
            let fire = action_state.pressed(PlayerAction::Fire);
            if rotation != 0 {
                velocity.angvel = rotation as f32 * ship.rotation_speed;
            }
            impulse.impulse = (transform.rotation * (Vec3::Y * thrust * ship.thrust)).truncate();

            if fire && ship.cannon_timer.finished() {
                laser_spawn_events.send(LaserSpawnEvent {
                    transform: *transform,
                    velocity: *velocity,
                });
                ship.cannon_timer.reset();
            }
        }
    }
}

fn ship_timers_system(time: Res<Time>, mut ship: Query<&mut Ship>) {
    for mut ship in ship.iter_mut() {
        ship.cannon_timer.tick(time.delta());
    }
}
