use crate::{components::prelude::*, GameState, HanTextStyle};
use bevy::{color::palettes::css::*, prelude::*};

pub fn game_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    // justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            place_header(parent);

            place_stage(parent);
        });
}

fn place_header(parent: &mut ChildBuilder) {
    let style = HanTextStyle::default()
        .with_color(bevy::prelude::Color::Srgba(BLACK))
        .with_font_size(30.0)
        .get_style();
    parent
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                // height: Val::Percent(10.0),
                ..default()
            },
            background_color: DARK_ORANGE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(20.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: LIGHT_BLUE.into(),
                        ..Default::default()
                    },
                    ButtonOnGamePage::BackMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("返回菜单", style.clone()));
                });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(60.0),
                    ..default()
                },
                ..default()
            });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(20.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            // margin: UiRect::top(Val::Px(20.0)),
                            ..Default::default()
                        },
                        background_color: LIGHT_GRAY.into(),
                        ..Default::default()
                    },
                    ButtonOnGamePage::ExitGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("关闭退出", style.clone()));
                });
        });
}

fn place_stage(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: DARK_ORANGE.into(),
            ..default()
        })
        .with_children(|parent| {
            // 三行布局：对面、中、自己。其中中间的包括左边、中桌、右边
            place_north_spot(parent);
            place_center_line(parent);
            place_south_spot(parent);
        });
}

fn place_north_spot(parent: &mut ChildBuilder) {

}
// 中间的包括左边、中桌、右边
fn place_center_line(parent: &mut ChildBuilder) {}

fn place_south_spot(parent: &mut ChildBuilder) {}

pub fn game_update(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonOnGamePage),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, btn_tag) in &mut interaction_query {
        match (btn_tag, *interaction) {
            (ButtonOnGamePage::BackMenuButton, Interaction::Pressed) => {
                game_state.set(GameState::Menu);
            }
            (ButtonOnGamePage::BackMenuButton, Interaction::Hovered) => {
                *color = DARK_BLUE.into();
            }
            (ButtonOnGamePage::ExitGameButton, Interaction::Hovered) => {
                *color = DARK_GREY.into();
            }
            (ButtonOnGamePage::ExitGameButton, Interaction::Pressed) => {
                app_exit_events.send(AppExit::Success);
            }
            (ButtonOnGamePage::ExitGameButton, Interaction::None) => {
                *color = LIGHT_GRAY.into();
            }
            (ButtonOnGamePage::BackMenuButton, Interaction::None) => {
                *color = LIGHT_BLUE.into();
            }
        }
    }
}
