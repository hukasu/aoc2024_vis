use bevy::{
    app::Update,
    color::Color,
    core::Name,
    prelude::{
        in_state, BuildChildren, Button, Changed, ChildBuild, ChildBuilder, Commands,
        IntoSystemConfigs, NextState, OnEnter, Query, ResMut, Text, With, Without,
    },
    text::{TextColor, TextFont},
    ui::{FlexDirection, Interaction, JustifyContent, Node, UiRect, Val},
};

use crate::scenes::components::{Disabled, StateChange};

use super::ScenesStates;

const DISABLED_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const HOVERED_COLOR: Color = Color::srgb(0.7, 1.0, 1.0);

type ButtonWithChangedInteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (
        &'static Interaction,
        &'static mut TextColor,
        &'static StateChange,
    ),
    (With<Button>, Without<Disabled>, Changed<Interaction>),
>;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnEnter(ScenesStates::MainMenu),
            (build_ui, update_disabled_text_color)
                .chain()
                .after(super::build_main_menu),
        );
        app.add_systems(
            Update,
            button_interaction.run_if(in_state(ScenesStates::MainMenu)),
        );
    }
}

fn build_ui(mut commands: Commands, mut main_menu_resource: ResMut<super::resources::MainMenu>) {
    let ui = commands
        .spawn((
            Name::new("main_menu_ui"),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
        ))
        .with_children(build_ui_divs)
        .id();

    main_menu_resource.ui = ui;
}

fn update_disabled_text_color(mut buttons: Query<&mut TextColor, (With<Button>, With<Disabled>)>) {
    for mut button in buttons.iter_mut() {
        button.0 = DISABLED_COLOR;
    }
}

fn button_interaction(
    mut buttons: ButtonWithChangedInteractionQuery,
    mut next_state: ResMut<NextState<ScenesStates>>,
) {
    for (interaction, mut text_color, button_next_state) in buttons.iter_mut() {
        match interaction {
            Interaction::None => text_color.0 = Color::WHITE,
            Interaction::Hovered => text_color.0 = HOVERED_COLOR,
            Interaction::Pressed => next_state.set(button_next_state.0),
        }
    }
}

fn build_ui_divs(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("main_menu_left"),
            Node {
                width: Val::Percent(50.),
                left: Val::Px(0.),
                ..Default::default()
            },
        ))
        .with_children(build_ui_options);
    parent
        .spawn((
            Name::new("main_menu_right"),
            Node {
                width: Val::Percent(50.),
                right: Val::Px(0.),
                justify_content: JustifyContent::End,
                ..Default::default()
            },
        ))
        .with_children(build_ui_title_card);
}

fn build_ui_options(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("main_menu_options"),
            Node {
                top: Val::Px(100.),
                left: Val::Px(25.),
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("main_menu_day_1_through_10"),
                    Node {
                        min_width: Val::Px(100.),
                        min_height: Val::Px(50.),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("main_menu_day_1"),
                        Text::new("Day 1"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_2"),
                        Text::new("Day 2"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_3"),
                        Text::new("Day 3"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_4"),
                        Text::new("Day 4"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_5"),
                        Text::new("Day 5"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_6"),
                        Text::new("Day 6"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_7"),
                        Text::new("Day 7"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_8"),
                        Text::new("Day 8"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_9"),
                        Text::new("Day 9"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_10"),
                        Text::new("Day 10"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                });
            parent
                .spawn((
                    Name::new("main_menu_day_11_through_20"),
                    Node {
                        min_width: Val::Px(100.),
                        min_height: Val::Px(50.),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("main_menu_day_11"),
                        Text::new("Day 11"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_12"),
                        Text::new("Day 12"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_13"),
                        Text::new("Day 13"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_14"),
                        Text::new("Day 14"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_15"),
                        Text::new("Day 15"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_16"),
                        Text::new("Day 16"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_17"),
                        Text::new("Day 17"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_18"),
                        Text::new("Day 18"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_19"),
                        Text::new("Day 19"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_20"),
                        Text::new("Day 20"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                });
            parent
                .spawn((
                    Name::new("main_menu_day_21_through_25"),
                    Node {
                        min_width: Val::Px(100.),
                        min_height: Val::Px(50.),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("main_menu_day_21"),
                        Text::new("Day 21"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_22"),
                        Text::new("Day 22"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_23"),
                        Text::new("Day 23"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_24"),
                        Text::new("Day 24"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        StateChange(ScenesStates::Day(24)),
                    ));
                    parent.spawn((
                        Name::new("main_menu_day_25"),
                        Text::new("Day 25"),
                        Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        Button,
                        Disabled,
                    ));
                });
        });
}

fn build_ui_title_card(parent: &mut ChildBuilder) {
    parent.spawn((
        Name::new("main_menu_title_card"),
        Text::new("AoC 2024"),
        TextFont {
            font_size: 128.,
            ..Default::default()
        },
        Node {
            top: Val::Px(20.),
            right: Val::Px(0.),
            ..Default::default()
        },
    ));
}
