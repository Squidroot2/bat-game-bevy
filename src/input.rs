use bevy::{
    input::{gamepad::GamepadEvent, keyboard::KeyboardInput},
    prelude::*,
};

#[derive(Event, PartialEq, Eq)]
pub enum UserInput {
    Start,
    Flap,
    Screetch,
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
pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                process_keyboard_events,
                process_gamepad_events,
                get_keyboard_direction,
                get_gamepad_direction,
                process_mouse,
            ),
        );
        app.add_systems(PostUpdate, reset_direction);
        app.add_event::<UserInput>();
        app.init_resource::<DirectionalInput>();
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

fn process_keyboard_events(mut writer: EventWriter<UserInput>, mut input_reader: EventReader<KeyboardInput>) {
    for input in input_reader.read() {
        if !input.state.is_pressed() {
            continue;
        }
        match input.key_code {
            KeyCode::Escape => writer.send(UserInput::Start),
            KeyCode::Space => writer.send(UserInput::Flap),
            KeyCode::ControlLeft => writer.send(UserInput::Screetch),
            _ => continue,
        };
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

fn process_gamepad_events(mut writer: EventWriter<UserInput>, mut input_reader: EventReader<GamepadEvent>) {
    for input in input_reader.read() {
        if let GamepadEvent::Button(button_event) = input {
            if !button_event.state.is_pressed() {
                continue;
            }
            match button_event.button {
                GamepadButton::Start => writer.send(UserInput::Start),
                GamepadButton::South => writer.send(UserInput::Flap),
                GamepadButton::West => writer.send(UserInput::Screetch),
                _ => continue,
            };
        }
    }
}
fn process_mouse() {
    //TODO
}
fn reset_direction(mut direction: ResMut<DirectionalInput>) {
    direction.reset()
}
