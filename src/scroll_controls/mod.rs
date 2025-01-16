pub mod ui;

use bevy::{
    app::Update,
    color::Color,
    prelude::{
        Button, Commands, Component, Entity, Event, OnAdd, Parent, Query, Res, Trigger, With,
    },
    time::Time,
    ui::{BackgroundColor, Interaction, Node, ScrollPosition},
};

pub const BUTTON_BACKGROUND_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
pub const BUTTON_HOVERED_BACKGROUND_COLOR: Color = Color::srgb(0.8, 0.8, 0.9);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ScrollControlHovered>();

        app.add_systems(Update, scroll_control_hovered);

        app.add_observer(attatch_observer_to_window);
    }
}

#[derive(Debug, Clone, Copy, Component)]
#[require(Button)]
pub struct ScrollControl {
    pub horizontal: f32,
    pub vertical: f32,
    pub target: Entity,
}

#[derive(Debug, Component)]
#[require(Node)]
pub struct ScrollWindow;

#[derive(Debug, Component)]
struct ScrollControlHovered(ScrollControl);

impl Event for ScrollControlHovered {
    type Traversal = &'static Parent;
    const AUTO_PROPAGATE: bool = true;
}

fn scroll_control_hovered(
    mut commands: Commands,
    mut buttons: Query<(&mut BackgroundColor, &ScrollControl, &Interaction), With<Button>>,
) {
    for (mut background_color, scroll_control, interaction) in buttons.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered | Interaction::Pressed => {
                background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR;
                commands
                    .trigger_targets(ScrollControlHovered(*scroll_control), scroll_control.target);
            }
        }
    }
}

fn attatch_observer_to_window(trigger: Trigger<OnAdd, ScrollWindow>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(scroll_on_control_hovered);
}

fn scroll_on_control_hovered(
    trigger: Trigger<ScrollControlHovered>,
    mut windows: Query<&mut ScrollPosition>,
    time: Res<Time>,
) {
    let Ok(mut window) = windows.get_mut(trigger.entity()) else {
        bevy::log::error!(
            "ScrollWindow {} did not have ScrollPosition.",
            trigger.entity()
        );
        return;
    };

    let offset = trigger.0;
    window.offset_x += offset.horizontal * time.delta_secs();
    window.offset_y += offset.vertical * time.delta_secs();
}
