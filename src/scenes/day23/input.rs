use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

use bevy::prelude::{Component, Resource};

use crate::loader::RawInput;

#[derive(Debug, Clone, Resource, Component)]
pub struct Input {
    pub connections: BTreeMap<[u8; 2], Vec<[u8; 2]>>,
    pub triples: BTreeSet<[[u8; 2]; 3]>,
    pub fully_connected: Vec<[u8; 2]>,
    pub password: String,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let connections: BTreeMap<[u8; 2], Vec<[u8; 2]>> = input
            .0
            .split(|c| *c == b'\n')
            .filter(|line| !line.is_empty())
            .fold(BTreeMap::new(), |mut map, line| {
                let (l, r) = line.split_at(2);

                let l: [u8; 2] = l.try_into().unwrap();
                let r: [u8; 2] = (r[1..]).try_into().unwrap();

                map.entry(l)
                    .and_modify(|connected_to| connected_to.push(r))
                    .or_insert(vec![r]);
                map.entry(r)
                    .and_modify(|connected_to| connected_to.push(l))
                    .or_insert(vec![l]);

                map
            });

        let triples = Self::get_triplets(&connections);
        let (password, fully_connected) = Self::fully_connected(&connections);

        Self {
            connections,
            triples,
            fully_connected,
            password,
        }
    }

    fn get_triplets(connections: &BTreeMap<[u8; 2], Vec<[u8; 2]>>) -> BTreeSet<[[u8; 2]; 3]> {
        let mut triplets = BTreeSet::new();

        connections.iter().for_each(|(pc, node_connections)| {
            if pc.starts_with(b"t") {
                Self::get_triple_connections(
                    pc,
                    node_connections.as_slice(),
                    connections,
                    &mut triplets,
                );
            }
        });

        triplets
    }

    fn fully_connected(connections: &BTreeMap<[u8; 2], Vec<[u8; 2]>>) -> (String, Vec<[u8; 2]>) {
        let mut computer_groups = Vec::with_capacity(500);

        connections.iter().for_each(|(pc, node_connections)| {
            Self::get_fully_connected(
                node_connections.as_slice(),
                vec![pc],
                connections,
                &mut computer_groups,
            );
        });

        computer_groups
            .into_iter()
            .max_by_key(|group| group.len())
            .map(|group| {
                (
                    group
                        .iter()
                        .map(|pc| String::from_utf8_lossy(*pc))
                        .collect::<Vec<_>>()
                        .join(","),
                    group.into_iter().cloned().collect(),
                )
            })
            .unwrap()
    }

    fn get_triple_connections<'a>(
        pc_node: &'a [u8; 2],
        pc_connections: &'a [[u8; 2]],
        all_connections: &BTreeMap<[u8; 2], Vec<[u8; 2]>>,
        triplets: &mut BTreeSet<[[u8; 2]; 3]>,
    ) {
        let [head_node, tail @ ..] = pc_connections else {
            return;
        };

        let head_connections = all_connections.get(head_node).unwrap();

        tail.iter()
            .filter(|tail_node| head_connections.contains(tail_node))
            .for_each(|tail_node| {
                let mut nodes = [*pc_node, *head_node, *tail_node];
                nodes.sort();
                triplets.insert(nodes);
            });

        Self::get_triple_connections(pc_node, tail, all_connections, triplets);
    }

    fn get_fully_connected<'a>(
        pc_connections: &'a [[u8; 2]],
        mut partial: Vec<&'a [u8; 2]>,
        all_connections: &BTreeMap<[u8; 2], Vec<[u8; 2]>>,
        fully_connected: &mut Vec<Vec<&'a [u8; 2]>>,
    ) {
        let [head_node, tail @ ..] = pc_connections else {
            partial.sort();
            fully_connected.push(partial);
            return;
        };

        Self::get_fully_connected(tail, partial.clone(), all_connections, fully_connected);

        let head_node_connections = all_connections.get(head_node).unwrap();
        if partial
            .iter()
            .all(|group_node| head_node_connections.contains(group_node))
        {
            partial.push(head_node);
            Self::get_fully_connected(tail, partial, all_connections, fully_connected);
        }
    }
}
