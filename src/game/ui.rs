// --- Plugin ---

use bevy::prelude::*;

use crate::game::InGameTag;
use crate::load_game::load_factions::BuildingBlueprint;
use crate::resources::SelectedFaction;

pub struct UiPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for UiPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), setup_ui)
            .add_systems(
                Update,
                building_btn_interactions.run_if(in_state(self.state.clone())),
            );
    }
}

// --- Components ---

#[derive(Component)]
struct BtnBuilding;
#[derive(Component)]
struct BtnBuildingImage;

// --- Systems ---

fn setup_ui(mut commands: Commands, selected_faction: Res<SelectedFaction>) {
    // Layout.
    commands
        .spawn((
            InGameTag, // Adding this tag, means it will be cleaned up, when exiting the "Game" app state.
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|layout| {
            // Top UI bar.
            layout.spawn(NodeBundle {
                ..Default::default()
            });
            // Bottom UI bar.
            layout
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::End,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|bottom_bar| {
                    // Building menu.
                    bottom_bar
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                // Add padding around the grid.
                                padding: UiRect::all(Val::Px(12.)),
                                // Set a height and the aspect ratio to be 1:1, so it will auto set the width
                                // to be equal to the height.
                                height: Val::Percent(30.),
                                aspect_ratio: Some(1.0),
                                // Set the grid to have 4 columns.
                                grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                                // Set the grid to have 4 rows.
                                grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                                // Set a gap between each cell.
                                row_gap: Val::Px(12.),
                                column_gap: Val::Px(12.),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.5)),
                            ..Default::default()
                        })
                        .with_children(|building_menu| {
                            for building in &selected_faction.0.buildings {
                                spawn_building_btn(building_menu, building);
                            }
                        });
                });
        });
}

fn building_btn_interactions(
    query: Query<(&Interaction, &Children), (Changed<Interaction>, With<BtnBuilding>)>,
    mut image_query: Query<&mut BackgroundColor>,
) {
    for (interaction, children) in &query {
        let mut image_bg_color = image_query.get_mut(children[0]).unwrap();
        match *interaction {
            // Interaction::Pressed => {
            //     text.sections[0].value = "Press".to_string();
            //     *color = PRESSED_BUTTON.into();
            //     border_color.0 = Color::RED;
            // }
            Interaction::Hovered => {
                *image_bg_color = Color::rgba(1., 1., 1., 0.5).into();
            }
            Interaction::None => {
                *image_bg_color = Color::WHITE.into();
            }
            _ => {}
        }
    }
}

// --- Helper functions ---

fn spawn_building_btn(builder: &mut ChildBuilder, building_blueprint: &BuildingBlueprint) {
    builder
        .spawn((
            BtnBuilding,
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|btn| {
            btn.spawn((
                BtnBuildingImage,
                ImageBundle {
                    image: UiImage::new(building_blueprint.icon.clone()),
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                },
            ));
        });
}
