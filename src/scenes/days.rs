use bevy::{
    color::Color,
    core::Name,
    prelude::{BuildChildren, Button, ChildBuild, ChildBuilder, Commands, Entity, Text},
    text::{TextColor, TextFont},
    ui::{
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, PositionType, UiRect, Val,
    },
};

use super::{
    components::{PartChange, StateChange},
    BUTTON_BACKGROUND_COLOR, FONT_HANDLE,
};

pub fn build_header(commands: &mut Commands, day: &str, part_change: bool) -> Entity {
    let mut header = commands.spawn((
        Name::new(format!("{day}_header")),
        Node {
            top: Val::Px(0.),
            left: Val::Px(0.),
            right: Val::Px(0.),
            position_type: PositionType::Absolute,
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
    ));
    header.with_children(|parent| build_state_buttons(parent, part_change));
    header.id()
}

pub fn build_content(commands: &mut Commands, day: &str) -> Entity {
    commands
        .spawn((
            Name::new(format!("{day}_content")),
            Node {
                top: Val::Px(0.),
                bottom: Val::Px(0.),
                left: Val::Px(0.),
                right: Val::Px(0.),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                height: Val::Percent(100.),
                ..Default::default()
            },
        ))
        .id()
}

pub fn build_footer(commands: &mut Commands, day: &str) -> Entity {
    commands
        .spawn((
            Name::new(format!("{day}_footer")),
            Node {
                bottom: Val::Px(0.),
                left: Val::Px(0.),
                right: Val::Px(0.),
                position_type: PositionType::Absolute,
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
        .id()
}

fn build_state_buttons(parent: &mut ChildBuilder, part_change: bool) {
    let font = FONT_HANDLE.get().expect("Font should be initialized.");

    parent
        .spawn((
            button_node(),
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
            Button,
            StateChange(super::states::States::MainMenu),
        ))
        .with_child((
            Text::new("Exit"),
            TextFont {
                font: font.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    if part_change {
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
}

pub fn button_node() -> Node {
    Node {
        width: Val::Px(75.),
        height: Val::Px(30.),
        justify_content: JustifyContent::SpaceEvenly,
        align_items: AlignItems::Center,
        ..Default::default()
    }
}
