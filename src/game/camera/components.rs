use bevy::prelude::*;

#[derive(Component)]
pub struct SecondaryCamera;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SecondaryCameraState {
    #[default]
    Hidden, // State for the main menu
    Visible, // State for when the game is running
}
