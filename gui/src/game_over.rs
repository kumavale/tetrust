use bevy::prelude::*;

use crate::{despawn_screen, GameState};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), game_over_setup)
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_screen::<GameOverScreen>,
            );
    }
}

#[derive(Component)]
struct GameOverScreen;

fn game_over_setup(mut commands: Commands) {
    commands
        .spawn((
            GameOverScreen,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("Game Over"), TextFont::default()));
            parent.spawn((Text::new("Pressed q key to exit"), TextFont::default()));
        });
}
