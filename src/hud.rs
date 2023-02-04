use crate::prelude::*;

#[derive(Component)]
pub struct UiScore {}
#[derive(Component)]
pub struct UiLife {
    pub min: u32,
}

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(hud_score_system)
                .with_system(hud_life_system),
        )
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(hud_spawn))
        .add_system_set(SystemSet::on_enter(PlayerState::Building).with_system(build_hud_spawn))
        .add_system_set(SystemSet::on_exit(PlayerState::Building).with_system(build_hud_despawn));
    }
}

fn hud_spawn(mut commands: Commands, assets: ResMut<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::Game],
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexEnd,
                        margin: UiRect {
                            left: Val::Px(10.0),
                            right: Val::Px(10.0),
                            top: Val::Px(10.0),
                            bottom: Val::Px(10.0),
                        },
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "0",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                        },
                    ),
                    ..Default::default()
                },
                UiScore {},
            ));
        });
    // Life counters
    // Not kept in 'GameOver' state, simplifying last counter removal.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::Game],
            },
        ))
        .with_children(|parent| {
            for i in 1..(START_LIFE + 1) {
                parent.spawn((
                    ImageBundle {
                        style: Style {
                            margin: UiRect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        image: assets.ship_life.clone(),
                        ..Default::default()
                    },
                    UiLife { min: i },
                ));
            }
        });
}

fn hud_score_system(arena: Res<Arena>, mut query: Query<&mut Text, With<UiScore>>) {
    if arena.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}", arena.score);
        }
    }
}
fn hud_life_system(ship_query: Query<&Ship>, mut uilife_query: Query<(&mut Visibility, &UiLife)>) {
    let mut life = 0;
    for ship in ship_query.iter() {
        if ship.player_id == 1 {
            life = ship.life;
        }
    }
    for (mut visibility, uilife) in uilife_query.iter_mut() {
        visibility.is_visible = life >= uilife.min;
    }
}

fn build_hud_spawn(mut commands: Commands, assets: ResMut<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::Game],
            },
            ForState {
                states: vec![PlayerState::Building],
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexEnd,
                            margin: UiRect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        text: Text::from_section(
                            "A",
                            TextStyle {
                                font: assets.font.clone(),
                                font_size: 50.0,
                                color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                            },
                        ),
                        ..Default::default()
                    });
                    parent.spawn(TextBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexEnd,
                            margin: UiRect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        text: Text::from_section(
                            "B",
                            TextStyle {
                                font: assets.font.clone(),
                                font_size: 50.0,
                                color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                            },
                        ),
                        ..Default::default()
                    });
                });
        });
}

fn build_hud_despawn(mut commands: Commands, query: Query<(Entity, &ForState<PlayerState>)>) {
    for (entity, for_state) in &mut query.iter() {
        if for_state.states.contains(&PlayerState::Building) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
