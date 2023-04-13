use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Left,
    Right,
    Jump,
}

#[derive(Resource)]
pub struct PlayerInput {
    input: InputMap<Action>,
}

impl Default for PlayerInput {
    fn default() -> Self {
        let mut input = InputMap::default();

        input
            .insert(KeyCode::A, Action::Left)
            .insert(KeyCode::D, Action::Right)
            .insert(KeyCode::Space, Action::Jump);
        Self { input }
    }
}
