use bevy::{input::InputSystem, prelude::*};

#[derive(Event, PartialEq, Eq)]
pub enum GameInput {
    Start,
    Flap,
    Screetch,
    Reset,
}

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
struct InputTranslationSystem;

pub struct InputTranslationPlugin;
impl Plugin for InputTranslationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                process_keyboard_events,
                process_gamepad_events,
                get_keyboard_direction,
                get_gamepad_direction,
                process_mouse,
            )
                .in_set(InputTranslationSystem),
        );
        app.add_systems(PostUpdate, reset_direction);
        app.add_event::<GameInput>();
        app.init_resource::<DirectionalInput>();
        app.configure_sets(PreUpdate, InputTranslationSystem.after(InputSystem));
    }
}

fn get_keyboard_direction(keys: Res<ButtonInput<KeyCode>>, mut direction: ResMut<DirectionalInput>) {
    if keys.pressed(KeyCode::KeyA) {
        direction.add_left();
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.add_right();
    }
}

fn process_keyboard_events(mut writer: EventWriter<GameInput>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        writer.send(GameInput::Start);
    }
    if keys.just_pressed(KeyCode::Space) {
        writer.send(GameInput::Flap);
    }
    if keys.just_pressed(KeyCode::ControlLeft) {
        writer.send(GameInput::Screetch);
    }
    if keys.just_pressed(KeyCode::KeyR) {
        writer.send(GameInput::Reset);
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

fn process_gamepad_events(mut writer: EventWriter<GameInput>, gamepads: Query<&Gamepad>) {
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::Start) {
            writer.send(GameInput::Start);
        }
        if gamepad.just_pressed(GamepadButton::South) {
            writer.send(GameInput::Flap);
        }
        if gamepad.just_pressed(GamepadButton::West) {
            writer.send(GameInput::Screetch);
        }
        if gamepad.just_pressed(GamepadButton::Select) {
            writer.send(GameInput::Reset);
        }
    }
}
fn process_mouse() {
    //TODO
}
fn reset_direction(mut direction: ResMut<DirectionalInput>) {
    direction.reset()
}
