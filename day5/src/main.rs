use std::ops::Range;

#[derive(Debug, Clone, Default)]
struct Map {
    range: Range<usize>,
    destination: usize,
}

impl Map {
    fn get_transform(&self, seed: &usize) -> Option<usize> {
        if self.range.contains(seed) {
            Some(self.destination + (seed - self.range.start))
        } else {
            None
        }
    }

    fn update(&mut self, range: Range<usize>, destination: usize) {
        self.range = range;
        self.destination = destination;
    }
}

#[derive(Debug, Clone, Default)]
struct TransformTable {
    maps: Vec<Map>,
}

impl TransformTable {
    fn get_transform(&self, seed: usize) -> usize {
        for map in &self.maps {
            if let Some(transform) = map.get_transform(&seed) {
                return transform;
            }
        }
        seed
    }

    fn push(&mut self, map: Map) {
        self.maps.push(map)
    }
}

#[derive(Debug, Clone, Default)]
struct Almanac {
    transform_tables: Vec<TransformTable>,
}

impl Almanac {
    fn get_location(&self, seed: usize) -> usize {
        let mut transformed_val = seed;

        for table in &self.transform_tables {
            print!("{transformed_val} -> ");
            transformed_val = table.get_transform(transformed_val);
        }
        print!("{transformed_val}");
        transformed_val
    }

    fn get_all_locations(&self, seeds: Vec<usize>) -> Vec<usize> {
        let mut transforms = Vec::new();
        for seed in seeds {
            transforms.push(self.get_location(seed));
            println!();
        }
        transforms
    }

    fn get_lowest_location(&self, seeds: Vec<usize>) -> usize {
        *self.get_all_locations(seeds).iter().min().unwrap()
    }

    fn push(&mut self, transform_table: TransformTable) {
        self.transform_tables.push(transform_table)
    }
}

fn extract_data(content: String) -> (Almanac, Vec<usize>) {
    let re = regex::Regex::new(r"\b\d+\b").unwrap();
    let mut lines = content.lines();
    let seeds = re
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut almanac = Almanac::default();

    let mut current_transform_table = TransformTable::default();
    for line in lines.skip(1) {
        if line.is_empty() {
            almanac.push(current_transform_table.clone());
        } else if line.contains("map") {
            current_transform_table = TransformTable::default();
        } else {
            let mut current_map = Map::default();
            let numbers = re
                .find_iter(line)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            if numbers.len() != 3 {
                panic!("Invalid mapping line: {}", line);
            }

            current_map.update(numbers[1]..numbers[1] + numbers[2], numbers[0]);
            current_transform_table.push(current_map);
        }
    }

    // so that the last transform is loaded into the almanac
    if !current_transform_table.maps.is_empty() {
        almanac.push(current_transform_table);
    }

    (almanac, seeds)
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let (almanac, seeds) = extract_data(content);

    println!("{seeds:#?}\n\n");
    println!("{almanac:#?}");

    let lowest_location = almanac.get_lowest_location(seeds);
    println!("Lowest Location for a seed: {lowest_location}")
}
