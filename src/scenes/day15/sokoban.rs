use std::time::Duration;

use bevy::{
    app::Update,
    color::palettes,
    prelude::{
        in_state, resource_exists_and_changed, BuildChildren, ChildBuild, Commands, Component,
        Condition, DespawnRecursiveExt, Entity, IntoSystemConfigs, NextState, Res, ResMut,
        Resource, Single, With,
    },
    time::common_conditions::on_timer,
    ui::{BackgroundColor, FlexDirection, Node, Val},
};

use crate::{
    scenes::states::VisualizationState,
    tools::{Coord, Direction},
};

use super::{controls::ControlState, input::Input};

type RobotMove = Direction;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            update_sokoban.run_if(
                in_state(VisualizationState::<15>::Ready)
                    .and(in_state(ControlState::Playing))
                    .and(on_timer(Duration::from_millis(100))),
            ),
        )
        .add_systems(
            Update,
            update_canvas.run_if(
                in_state(VisualizationState::<15>::Ready)
                    .and(resource_exists_and_changed::<Warehouse>),
            ),
        );
    }
}

#[derive(Debug, Component)]
#[require(Node)]
pub struct SokobanCanvas;

fn update_sokoban(mut sokoban: ResMut<Warehouse>, mut next_state: ResMut<NextState<ControlState>>) {
    if sokoban.has_instructions() {
        sokoban.next_move();
    } else {
        next_state.set(ControlState::Paused);
    }
}

fn update_canvas(
    mut commands: Commands,
    canvas: Single<Entity, With<SokobanCanvas>>,
    sokoban: Res<Warehouse>,
) {
    commands
        .entity(*canvas)
        .despawn_descendants()
        .with_children(|parent| {
            for y in 0..sokoban.dimensions.row {
                parent
                    .spawn(Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(10000. / sokoban.dimensions.row as f32),
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        for x in 0..sokoban.dimensions.column {
                            let mut tile = parent.spawn(Node {
                                height: Val::Percent(100.),
                                aspect_ratio: Some(1.),
                                ..Default::default()
                            });

                            match sokoban.map[y * sokoban.dimensions.column + x] {
                                WarehouseTile::Empty => (),
                                WarehouseTile::Robot => {
                                    tile.insert(BackgroundColor(
                                        palettes::tailwind::EMERALD_800.into(),
                                    ));
                                }
                                WarehouseTile::Wall => {
                                    tile.insert(BackgroundColor(
                                        palettes::tailwind::GRAY_800.into(),
                                    ));
                                }
                                WarehouseTile::BoxLeft | WarehouseTile::BoxRight => {
                                    tile.insert(BackgroundColor(
                                        palettes::tailwind::ORANGE_800.into(),
                                    ));
                                }
                            }
                        }
                    });
            }
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WarehouseTile {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

#[derive(Debug, Resource)]
pub struct Warehouse {
    robot: Coord,
    map: Vec<WarehouseTile>,
    dimensions: Coord,
    wide: bool,
    instructions: Vec<RobotMove>,
}

impl Warehouse {
    pub fn from_input(input: &Input, wide: bool) -> Self {
        let multiplier = if wide { 2 } else { 1 };

        let dimensions = {
            let max = input.walls.iter().max().unwrap();
            Coord::new(max.1 + 1, (max.0 + 1) * multiplier)
        };

        let mut tiles = vec![WarehouseTile::Empty; dimensions.row * dimensions.column];

        let robot = Coord::new(input.robot.1, input.robot.0 * multiplier);
        tiles[robot.row * dimensions.column + robot.column] = WarehouseTile::Robot;

        for wall in input.walls.iter() {
            tiles[wall.1 * dimensions.column + (wall.0 * multiplier)] = WarehouseTile::Wall;
            if wide {
                tiles[wall.1 * dimensions.column + ((wall.0 * multiplier) + 1)] =
                    WarehouseTile::Wall;
            }
        }

        for box_ in input.boxes.iter() {
            tiles[box_.1 * dimensions.column + (box_.0 * multiplier)] = WarehouseTile::BoxLeft;
            if wide {
                tiles[box_.1 * dimensions.column + ((box_.0 * multiplier) + 1)] =
                    WarehouseTile::BoxRight;
            }
        }

        let mut instructions = input.instructions.clone();
        instructions.reverse();

        Self {
            dimensions,
            robot,
            map: tiles,
            wide,
            instructions,
        }
    }

    pub fn compute_gps(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(i, tile)| {
                let row = i / self.dimensions.column;
                let column = i % self.dimensions.column;
                match tile {
                    WarehouseTile::Empty
                    | WarehouseTile::Wall
                    | WarehouseTile::Robot
                    | WarehouseTile::BoxRight => 0,
                    WarehouseTile::BoxLeft => row * 100 + column,
                }
            })
            .sum()
    }

    pub fn next_move(&mut self) {
        let Some(mov) = self.instructions.pop() else {
            return;
        };

        self.move_robot(mov);
    }

    pub fn has_instructions(&self) -> bool {
        !self.instructions.is_empty()
    }

    pub fn dimensions(&self) -> Coord {
        self.dimensions
    }

    fn move_robot(&mut self, robot_move: RobotMove) {
        let next = robot_move.step_unchecked(self.robot);
        if self.can_push_box(next, robot_move, true) {
            self.propagate_push(self.robot, robot_move, true);
            self.robot = next;
        }
    }

    fn get_coord(&self, coord: Coord) -> WarehouseTile {
        self.map[coord.row * self.dimensions.column + coord.column]
    }

    fn get_coord_mut(&mut self, coord: Coord) -> &mut WarehouseTile {
        &mut self.map[coord.row * self.dimensions.column + coord.column]
    }

    fn can_push_box(&mut self, coord: Coord, robot_move: RobotMove, check_sides: bool) -> bool {
        match self.get_coord(coord) {
            WarehouseTile::Empty => true,
            WarehouseTile::Wall => false,
            WarehouseTile::BoxLeft => {
                let next = robot_move.step_unchecked(coord);
                let right = coord + (0, 1);

                if right == next {
                    self.can_push_box(next, robot_move, false)
                } else {
                    self.can_push_box(next, robot_move, true)
                        && if self.wide && check_sides {
                            self.can_push_box(right, robot_move, false)
                        } else {
                            true
                        }
                }
            }
            WarehouseTile::BoxRight => {
                let next = robot_move.step_unchecked(coord);
                let left = coord - (0, 1);

                if left == next {
                    self.can_push_box(next, robot_move, false)
                } else {
                    self.can_push_box(next, robot_move, true)
                        && if check_sides {
                            self.can_push_box(left, robot_move, false)
                        } else {
                            true
                        }
                }
            }
            _ => unreachable!("Invalid coord for move"),
        }
    }

    fn propagate_push(&mut self, coord: Coord, robot_move: RobotMove, propagate_sides: bool) {
        match self.get_coord(coord) {
            WarehouseTile::Empty => (),
            WarehouseTile::Wall => unreachable!("Wall should not be part of propagation"),
            WarehouseTile::BoxLeft => {
                let next = robot_move.step_unchecked(coord);
                let right = coord + (0, 1);
                if right == next {
                    self.propagate_push(next, robot_move, false);
                } else {
                    self.propagate_push(next, robot_move, true);
                    if self.wide && propagate_sides {
                        self.propagate_push(right, robot_move, false);
                    }
                }
                *self.get_coord_mut(next) = WarehouseTile::BoxLeft;
                *self.get_coord_mut(coord) = WarehouseTile::Empty;
            }
            WarehouseTile::BoxRight => {
                let next = robot_move.step_unchecked(coord);
                let left = coord - (0, 1);
                if left == next {
                    self.propagate_push(next, robot_move, false);
                } else {
                    self.propagate_push(next, robot_move, true);
                    if self.wide && propagate_sides {
                        self.propagate_push(left, robot_move, false);
                    }
                }
                *self.get_coord_mut(next) = WarehouseTile::BoxRight;
                *self.get_coord_mut(coord) = WarehouseTile::Empty;
            }
            WarehouseTile::Robot => {
                let next = robot_move.step_unchecked(coord);
                self.propagate_push(next, robot_move, true);
                *self.get_coord_mut(next) = WarehouseTile::Robot;
                *self.get_coord_mut(coord) = WarehouseTile::Empty;
            }
        };
    }
}
