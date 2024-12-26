mod game;
mod game_over;
mod tetrust;

use bevy::prelude::*;

use game::GamePlugin;
use game_over::GameOverPlugin;

#[derive(Debug, Default, States, Clone, Hash, PartialEq, Eq)]
enum GameState {
    GameOver,
    #[default]
    Game,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin, GameOverPlugin))
        .init_state::<GameState>()
        .add_systems(Update, pressed_q_exit)
        .run();
}

fn pressed_q_exit(input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyQ) {
        std::process::exit(0)
    }
}

fn despawn_screen<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
