use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, IntoSystemConfigs, NextState,
        Res, ResMut, Text,
    },
    text::TextColor,
    ui::{BorderColor, BorderRadius, FlexDirection, Node, Overflow, PositionType, UiRect, Val},
};

use crate::{
    scenes::{
        days::{build_content, build_header},
        resources::GenericDay,
        states::{UiState, VisualizationState},
        FONT_SYMBOLS_HANDLE,
    },
    scroll_controls::{ui::build_vertical_scroll_buttons, ScrollWindow, BUTTON_BACKGROUND_COLOR},
};

use super::input;

const SCROLL_SPEED: f32 = 512.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui.run_if(in_state(VisualizationState::<2>::WaitingUi)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day2_resource: Res<GenericDay>,
    input: Res<input::Input>,
    mut next_state: ResMut<NextState<UiState>>,
) {
    bevy::log::trace!("Day 2");
    let header = build_header(&mut commands, "day2", false);
    let content = build_content(&mut commands, "day2");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input));

    commands
        .entity(day2_resource.ui)
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &input::Input) {
    let safe = input
        .reports
        .iter()
        .filter(|report| matches!(report.safety, input::Safety::Safe))
        .count();
    let kinda_safe = input
        .reports
        .iter()
        .filter(|report| {
            matches!(
                report.safety,
                input::Safety::Safe | input::Safety::OneError(_)
            )
        })
        .count();

    parent
        .spawn(Node {
            top: Val::Px(50.),
            bottom: Val::Px(0.),
            left: Val::Px(10.),
            right: Val::Px(10.),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child(Text::new("Safe"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(5.)),
                                padding: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(safe.to_string()));
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child(Text::new("Almost Safe"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(5.)),
                                padding: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(kinda_safe.to_string()));
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    let window = parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::scroll_y(),
                                row_gap: Val::Px(3.),
                                ..Default::default()
                            },
                            ScrollWindow,
                        ))
                        .with_children(|parent| {
                            for report in &input.reports {
                                let border_color = match report.safety {
                                    input::Safety::Safe => palettes::basic::GREEN,
                                    input::Safety::OneError(_) => palettes::basic::YELLOW,
                                    input::Safety::Unsafe => palettes::basic::RED,
                                };
                                parent
                                    .spawn((
                                        Node {
                                            flex_direction: FlexDirection::Row,
                                            column_gap: Val::Px(8.),
                                            border: UiRect::all(Val::Px(5.)),
                                            padding: UiRect::all(Val::Px(3.)),
                                            ..Default::default()
                                        },
                                        BorderColor(border_color.into()),
                                        BorderRadius::all(Val::Px(5.)),
                                    ))
                                    .with_children(|parent| {
                                        let unsafe_val =
                                            if let input::Safety::OneError(err) = report.safety {
                                                err
                                            } else {
                                                usize::MAX
                                            };
                                        for (i, val) in report.report.iter().enumerate() {
                                            let text_color = if i == unsafe_val {
                                                palettes::basic::RED.into()
                                            } else {
                                                Color::WHITE
                                            };
                                            parent.spawn(Node::default()).with_child((
                                                Text::new(val.to_string()),
                                                TextColor(text_color),
                                            ));
                                        }
                                    });
                            }
                        })
                        .id();

                    build_vertical_scroll_buttons(
                        parent,
                        window,
                        SCROLL_SPEED,
                        BUTTON_BACKGROUND_COLOR,
                        FONT_SYMBOLS_HANDLE.get().unwrap().clone(),
                    );
                });
        });
}
