#[path = "../util/util.rs"] mod util;

pub fn main() {
    println!("Day 5: If You Give A Seed A Fertilizer");
    #[derive(Debug)]
    enum Map {
        SeedToSoil,
        SoilToFertilizer,
        FertilizerToWater,
        WaterToLight,
        LightToTemp,
        TempToHumidity,
        HumidityToLocation,
    }
    let mut seeds: Vec<i64> = Vec::new();
    let mut ss = Vec::new();
    let mut sf = Vec::new();
    let mut fw = Vec::new();
    let mut wl = Vec::new();
    let mut lt = Vec::new();
    let mut th = Vec::new();
    let mut hl = Vec::new();
    let mut current_map = Map::SeedToSoil;
    for line in util::read_input_iter("src/day5/input.txt") {
        if line.contains("seeds:") {
            seeds = str_to_i64_vec(&line[7..]);
            continue;
        }
        match line.as_str() {
            "seed-to-soil map:" => current_map = Map::SeedToSoil,
            "soil-to-fertilizer map:" => current_map = Map::SoilToFertilizer,
            "fertilizer-to-water map:" => current_map = Map::FertilizerToWater,
            "water-to-light map:" => current_map = Map::WaterToLight,
            "light-to-temperature map:" => current_map = Map::LightToTemp,
            "temperature-to-humidity map:" => current_map = Map::TempToHumidity,
            "humidity-to-location map:" => current_map = Map::HumidityToLocation,
            "" => (),
            _ => {
                let vec = str_to_i64_vec(&line);
                match current_map {
                    Map::SeedToSoil => ss.push(vec),
                    Map::SoilToFertilizer => sf.push(vec),
                    Map::FertilizerToWater => fw.push(vec),
                    Map::WaterToLight => wl.push(vec),
                    Map::LightToTemp => lt.push(vec),
                    Map::TempToHumidity => th.push(vec),
                    Map::HumidityToLocation => hl.push(vec),
                    _ => panic!()
                }
            }
        }
    }
    // println!("{seeds:?}");
    // println!("{ss:?}");
    // println!("{sf:?}");
    // println!("{fw:?}");
    // println!("{wl:?}");
    // println!("{lt:?}");
    // println!("{th:?}");
    // println!("{hl:?}");
    
    let mut lowest_location = i64::MAX;
    for seed in seeds {
        // println!("{seed}");
        let soil = next_map(seed, &ss);
        // println!("s: {soil}");
        let fert = next_map(soil, &sf);
        // println!("f: {fert}");
        let wada = next_map(fert, &fw);
        // println!("w: {wada}");
        let lite = next_map(wada, &wl);
        // println!("l: {lite}");
        let temp = next_map(lite, &lt);
        // println!("t: {temp}");
        let humd = next_map(temp, &th);
        // println!("h: {humd}");
        let locn = next_map(humd, &hl);
        // println!("l: {locn}");
        
        if locn < lowest_location {
            lowest_location = locn;
        }
    }
    println!("The seed with the lowest location is at {lowest_location}.");
}

fn str_to_i64_vec(s: &str) -> Vec<i64> { // todo: use generics?
    let v: Vec<&str> = s.split_whitespace().collect();
    return v.iter().map(|&s| s.parse().expect("Tried to parse non-integer value")).collect();
}

fn next_map(value: i64, v: &Vec<Vec<i64>>) -> i64 {
    for i in v {
        let source_start = i[1];
        let dest_start = i[0];
        let range_length = i[2];
        let delta = value - source_start;
        if delta >= 0 && delta <= range_length {
            return delta + dest_start;
        }
    }
    return value;
}
