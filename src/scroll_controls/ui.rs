use bevy::{
    asset::Handle,
    color::Color,
    prelude::{BuildChildren, ChildBuild, ChildBuilder, Entity, Text},
    text::{Font, TextColor, TextFont},
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, PositionType, Val},
};

use super::ScrollControl;

pub fn build_vertical_scroll_buttons(
    parent: &mut ChildBuilder,
    window: Entity,
    scroll_speed: f32,
    button_background_color: Color,
    symbol_font: Handle<Font>,
) {
    parent
        .spawn((Node {
            bottom: Val::Px(1.),
            right: Val::Px(1.),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.),
            ..Default::default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(25.),
                        height: Val::Px(25.),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_background_color),
                    ScrollControl {
                        horizontal: 0.,
                        vertical: -scroll_speed,
                        target: window,
                    },
                ))
                .with_child((
                    Text::new("↑"),
                    TextColor::BLACK,
                    TextFont {
                        font: symbol_font.clone(),
                        ..Default::default()
                    },
                ));
            parent
                .spawn((
                    Node {
                        width: Val::Px(25.),
                        height: Val::Px(25.),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_background_color),
                    ScrollControl {
                        horizontal: 0.,
                        vertical: scroll_speed,
                        target: window,
                    },
                ))
                .with_child((
                    Text::new("↓"),
                    TextColor::BLACK,
                    TextFont {
                        font: symbol_font.clone(),
                        ..Default::default()
                    },
                ));
        });
}

pub fn build_horizontal_scroll_buttons(
    parent: &mut ChildBuilder,
    window: Entity,
    scroll_speed: f32,
    button_background_color: Color,
    symbol_font: Handle<Font>,
) {
    parent
        .spawn(Node {
            bottom: Val::Px(5.),
            left: Val::Px(5.),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(75.),
                        height: Val::Px(30.),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ScrollControl {
                        horizontal: -scroll_speed,
                        vertical: 0.,
                        target: window,
                    },
                    BackgroundColor(button_background_color),
                ))
                .with_child((
                    Text::new("⏮"),
                    TextFont {
                        font: symbol_font.clone(),
                        ..Default::default()
                    },
                    TextColor(Color::BLACK),
                ));
            parent
                .spawn((
                    Node {
                        width: Val::Px(75.),
                        height: Val::Px(30.),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ScrollControl {
                        horizontal: scroll_speed,
                        vertical: 0.,
                        target: window,
                    },
                    BackgroundColor(button_background_color),
                ))
                .with_child((
                    Text::new("⏭"),
                    TextFont {
                        font: symbol_font.clone(),
                        ..Default::default()
                    },
                    TextColor(Color::BLACK),
                ));
        });
}
