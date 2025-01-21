use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, resource_changed, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, IntoSystemConfigs, NextState, Res, ResMut, Text,
    },
    text::TextColor,
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, FlexWrap, Node, Overflow,
        PositionType, UiRect, Val,
    },
};

use crate::{
    scenes::{
        day09::controls::build_control,
        days::{build_content, build_footer, build_header},
        resources::{FontHandles, GenericDay},
        states::{Part, UiState, VisualizationState},
    },
    scroll_controls::{ui::build_vertical_scroll_buttons, ScrollWindow, BUTTON_BACKGROUND_COLOR},
};

use super::{controls::ControlState, input::Input};

const SCROLL_SPEED: f32 = 512.;
const BLOCK_SIZE: f32 = 4.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui.run_if(
                in_state(Part::Part1)
                    .and(in_state(VisualizationState::<9>::WaitingUi))
                    .or(in_state(Part::Part1).and(
                        in_state(VisualizationState::<9>::Ready).and(resource_changed::<Input>),
                    )),
            ),
        )
        .add_systems(
            Update,
            super::process_input
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<9>::WaitingInput))),
        )
        .add_systems(
            Update,
            run_refrag.run_if(
                in_state(Part::Part1)
                    .and(in_state(VisualizationState::<9>::Ready))
                    .and(in_state(ControlState::Playing)),
            ),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day9_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 9 Part 1");
    let header = build_header(&mut commands, "day9", true, fonts.font.clone());
    let content = build_content(&mut commands, "day9");
    let footer = build_footer(&mut commands, "day9");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &fonts));
    commands
        .entity(footer)
        .with_children(|parent| build_control(parent, fonts.symbol2.clone()));

    commands
        .entity(day9_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content, footer]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input, fonts: &FontHandles) {
    parent
        .spawn(Node {
            top: Val::Px(50.),
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            right: Val::Px(10.),
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            row_gap: Val::Px(12.),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((Text::new("Filesystem checksum"), TextColor(Color::WHITE)));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                padding: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child((
                            Text::new(input.calculate_checksum().to_string()),
                            TextColor(Color::WHITE),
                        ));
                });

            let window = parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(1.),
                        column_gap: Val::Px(1.),
                        flex_wrap: FlexWrap::Wrap,
                        overflow: Overflow::scroll_y(),
                        ..Default::default()
                    },
                    ScrollWindow,
                ))
                .with_children(|parent| {
                    for block in input.disk.iter() {
                        parent.spawn((
                            Node {
                                width: Val::Px(BLOCK_SIZE),
                                height: Val::Px(BLOCK_SIZE),
                                ..Default::default()
                            },
                            BackgroundColor(block.1),
                        ));
                    }
                })
                .id();

            build_vertical_scroll_buttons(
                parent,
                window,
                SCROLL_SPEED,
                BUTTON_BACKGROUND_COLOR,
                fonts.symbol1.clone(),
            );
        });
}

pub fn run_refrag(mut input: ResMut<Input>, mut next_state: ResMut<NextState<ControlState>>) {
    for _ in 0..64 {
        if input.pos == 0 {
            next_state.set(ControlState::Paused);
        }
        input.defrag_single();
    }
}
