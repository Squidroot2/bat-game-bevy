use bevy::prelude::*;

use crate::input_translation::MenuInput;

#[derive(Component)]
pub struct AcceptAction<T>(T);

impl<T> AcceptAction<T>
where
    T: Event + Clone,
{
    fn send_with(&self, writer: &mut EventWriter<T>) {
        writer.send(self.0.clone());
    }
}
impl<T: Default> Default for AcceptAction<T> {
    fn default() -> Self {
        Self(T::default())
    }
}

#[derive(Component)]
pub struct Focused;

pub fn accept_action<T>(
    mut events: EventReader<MenuInput>,
    focus_query: Option<Single<&AcceptAction<T>, With<Focused>>>,
    mut writer: EventWriter<T>,
) where
    T: Event + Clone,
{
    if let Some(focused) = focus_query {
        let accept_action = focused.into_inner();
        for event in events.read() {
            if *event == MenuInput::Accept {
                accept_action.send_with(&mut writer);
            }
        }
    }
}
