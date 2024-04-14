// --- Plugin ---

use bevy::prelude::*;

use crate::load_game::load_factions::Factions;
use crate::main_menu::MainMenuTag;
use crate::resources::SelectedFaction;
use crate::AppState;

pub struct InitScreenPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for InitScreenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), spawn_ui)
            .add_systems(
                Update,
                (btn_interaction_handler, btn_action_handler).run_if(in_state(self.state.clone())),
            );
    }
}

// --- Constants ---

const NORMAL_BUTTON: Color = Color::rgba(1., 1., 1., 0.2);
const HOVERED_BUTTON: Color = Color::rgba(1., 1., 1., 0.3);
const PRESSED_BUTTON: Color = Color::rgba(1., 1., 1., 0.5);

// --- Components ---

#[derive(Component)]
struct Label;

#[derive(PartialEq)]
enum ButtonAction {
    Play,
}

#[derive(Component)]
struct MenuButton {
    action: ButtonAction,
}

#[derive(Component)]
struct BtnPlay;

// --- Systems ---

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("ui/fonts/warioland4tt/Warioland4chmc-VApe.ttf");
    commands
        .spawn((
            MainMenuTag,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            // Spawn title.
            builder.spawn((
                Label,
                TextBundle::from_section(
                    "Bevy Castle Fight",
                    TextStyle {
                        font: font.clone(),
                        font_size: 50.0,
                        ..Default::default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::vertical(Val::Px(50.)),
                    ..Default::default()
                }),
            ));
            // Spawn button.
            builder
                .spawn((
                    MenuButton {
                        action: ButtonAction::Play,
                    },
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(300.),
                            border: UiRect::all(Val::Px(3.)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        background_color: BackgroundColor(Color::rgba(1., 1., 1., 0.2)),
                        ..Default::default()
                    },
                ))
                .with_children(|btn_builder| {
                    btn_builder.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font,
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

fn btn_action_handler(
    mut commands: Commands,
    query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<AppState>>,
    loaded_factions: Option<Res<Factions>>,
) {
    for (interaction, menu_button) in query.iter() {
        match *interaction {
            Interaction::Pressed => match menu_button.action {
                ButtonAction::Play => {
                    if let Some(factions) = &loaded_factions {
                        if let Some(selected_faction) = factions.0.first().cloned() {
                            commands.insert_resource(SelectedFaction(selected_faction));
                            next_state.set(AppState::Game)
                        } else {
                            error!("Couldn't get a faction from loaded factions to set as the selected faction.")
                        }
                    } else {
                        error!("No factions loaded...")
                    }
                }
            },
            Interaction::Hovered | Interaction::None => {}
        }
    }
}

fn btn_interaction_handler(
    mut query: Query<(&Interaction, &mut BackgroundColor), With<MenuButton>>,
) {
    for (interaction, mut bg_colour) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_colour = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *bg_colour = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *bg_colour = NORMAL_BUTTON.into();
            }
        }
    }
}
