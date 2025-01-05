use bevy::{input::InputSystem, prelude::*, window::PrimaryWindow};

#[derive(Event, PartialEq, Eq)]
pub enum GameInput {
    Start,
    Flap,
    Screetch,
}

#[derive(Event, PartialEq, Eq)]
pub enum MenuInput {
    Up,
    Down,
    Left,
    Right,
    Accept,
    Back,
}
/// Sum Horizontal directional input from the player. Ensure this is reset every frame
#[derive(Resource, Default)]
pub struct DirectionalInput {
    raw: f32,
}
impl DirectionalInput {
    pub fn get_normalized(&self) -> f32 {
        self.raw.clamp(-1.0, 1.0)
    }
    fn add_left(&mut self) {
        self.raw -= 1.0
    }
    fn add_right(&mut self) {
        self.raw += 1.0
    }
    fn add_value(&mut self, val: f32) {
        self.raw += val;
    }
    fn reset(&mut self) {
        self.raw = 0.0
    }
}

#[derive(SystemSet, Clone, Eq, PartialEq, Debug, Hash)]
pub struct InputTranslationSystem;

pub struct InputTranslationPlugin;
impl Plugin for InputTranslationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                process_key_game_input,
                process_gamepad_game_input,
                get_keyboard_direction,
                get_gamepad_direction,
                process_mouse_game_input,
            )
                .in_set(InputTranslationSystem),
        );
        app.add_systems(PostUpdate, reset_direction);
        app.add_event::<GameInput>();
        app.init_resource::<DirectionalInput>();
        app.configure_sets(PreUpdate, InputTranslationSystem.run_if(window_in_focus).after(InputSystem));
    }
}

fn window_in_focus(window: Single<&Window, With<PrimaryWindow>>) -> bool {
    let window = window.into_inner();
    window.focused
}

fn get_keyboard_direction(keys: Res<ButtonInput<KeyCode>>, mut direction: ResMut<DirectionalInput>) {
    if keys.pressed(KeyCode::KeyA) {
        direction.add_left();
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.add_right();
    }
}

fn process_key_game_input(mut writer: EventWriter<GameInput>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        writer.send(GameInput::Start);
    }
    if keys.just_pressed(KeyCode::Space) {
        writer.send(GameInput::Flap);
    }
    if keys.just_pressed(KeyCode::ControlLeft) {
        writer.send(GameInput::Screetch);
    }
}
const GAMEPAD_DEADZONE: f32 = 0.08;
fn get_gamepad_direction(gamepads: Query<&Gamepad>, mut direction: ResMut<DirectionalInput>) {
    for gamepad in gamepads.iter() {
        let input_direction = gamepad.left_stick().x;
        if input_direction.abs() > GAMEPAD_DEADZONE {
            direction.add_value(input_direction);
        }
    }
}

fn process_gamepad_game_input(mut writer: EventWriter<GameInput>, gamepads: Query<&Gamepad>) {
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::Start) {
            writer.send(GameInput::Start);
        }
        if gamepad.just_pressed(GamepadButton::South) {
            writer.send(GameInput::Flap);
        }
        if gamepad.just_pressed(GamepadButton::West) {
            println!("West");
            writer.send(GameInput::Screetch);
        }
    }
}
fn process_mouse_game_input(mut writer: EventWriter<GameInput>, mouse_buttons: Res<ButtonInput<MouseButton>>) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        writer.send(GameInput::Flap);
    }
    if mouse_buttons.just_pressed(MouseButton::Right) {
        writer.send(GameInput::Screetch);
    }
}

fn process_key_menu_navigation(mut writer: EventWriter<MenuInput>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::KeyK]) {
        writer.send(MenuInput::Up);
    }
    if keys.any_just_pressed([KeyCode::ArrowLeft, KeyCode::KeyA, KeyCode::KeyH]) {
        writer.send(MenuInput::Left);
    }
    if keys.any_just_pressed([KeyCode::ArrowRight, KeyCode::KeyD, KeyCode::KeyL]) {
        writer.send(MenuInput::Right);
    }
    if keys.any_just_pressed([KeyCode::ArrowDown, KeyCode::KeyS, KeyCode::KeyJ]) {
        writer.send(MenuInput::Down);
    }
    if keys.any_just_pressed([KeyCode::Enter, KeyCode::KeyE]) {
        writer.send(MenuInput::Accept);
    }
    if keys.just_pressed(KeyCode::Escape) {
        writer.send(MenuInput::Back);
    }
}
fn reset_direction(mut direction: ResMut<DirectionalInput>) {
    direction.reset()
}
