pub fn main() {
    println!("Day 6: Wait for It");

    // example
    // let times = vec![7, 15, 30];
    // let dists = vec![9, 40, 200];
    let times = vec![55, 99, 97, 93];
    let dists = vec![401, 1485, 2274, 1405];
    let mut margin_of_error = 1;
    for i in 0..times.len() {
        let mut sum = 0;
        let total_time = times[i];
        let distance_to_beat = dists[i];
        for wait_time in 1..total_time {
            let distance = (total_time - wait_time) * wait_time;
            if distance > distance_to_beat {
                sum += 1;
            }
        }
        margin_of_error *= sum;
    }

    println!("The 'error margin' is: {margin_of_error}.");
    println!("Assuming no elves try to sink your boat, that is...");

    let mut sum: i64 = 0;
    let total_time: i64 = 55999793;
    let distance_to_beat: i64 = 401148522741405;
    for wait_time in 1..total_time {
        let distance = (total_time - wait_time) * wait_time;
        if distance > distance_to_beat {
            sum += 1;
        }
    }
    println!("The big race has a corresponding error margin of {sum}.");
}
