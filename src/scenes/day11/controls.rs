use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, Button, Changed, ChildBuild, ChildBuilder, Component,
        IntoSystemConfigs, Query, ResMut, Text,
    },
    text::TextColor,
    ui::{BackgroundColor, FlexDirection, Interaction, Node, UiRect},
};

use crate::{
    scenes::states::VisualizationState,
    scroll_controls::{BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR},
};

use super::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            control_interaction.run_if(in_state(VisualizationState::<11>::Ready)),
        );
    }
}

#[derive(Debug, Component)]
#[require(Button)]
pub enum Control {
    Reset,
    Blink,
}

pub fn build_controls(parent: &mut ChildBuilder) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: bevy::ui::Val::Px(4.),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        padding: UiRect::horizontal(bevy::ui::Val::Px(8.))
                            .with_top(bevy::ui::Val::Px(5.))
                            .with_bottom(bevy::ui::Val::Px(5.)),
                        ..Default::default()
                    },
                    Control::Reset,
                ))
                .with_child((Text::new("Reset"), TextColor(Color::BLACK)));
            parent
                .spawn((
                    Node {
                        padding: UiRect::horizontal(bevy::ui::Val::Px(8.))
                            .with_top(bevy::ui::Val::Px(5.))
                            .with_bottom(bevy::ui::Val::Px(5.)),
                        ..Default::default()
                    },
                    Control::Blink,
                ))
                .with_child((Text::new("Blink"), TextColor(Color::BLACK)));
        });
}

fn control_interaction(
    mut controls: Query<(&Interaction, &Control, &mut BackgroundColor), Changed<Interaction>>,
    mut input: ResMut<Input>,
) {
    for (interaction, control, mut background_color) in controls.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR,
            Interaction::Pressed => match control {
                Control::Reset => input.reset(),
                Control::Blink => input.blink(),
            },
        }
    }
}
