use bevy::prelude::*;

use crate::components::prelude::*;
use crate::resources::prelude::*;

pub fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load::<Font>(crate::constants::APP_FONT);
    unsafe { crate::constants::HAN_FONT_OPTION = Some(font_handle) };
    let icon_handle = asset_server.load(crate::constants::APP_ICON);
    unsafe { crate::constants::APP_ICON_IMAGE = Some(icon_handle) };
    
    
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            let icon = asset_server.load("poker-title.png");
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
}

pub fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
