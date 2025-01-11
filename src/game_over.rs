use bevy::prelude::*;

use crate::{
    game::{GameoverTriggersSubSystem, Reset},
    input_translation::InputTranslationSystem,
    ui::{accept_action, AcceptAction, Focused},
    GameState,
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>();
        app.add_systems(Startup, spawn_game_over_menu);
        app.add_systems(PreUpdate, accept_action::<Reset>.after(InputTranslationSystem));
        app.add_systems(Update, (set_game_over_reason, set_game_over_state).in_set(GameoverResponseSystem));
        app.add_systems(OnExit(GameState::Gameover), enter_game_over);
        app.add_systems(OnExit(GameState::Gameover), exit_game_over);
        app.configure_sets(Update, GameoverResponseSystem.after(GameoverTriggersSubSystem));
    }
}

#[derive(SystemSet, Clone, Eq, PartialEq, Debug, Hash)]
pub struct GameoverResponseSystem;

#[derive(Event)]
pub struct GameOver {
    reason: GameOverReason,
}
impl GameOver {
    pub fn new(reason: GameOverReason) -> Self {
        Self { reason }
    }
}
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum GameOverReason {
    Crashed,
    //TODO
}

impl GameOverReason {
    fn message(&self) -> &'static str {
        match self {
            GameOverReason::Crashed => "You crashed!",
        }
    }
}

fn set_game_over_state(mut events: EventReader<GameOver>, mut next_state: ResMut<NextState<GameState>>) {
    if !events.is_empty() {
        next_state.set(GameState::Gameover);
        events.clear();
    }
}

fn set_game_over_reason(mut events: EventReader<GameOver>, reason_text: Single<&mut Text, With<GameOverReasonText>>) {
    let mut text = reason_text.into_inner();
    for event in events.read() {
        **text = event.reason.message().to_owned();
    }
}

fn enter_game_over(
    mut commands: Commands,
    menu_visible: Single<&mut Visibility, With<GameOverMenu>>,
    restart_button: Single<Entity, With<GameOverRestartButton>>,
) {
    let mut visible = menu_visible.into_inner();
    *visible = Visibility::Visible;
    let entity = restart_button.into_inner();
    commands.entity(entity).insert(Focused);
}

fn exit_game_over(
    mut commands: Commands,
    menu_visible: Single<&mut Visibility, With<GameOverMenu>>,
    focused: Query<Entity, With<Focused>>,
) {
    let mut visible = menu_visible.into_inner();
    *visible = Visibility::Hidden;

    for entity in focused.into_iter() {
        commands.entity(entity).remove::<Focused>();
    }
}

pub fn spawn_game_over_menu(mut commands: Commands) {
    commands.spawn(GameOverMenu).with_children(|parent| {
        parent.spawn(GameOverTitleText);
        parent.spawn(GameOverReasonText);
        parent.spawn(GameOverRestartButton);
    });
}

#[derive(Component)]
#[require(AcceptAction<Reset>)]
struct GameOverRestartButton;

#[derive(Component)]
#[require(Text, Node(Self::node))]
struct GameOverReasonText;
impl GameOverReasonText {
    fn node() -> Node {
        Node {
            grid_column: GridPlacement::span(2),
            ..default()
        }
    }
}

#[derive(Component)]
#[require(Text(Self::text), Node(Self::node))]
struct GameOverTitleText;

impl GameOverTitleText {
    fn text() -> Text {
        Text::new("Game Over")
    }
    fn node() -> Node {
        Node {
            grid_column: GridPlacement::span(2),
            ..default()
        }
    }
}

#[derive(Component)]
#[require(Node(Self::node), BackgroundColor(Self::background_color), BorderColor(Self::border_color), Visibility(|| Visibility::Hidden))]
struct GameOverMenu;

impl GameOverMenu {
    fn border_color() -> BorderColor {
        BorderColor(Color::BLACK)
    }
    fn node() -> Node {
        Node {
            width: Val::Percent(40.0),
            height: Val::Percent(30.0),
            display: Display::Grid,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::flex(1.0)],
            grid_template_columns: vec![GridTrack::flex(0.5), GridTrack::flex(0.5)],
            ..default()
        }
    }
    fn background_color() -> BackgroundColor {
        BackgroundColor(Color::srgb(0.8, 0.5, 0.2))
    }
}
