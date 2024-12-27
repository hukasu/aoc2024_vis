use bevy::{
    app::Update,
    color::Color,
    core::Name,
    prelude::{
        in_state, not, resource_added, resource_exists, BuildChildren, Button, Changed, ChildBuild,
        ChildBuilder, Commands, DespawnRecursiveExt, Entity, IntoSystemConfigs, OnExit, Query, Res,
        Text, With,
    },
    text::{TextColor, TextFont},
    ui::{
        AlignItems, BackgroundColor, BorderColor, FlexDirection, Interaction, JustifyContent, Node,
        PositionType, UiRect, Val,
    },
};

use crate::scenes::{
    components::{PartChange, StateChange},
    BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR, FONT_HANDLE, FONT_SYMBOLS_HANDLE,
};

use super::{components::Controls, resources::Input};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(super::states::States::Part1))
                .run_if(resource_added::<Input>),
        )
        .add_systems(OnExit(super::states::States::Part1), destroy_ui)
        .add_systems(
            Update,
            super::process_input
                .run_if(in_state(super::states::States::Part1))
                .run_if(not(resource_exists::<Input>)),
        );
    }
}

type ControlWithChangedInteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (
        Entity,
        &'static mut BackgroundColor,
        &'static Interaction,
        &'static Controls,
    ),
    (With<Button>, Changed<Interaction>),
>;

fn controls_interaction(mut commands: Commands, mut controls: ControlWithChangedInteractionQuery) {
    for (button, mut background_color, interaction, control) in controls.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR,
            Interaction::Pressed => {
                commands.trigger(control);
            }
        }
    }
}

fn build_ui(
    mut commands: Commands,
    day24_resource: Res<super::resources::Day24>,
    input: Res<Input>,
) {
    commands
        .entity(day24_resource.ui)
        .with_children(|parent| build_divs(parent, &input));
}

fn destroy_ui(mut commands: Commands, day24_resource: Res<super::resources::Day24>) {
    commands.remove_resource::<Input>();
    commands.entity(day24_resource.ui).despawn_descendants();
}

fn build_divs(parent: &mut ChildBuilder, input: &Input) {
    parent
        .spawn((
            Name::new("day24_states"),
            Node {
                top: Val::Px(0.),
                left: Val::Px(0.),
                padding: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.),
                ..Default::default()
            },
        ))
        .with_children(build_state_buttons);
    parent
        .spawn((
            Name::new("day24_content"),
            Node {
                flex_direction: FlexDirection::Row,
                height: Val::Percent(100.),
                border: UiRect::all(Val::Px(2.)),
                ..Default::default()
            },
            BorderColor(Color::WHITE),
        ))
        .with_children(|parent| build_visualization(parent, input));
    parent
        .spawn((
            Name::new("day24_controls"),
            Node {
                bottom: Val::Px(0.),
                left: Val::Px(0.),
                padding: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.),
                ..Default::default()
            },
        ))
        .with_children(build_control_buttons);
}

pub(super) fn build_state_buttons(parent: &mut ChildBuilder) {
    let font = FONT_HANDLE.get().expect("Font should be initialized.");

    parent
        .spawn((
            button_node(),
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
            Button,
            StateChange(super::SceneStates::MainMenu),
        ))
        .with_child((
            Text::new("Exit"),
            TextFont {
                font: font.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    parent
        .spawn((
            button_node(),
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
            PartChange::Part1,
        ))
        .with_child((
            Text::new("Part 1"),
            TextFont {
                font: font.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    parent
        .spawn((
            button_node(),
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
            PartChange::Part2,
        ))
        .with_child((
            Text::new("Part 2"),
            TextFont {
                font: font.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input) {
    parent
        .spawn((
            Name::new("day_24_part1_visualization"),
            Node {
                top: Val::Px(10.),
                bottom: Val::Px(10.),
                left: Val::Px(10.),
                right: Val::Px(10.),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(5.)),
                ..Default::default()
            },
            BorderColor(Color::WHITE),
        ))
        .with_children(|parent| {
            spawn_input_row(parent, "x", &input.x);
            spawn_input_row(parent, "y", &input.y);
            parent.spawn(Node {
                height: Val::Px(5.),
                ..Default::default()
            });
            spawn_input_row(parent, "z", &input.z);
            parent.spawn(Node {
                height: Val::Px(5.),
                ..Default::default()
            });
        });
}

fn spawn_input_row(parent: &mut ChildBuilder, title: &str, row: &[u8]) {
    parent
        .spawn((Node {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        min_width: Val::Px(40.),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(BUTTON_BACKGROUND_COLOR),
                ))
                .with_child((Text::new(title), TextColor::BLACK));
            for val in row.iter().rev() {
                parent
                    .spawn((
                        Node {
                            border: UiRect::all(Val::Px(1.)),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        BorderColor(BUTTON_BACKGROUND_COLOR),
                    ))
                    .with_child((
                        Text::new(val.to_string()),
                        TextColor(BUTTON_BACKGROUND_COLOR),
                    ));
            }
        });
}

fn build_control_buttons(parent: &mut ChildBuilder) {
    let font = FONT_SYMBOLS_HANDLE
        .get()
        .expect("Font should be initialized.");
    parent
        .spawn((
            button_node(),
            Controls::Reset,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏮"),
            TextFont {
                font: font.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    parent
        .spawn((
            button_node(),
            Controls::Step,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏵"),
            TextFont {
                font: font.clone(),

                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    parent
        .spawn((
            button_node(),
            Controls::FastForward,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏭"),
            TextFont {
                font: font.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
}

fn button_node() -> Node {
    Node {
        width: Val::Px(75.),
        height: Val::Px(30.),
        justify_content: JustifyContent::SpaceEvenly,
        align_items: AlignItems::Center,
        ..Default::default()
    }
}
