use std::{collections::HashMap, fmt::Debug};

use bevy::{
    asset::{AssetServer, Handle, RenderAssetUsages},
    color::{palettes, ColorToPacked},
    image::{Image, ImageSampler},
    prelude::{Component, Resource},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::loader::RawInput;

pub const IMAGE_HEIGHT: usize = 5;

#[derive(Debug, Clone, Resource, Component)]
pub struct Input {
    pub longest_towel: usize,
    pub towels: Vec<Towel>,
    pub patterns: Vec<Pattern>,
}

impl Input {
    pub fn parse(input: &RawInput, asset_server: &AssetServer) -> Self {
        let mut input = input.split(|c| *c == b'\n').filter(|line| !line.is_empty());

        let towels = input
            .next()
            .map(|line| {
                line.split(|c| *c == b',')
                    .map(|towel| String::from_utf8_lossy(towel.trim_ascii()))
                    .collect::<Vec<_>>()
            })
            .unwrap();

        let longest_towel = towels.iter().max_by_key(|towel| towel.len()).unwrap().len();

        let towels = towels
            .into_iter()
            .map(|towel| {
                let (stripes, image) = Self::towel_to_image(&towel, longest_towel, asset_server);
                Towel { stripes, image }
            })
            .collect();

        let _ = input.next().unwrap();

        let patterns = {
            let patterns = input
                .map(|pattern| String::from_utf8_lossy(pattern))
                .collect::<Vec<_>>();

            patterns
                .into_iter()
                .map(|pattern| {
                    let (pattern, image) = Self::pattern_to_image(&pattern, asset_server);
                    Pattern { pattern, image }
                })
                .collect()
        };

        Self {
            longest_towel,
            towels,
            patterns,
        }
    }

    pub fn match_pattern<'a>(&'a self, pattern: &Pattern) -> Vec<Vec<&'a Towel>> {
        let mut matched = vec![];

        let mut cache = HashMap::new();

        self.find_pattern_matches(&pattern.pattern, vec![], &mut cache, &mut matched);

        matched
    }

    pub fn count_patterns(&self) -> usize {
        let mut cache = HashMap::new();

        self.patterns
            .iter()
            .map(|pattern| self.possible_patterns(&pattern.pattern, &mut cache))
            .sum()
    }

    fn possible_patterns<'a>(
        &self,
        pattern: &'a str,
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if pattern.is_empty() {
            return 1;
        }
        self.towels
            .iter()
            .filter(|towel| pattern.starts_with(&towel.stripes))
            .map(|towel| {
                if let Some(cached) = cache.get(&pattern[towel.stripes.len()..]) {
                    *cached
                } else {
                    let count = self.possible_patterns(&pattern[towel.stripes.len()..], cache);
                    cache.insert(&pattern[towel.stripes.len()..], count);
                    count
                }
            })
            .sum()
    }

    fn find_pattern_matches<'s, 'a>(
        &'s self,
        pattern: &'a str,
        towels: Vec<&'s Towel>,
        cache: &mut HashMap<&'a str, Vec<Vec<&'s Towel>>>,
        matched: &mut Vec<Vec<&'s Towel>>,
    ) -> bool {
        if pattern.is_empty() {
            matched.push(towels);
            true
        } else if let Some(cached) = cache.get(pattern) {
            if cached.is_empty() {
                false
            } else {
                for group in cached {
                    let mut towels = towels.clone();
                    towels.extend(group);
                    matched.push(towels);
                }
                true
            }
        } else {
            let mut any = false;

            let towels_that_match_start = self
                .towels
                .iter()
                .filter(|towel| pattern.starts_with(&towel.stripes))
                .collect::<Vec<_>>();

            if towels_that_match_start.is_empty() {
                cache.insert(pattern, vec![]);
            } else {
                for towel in towels_that_match_start {
                    let mut towels = towels.clone();
                    towels.push(towel);

                    let remaining = &pattern[towel.stripes.len()..];

                    if self.find_pattern_matches(remaining, towels.clone(), cache, matched) {
                        cache
                            .entry(remaining)
                            .and_modify(|vec| vec.push(towels.clone()))
                            .or_insert_with(|| vec![towels]);
                        any |= true;
                    } else {
                        cache.insert(remaining, vec![]);
                    }
                }
            }

            any
        }
    }

    fn towel_to_image(
        pattern: &str,
        longest: usize,
        asset_server: &AssetServer,
    ) -> (String, Handle<Image>) {
        let mut image = Image::new(
            Extent3d {
                width: u32::try_from(longest + 2).unwrap(),
                height: u32::try_from(IMAGE_HEIGHT).unwrap(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            [&0].into_iter()
                .chain(pattern.as_bytes().iter())
                .chain(vec![&0; longest - pattern.len() + 1])
                .cycle()
                .take((longest + 2) * IMAGE_HEIGHT)
                .flat_map(|stripe| match stripe {
                    0 => palettes::tailwind::BLUE_200.to_u8_array(),
                    b'r' => palettes::basic::RED.to_u8_array(),
                    b'g' => palettes::basic::GREEN.to_u8_array(),
                    b'u' => palettes::basic::BLUE.to_u8_array(),
                    b'w' => palettes::basic::WHITE.to_u8_array(),
                    b'b' => palettes::basic::BLACK.to_u8_array(),
                    _ => unreachable!("AoC inputs are well formed"),
                })
                .collect::<Vec<_>>(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        image.sampler = ImageSampler::nearest();

        (pattern.to_string(), asset_server.add(image))
    }

    fn pattern_to_image(pattern: &str, asset_server: &AssetServer) -> (String, Handle<Image>) {
        let length = pattern.len();
        let mut image = Image::new(
            Extent3d {
                width: u32::try_from(length).unwrap(),
                height: u32::try_from(IMAGE_HEIGHT).unwrap(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            pattern
                .as_bytes()
                .iter()
                .cycle()
                .take(length * IMAGE_HEIGHT)
                .flat_map(|stripe| match stripe {
                    b'r' => palettes::basic::RED.to_u8_array(),
                    b'g' => palettes::basic::GREEN.to_u8_array(),
                    b'u' => palettes::basic::BLUE.to_u8_array(),
                    b'w' => palettes::basic::WHITE.to_u8_array(),
                    b'b' => palettes::basic::BLACK.to_u8_array(),
                    _ => unreachable!("AoC inputs are well formed"),
                })
                .collect::<Vec<_>>(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        image.sampler = ImageSampler::nearest();

        (pattern.to_string(), asset_server.add(image))
    }
}

#[derive(Debug, Clone)]
pub struct Towel {
    pub stripes: String,
    pub image: Handle<Image>,
}

#[derive(Debug, Clone, PartialEq, Component)]
pub struct Pattern {
    pub pattern: String,
    pub image: Handle<Image>,
}
