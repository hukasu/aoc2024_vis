mod components;
mod operation;
mod part1;
mod part2;
mod resources;
mod states;

use bevy::{
    app::Update,
    asset::{AssetServer, Assets},
    color::Color,
    core::Name,
    prelude::{
        in_state, AppExtStates, Camera2d, ClearColor, Commands, DespawnRecursiveExt,
        IntoSystemConfigs, OnEnter, OnExit, Res,
    },
    ui::{FlexDirection, Node, Val},
};
use operation::{Operation, Operator};
use resources::{Day24, Input};

use crate::{loader::Input as InputAsset, scenes::states::States as SceneStates};

use super::state_button_interactions;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((part1::Plugin, part2::Plugin));

        app.add_sub_state::<states::States>();

        app.add_systems(OnEnter(SceneStates::Day(24)), build_day_24);
        app.add_systems(OnExit(SceneStates::Day(24)), destroy_day_24);
        app.add_systems(
            Update,
            state_button_interactions::<states::States>.run_if(in_state(SceneStates::Day(24))),
        );
    }
}

fn build_day_24(mut commands: Commands, asset_server: Res<AssetServer>) {
    let day24_resource = resources::Day24 {
        input: asset_server.load("inputs/day24.txt"),
        camera: commands.spawn((Name::new("day24_camera"), Camera2d)).id(),
        ui: commands
            .spawn((
                Name::new("day24_ui"),
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
            ))
            .id(),
    };

    commands.insert_resource(ClearColor(Color::srgb_u8(0x0f, 0x0f, 0x23)));
    commands.insert_resource(day24_resource);
}

fn destroy_day_24(mut commands: Commands, day24_resource: Res<resources::Day24>) {
    commands.entity(day24_resource.camera).despawn_recursive();
    commands.entity(day24_resource.ui).despawn_recursive();

    commands.remove_resource::<resources::Day24>();
}

fn process_input(mut commands: Commands, day24: Res<Day24>, inputs: Res<Assets<InputAsset>>) {
    if let Some(input) = inputs.get(day24.input.id()) {
        let mut lines = input.split(|c| *c == b'\n');

        let mut part1 = Input::default();

        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let (l, r) = line.split_at(3);
            match l {
                [b'x', d, u] => part1.x[usize::from(ascii_to_num(*d, *u))] = r[2] - b'0',
                [b'y', d, u] => part1.y[usize::from(ascii_to_num(*d, *u))] = r[2] - b'0',
                _ => unreachable!("First section must only contain x and y."),
            }
        }

        for line in lines.filter(|line| !line.is_empty()) {
            let line = line.split(|c| *c == b' ').collect::<Vec<_>>();
            let [l, op, r, _, out] = line.as_slice() else {
                unreachable!("Line on second section must have 5 items.");
            };

            let op = match op {
                [b'A', b'N', b'D'] => Operator::And,
                [b'O', b'R'] => Operator::Or,
                [b'X', b'O', b'R'] => Operator::Xor,
                _ => unreachable!("Invalid operator"),
            };

            let l: [u8; 3] = (*l).try_into().expect("Should be able to convert");
            if !matches!(l, [b'x', _, _] | [b'y', _, _] | [b'z', _, _]) {
                part1.intermediate.insert(l, 0);
            }
            let r: [u8; 3] = (*r).try_into().expect("Should be able to convert");
            if !matches!(r, [b'x', _, _] | [b'y', _, _] | [b'z', _, _]) {
                part1.intermediate.insert(r, 0);
            }
            let out: [u8; 3] = (*out).try_into().expect("Should be able to convert");
            if !matches!(r, [b'x', _, _] | [b'y', _, _] | [b'z', _, _]) {
                part1.intermediate.insert(out, 0);
            }

            part1.operations.push(Operation {
                l,
                operator: op,
                r,
                out,
            });
        }

        commands.insert_resource(part1);
    }
}

fn ascii_to_num(d: u8, u: u8) -> u8 {
    (d - b'0') * 10 + (u - b'0')
}
