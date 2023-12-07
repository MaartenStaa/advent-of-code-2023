use std::ops::Range;

use chumsky::prelude::*;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let almanac = almanac_parser().parse(input).unwrap();

    println!(
        "Lowest mapped location: {}",
        almanac
            .seeds
            .iter()
            .map(|seed| almanac.resolve_seed_location(*seed))
            .min()
            .unwrap()
    );
    println!(
        "Lowest mapped location with ranges: {}",
        almanac
            .seeds_from_ranges()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|s| almanac.resolve_seed_location(s))
            .min()
            .unwrap(),
    );
}

fn almanac_parser() -> impl Parser<char, Almanac, Error = Simple<char>> {
    let number = text::int(10).map(|s: String| s.parse().unwrap());
    let number_list = number.separated_by(just(' ')).collect();

    let mapping = number
        .then_ignore(just(' '))
        .then(number)
        .then_ignore(just(' '))
        .then(number)
        .map(
            |((destination_range_start, source_range_start), source_range_length)| Mapping {
                destination_range_start,
                source_range: source_range_start..(source_range_start + source_range_length),
            },
        )
        .boxed();
    let map = mapping
        .separated_by(text::newline())
        .at_least(1)
        .map(|mappings| Map { mappings });

    let newlines = text::newline().repeated().at_least(1);

    just("seeds:")
        .padded()
        .ignore_then(number_list)
        .then_ignore(newlines)
        .then_ignore(just("seed-to-soil map:"))
        .then_ignore(text::newline())
        .then(map.clone())
        .then_ignore(newlines)
        .then_ignore(just("soil-to-fertilizer map:"))
        .then_ignore(text::newline())
        .then(map.clone())
        .then_ignore(newlines)
        .then_ignore(just("fertilizer-to-water map:"))
        .then_ignore(text::newline())
        .then(map.clone())
        .then_ignore(newlines)
        .then_ignore(just("water-to-light map:"))
        .then_ignore(text::newline())
        .then(map.clone())
        .then_ignore(newlines)
        .then_ignore(just("light-to-temperature map:"))
        .then_ignore(text::newline())
        .then(map.clone())
        .then_ignore(newlines)
        .then_ignore(just("temperature-to-humidity map:"))
        .then_ignore(text::newline())
        .then(map.clone())
        .then_ignore(newlines)
        .then_ignore(just("humidity-to-location map:"))
        .then_ignore(text::newline())
        .then(map)
        .then_ignore(text::newline().repeated())
        .map(
            |(
                (
                    (
                        (
                            (
                                ((seeds, seed_to_soil_map), soil_to_fertilizer_map),
                                fertilizer_to_water_map,
                            ),
                            water_to_light_map,
                        ),
                        light_to_temperature_map,
                    ),
                    temperature_to_humidity_map,
                ),
                humidity_to_location_map,
            )| Almanac {
                seeds,
                seed_to_soil_map,
                soil_to_fertilizer_map,
                fertilizer_to_water_map,
                water_to_light_map,
                light_to_temperature_map,
                temperature_to_humidity_map,
                humidity_to_location_map,
            },
        )
        .then_ignore(end())
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temperature_to_humidity_map: Map,
    humidity_to_location_map: Map,
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    destination_range_start: u64,
    source_range: Range<u64>,
}

impl Almanac {
    fn resolve_seed_location(&self, seed: u64) -> u64 {
        let mut value = seed;
        for map in &[
            &self.seed_to_soil_map,
            &self.soil_to_fertilizer_map,
            &self.fertilizer_to_water_map,
            &self.water_to_light_map,
            &self.light_to_temperature_map,
            &self.temperature_to_humidity_map,
            &self.humidity_to_location_map,
        ] {
            value = map.resolve(value);
        }

        value
    }

    fn seeds_from_ranges(&self) -> impl Iterator<Item = u64> + '_ {
        self.seeds
            .chunks(2)
            .flat_map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
    }
}

impl Map {
    fn resolve(&self, value: u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(resolved) = mapping.resolve(value) {
                return resolved;
            }
        }

        value
    }
}

impl Mapping {
    fn resolve(&self, value: u64) -> Option<u64> {
        if self.source_range.contains(&value) {
            let offset = value - self.source_range.start();
            Some(self.destination_range_start + offset)
        } else {
            None
        }
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[test]
fn day5_part1() {
    let almanac = almanac_parser().parse(TEST_INPUT).unwrap();

    assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
    assert_eq!(almanac.seed_to_soil_map.resolve(79), 81);
    assert_eq!(almanac.seed_to_soil_map.resolve(14), 14);
    assert_eq!(almanac.seed_to_soil_map.resolve(55), 57);
    assert_eq!(almanac.seed_to_soil_map.resolve(13), 13);

    assert_eq!(almanac.resolve_seed_location(79), 82);
    assert_eq!(almanac.resolve_seed_location(14), 43);
    assert_eq!(almanac.resolve_seed_location(55), 86);
    assert_eq!(almanac.resolve_seed_location(13), 35);
}

#[test]
fn day5_part2() {
    let almanac = almanac_parser().parse(TEST_INPUT).unwrap();

    assert_eq!(almanac.seeds_from_ranges().count(), 27);
    assert_eq!(
        almanac
            .seeds_from_ranges()
            .map(|s| almanac.resolve_seed_location(s))
            .min()
            .unwrap(),
        46
    );
}
