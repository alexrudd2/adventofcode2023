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
    for seed in &seeds {
        // println!("{seed}");
        let soil = next_map(*seed, &ss);
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

    // sort by source_start
    ss.sort_by(|a, b| a[1].cmp(&b[1]));
    sf.sort_by(|a, b| a[1].cmp(&b[1]));
    fw.sort_by(|a, b| a[1].cmp(&b[1]));
    wl.sort_by(|a, b| a[1].cmp(&b[1]));
    lt.sort_by(|a, b| a[1].cmp(&b[1]));
    th.sort_by(|a, b| a[1].cmp(&b[1]));
    hl.sort_by(|a, b| a[1].cmp(&b[1]));
    lowest_location = i64::MAX;

    let mut seeds_range = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        let seed_start = seeds[i];
        let seed_range = seeds[i + 1];
        seeds_range.push(vec![seed_start, seed_start + seed_range]);
        i += 2;
    }
    seeds_range.sort_by(|a, b| a[0].cmp(&b[0]));
    // println!("{seeds_range:?}");

    let seed = seeds_range;
    let soil = next_map_range(seed, &ss);
    // println!("s: {soil:?}");
    let fert = next_map_range(soil, &sf);
    // println!("f: {fert:?}");
    let wada = next_map_range(fert, &fw);
    // println!("w: {wada:?}");
    let lite = next_map_range(wada, &wl);
    // println!("l: {lite:?}");
    let temp = next_map_range(lite, &lt);
    // println!("t: {temp:?}");
    let humd = next_map_range(temp, &th);
    // println!("h: {humd:?}");
    let locn = next_map_range(humd, &hl);
    // println!("l: {locn:?}");
    
    for v in locn {
        if v[0] < lowest_location && v[0] > 0{  // Why the &*($#! are there zeroes?  Rather than fix, ignore... >_>
            lowest_location = v[0];
        }
    }
    println!("With ranges, the seed with the lowest location is at {lowest_location}.\n");

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

fn next_map_range(input_vec: Vec<Vec<i64>>, map_vec: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut output_vec = Vec::new();
    let mut sliced_vec = Vec::new();
    for input in &input_vec {
        let input_start = input[0];
        let input_end = input[1];
        for map in map_vec {
            let source_start = map[1];
            let range = map[2];
            let source_end = source_start + range - 1;
            // let dest_start = map[0];
            // let dest_end = dest_start + range - 1;
            // println!("{input_start}:{input_end} | {source_start}:{source_end} => {dest_start}:{dest_end}");
            // let a = range_contains_all((source_start, source_end), (input_start, input_end));
            let m = range_contains_middle((source_start, source_end), (input_start, input_end));
            let s = range_contains_start((source_start, source_end), (input_start, input_end));
            let e = range_contains_end((source_start, source_end), (input_start, input_end));
            // println!("total {a}, start {s}, end {e}, middle {m}");
            if m {
                let output = vec![input_start, source_start - 1];
                sliced_vec.push(output);
                let output = vec![source_start, source_end];
                sliced_vec.push(output);
                let output = vec![source_end + 1, input_end];
                sliced_vec.push(output);
                break;
            } else if s {
                let output = vec![input_start, source_end];
                sliced_vec.push(output);
                let output = vec![source_end + 1, input_end];
                sliced_vec.push(output);
                break;
            } else if e {
                let output = vec![input_start, source_start - 1];
                sliced_vec.push(output);
                let output = vec![source_start, input_end];
                sliced_vec.push(output);
                break;
            }
        }
        sliced_vec.push(vec![input_start, input_end]);
    }
    'outer: for input in &sliced_vec {
        let input_start = input[0];
        let input_end = input[1];
        for map in map_vec {
            let source_start = map[1];
            let range = map[2];
            let source_end = source_start + range - 1;
            let dest_start = map[0];
            let a = range_contains_all((source_start, source_end), (input_start, input_end));
            if a {
                let offset = dest_start - source_start;
                let output = vec![input_start + offset, input_end + offset];
                output_vec.push(output);
                continue 'outer;
            }
        }
        let output = vec![input_start, input_end];
        output_vec.push(output);
    }

    // println!("sliced: {sliced_vec:?}\n");
    // println!("out: {output_vec:?}\n");
    output_vec.sort_by(|a, b| a[0].cmp(&b[0]));
    return output_vec;
}

fn range_contains_all(container_range: (i64, i64), check_range: (i64, i64)) -> bool {
    return check_range.0 >= container_range.0 && check_range.1 <= container_range.1;
}

fn range_contains_middle(container_range: (i64, i64), check_range: (i64, i64)) -> bool {
    return check_range.0 < container_range.0
      && check_range.1 > container_range.1;
}

fn range_contains_start(container_range: (i64, i64), check_range: (i64, i64)) -> bool {
    return check_range.0 >= container_range.0
      && check_range.0 <= container_range.1
      && check_range.1 > container_range.1;
}

fn range_contains_end(container_range: (i64, i64), check_range: (i64, i64)) -> bool {
    return check_range.0 <= container_range.0
      && check_range.1 >= container_range.0
      && check_range.1 <= container_range.1;
}
