use super::super::read_file;
use std::{thread, sync::Arc};

const DAY: usize = 5; 

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");
    let seeds = Seeds::new(&input);
    let maps = Maps::new(&input);

    let result_pt1 = maps.nearest_location_in_seeds(&seeds.seeds);
    let result_pt2 = maps.nearest_location_in_seed_ranges_bruteforce(&seeds.seeds);
    // let result_pt2 = maps.nearest_location_in_seed_ranges_reverse(&seeds.seeds);

    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}

#[derive(Debug)]
struct Seeds {
    seeds: Vec<usize>
}

impl Seeds {
    fn new(input: &str) -> Self {
        let mut seeds = vec![];

        for number in input.lines().next().unwrap().split(' ').skip(1) {
            seeds.push(number.parse::<usize>().unwrap());    
        }
        Seeds {seeds}
    }
 
    
}




#[derive(Clone, Copy, Debug)]
struct MapRange {
    destination: usize,
    source: usize,
    range: usize
}

impl MapRange {
    fn zero() -> Self {
        MapRange { destination: 0, source: 0, range: 0 }
    }

    fn is_mapped(&self, number: usize) -> Option<usize> {
        if number >= self.source && number - self.source < self.range {
            Some(self.destination + number - self.source)
        } else {
            None
        }
    }

    fn is_mapped_reverse(&self, destination: usize) -> Option<usize> {
        if destination >= self.destination && destination - self.destination < self.range {
            Some(self.source + destination - self.destination)
         } else {
            None
         }
    }
}

#[derive(Debug, Clone)]
struct Map {
    name: String,
    ranges: Vec<MapRange>
}

impl Map {
    fn is_mapped(&self, number: usize) -> Option<usize> {
        self.ranges.iter()
            .find(|range| range.is_mapped(number).is_some())
            .and_then(|r| r.is_mapped(number))    
    }

    fn is_mapped_reverse(&self, number: usize) -> Option<usize> {
        self.ranges.iter().rev()
            .find(|range| range.is_mapped_reverse(number).is_some())
            .and_then(|r| r.is_mapped_reverse(number))    
    }
}

#[derive(Debug, Clone)]
struct Maps {
    maps: Vec<Map>
}

impl Maps {
    fn new(input: &str) -> Self {
        let mut maps = vec![];

        let mut name = String::new();
        let mut ranges = vec![];

        for line in input.lines() {
            // skipping the seeds region
            if line.contains("seeds") {
                continue;
            }
            if line.contains("map") {
                name = line.split(' ').next().unwrap().to_string();
                continue;
            }
            if line.is_empty() {
                if !name.is_empty() {   // first empty line after "seeds" is causing an empty map in maps-vec otherwise
                    maps.push(Map {name: name.clone(), ranges: ranges.clone()});
                    name.clear();
                    ranges.clear();
                }
                continue;
            }
            let mut map_range = MapRange::zero();
            let mut numbers = line.split(' ');

            map_range.destination = numbers.next().unwrap().parse::<usize>().unwrap();
            map_range.source = numbers.next().unwrap().parse::<usize>().unwrap();
            map_range.range = numbers.next().unwrap().parse::<usize>().unwrap();
            ranges.push(map_range);
        }

        // and pushing the last map to the vec since input ends without empty line
        maps.push(Map {name: name.clone(), ranges: ranges.clone()});

        Maps {maps}
    }    

    fn get_seed_location(&self, seed: usize) -> usize {
        let mut destination = seed;
        for map in &self.maps {
            if let Some(next_destinaton) = map.is_mapped(destination) {
                destination = next_destinaton;
            }
        
        }
        destination
    }

    fn get_seed_location_reverse(&self, location: usize) -> usize {
        let mut source = location;
        for map in self.maps.iter().rev() {
            if let Some(next_source) = map.is_mapped_reverse(source) {
                source = next_source;
            }
        
        }
        
        source
    }

    fn nearest_location_in_seeds(&self, seeds: &Vec<usize>) -> usize {
        let mut location = self.get_seed_location(*seeds.iter().next().unwrap());
        for seed in seeds {
            location = location.min(self.get_seed_location(*seed));
        }
        location
    }

    fn nearest_location_in_seed_ranges_bruteforce(&self, seeds: &Vec<usize>) -> usize {

        // takes around 30 min on sinlge core ...
        // takes around 13 min with threads
        
        let mut seed_iter = seeds.iter();
        
        let mut pool = vec![];
        let self_arc = Arc::new(self.clone());
        
        for _ in 0..seeds.len()/2 {
            let seed = *seed_iter.next().unwrap();
            let range = *seed_iter.next().unwrap();
            let self_arc_clone = self_arc.clone();
            
            pool.push(thread::spawn( move || {
                let mut location: Option<usize> = None;
                for i in 0..range {
                    let next_location = self_arc_clone.get_seed_location(seed+i);
                    location = match (location, next_location) {
                        (Some(loc), next_loc) => Some(loc.min(next_loc)),
                        (None, next_loc) => Some(next_loc),
                    };
                    
                }
                location.unwrap()
            }));
        }
        let mut results = vec![];
        for handle in pool {
            results.push(handle.join().unwrap());
        }
        *results.iter().min().unwrap()
        
        
    }

    fn nearest_location_in_seed_ranges_reverse(&self, seeds: &Vec<usize>) -> usize {

        // not checking all seeds but checking all locations until reverse lookup returns location in ranges
        // this implementation needs around 20 sec. But seams to have a bug. We miss the correct result by 4! (returns: 108956231)

        let mut ranges: Vec<(usize, usize)> = vec![];
        let mut seed_iter = seeds.iter();
        for _ in 0..seeds.len()/2 {
            let seed = *seed_iter.next().unwrap();
            let range = *seed_iter.next().unwrap();

            ranges.push((seed, seed+range-1));
        }


        let mut potential_location = 0;
        loop {
            let potential_seed = self.get_seed_location_reverse(potential_location);
            for range in &ranges {
                if potential_seed >= range.0 && potential_seed <= range.1 {
            
                    return potential_location;
                }

                potential_location += 1;
            }
        }
        
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_5() {
        let input = "seeds: 79 14 55 13

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

        let seeds = Seeds::new(input);
        let maps = Maps::new(input);
        // dbg!(&maps);

        assert_eq!(Some(51), maps.maps[0].ranges[0].is_mapped(99));
        assert_eq!(None, maps.maps[0].ranges[0].is_mapped(100));

        assert_eq!(Some(99), maps.maps[0].ranges[0].is_mapped_reverse(51));
        assert_eq!(None, maps.maps[0].ranges[0].is_mapped_reverse(53));

        assert_eq!(Some(50), maps.maps[0].is_mapped(98));
        assert_eq!(Some(51), maps.maps[0].is_mapped(99));
        assert_eq!(None, maps.maps[0].is_mapped(100));

        assert_eq!(Some(99), maps.maps[0].is_mapped_reverse(51));
        assert_eq!(None, maps.maps[0].is_mapped_reverse(100));
        
        assert_eq!(82, maps.get_seed_location(79));
        assert_eq!(43, maps.get_seed_location(14));
        assert_eq!(86, maps.get_seed_location(55));
        assert_eq!(35, maps.get_seed_location(13));


        assert_eq!(35, maps.nearest_location_in_seeds(&seeds.seeds));

        assert_eq!(46, maps.nearest_location_in_seed_ranges_bruteforce(&seeds.seeds));
        assert_eq!(46, maps.nearest_location_in_seed_ranges_reverse(&seeds.seeds));

        
        // part 1
        assert_eq!(0, 0);

        // part 2
        assert_eq!(0, 0);
    }
}

