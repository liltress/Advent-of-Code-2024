use std::fs;
use std::time::Instant;

fn sum_difference(input: &String) -> i32 { //puzzle 1
    let mut acc: i32 = 0;
    let mut nums1 = Vec::<i32>::new();
    let mut nums2 = Vec::<i32>::new();
    
    for line in input.lines() {
        let mut pair = line.split("   ");
        nums1.push( pair.next().expect("oopsie").parse::<i32>().unwrap() );
        nums2.push( pair.next().expect("oopsie").parse::<i32>().unwrap() );
    }

    nums1.sort();
    nums2.sort();

    for i in 0..nums1.len() {
        acc += (nums1[i] - nums2[i]).abs();
    }

    return acc;
}

fn similarity_score_sum(input: &String) -> i32 { //puzzle 2
    let mut acc: i32 = 0;
    let mut nums1 = Vec::<i32>::new();
    let mut nums2 = Vec::<i32>::new();

    for line in input.lines() {
        let mut pair = line.split("   ");
        nums1.push( pair.next().expect("oopsie").parse::<i32>().unwrap() );
        nums2.push( pair.next().expect("oopsie").parse::<i32>().unwrap() );
    }

    for n in &nums1 {
        let mut count = 0;
        for n2 in &nums2 {
            if n == n2 { count += 1; } 
        }
        acc += n*count;
    }

    return acc;

}

fn main() {
    
    let mut load_times = Vec::<u128>::new();
    let mut op1_times = Vec::<u128>::new();
    let mut op2_times = Vec::<u128>::new();
    let mut total_times = Vec::<u128>::new();
    


    const NUM_OF_TESTS: u128 = 10_000;
    
    for _ in 0..NUM_OF_TESTS {

    let start = Instant::now();

    let now = Instant::now();
    let data: String = fs::read_to_string("data.txt").expect("file missing!");
    //println!("loading numbers: {}", now.elapsed().as_nanos());
    load_times.push(now.elapsed().as_micros().try_into().unwrap());

    let now = Instant::now();
    let _sum = similarity_score_sum(&data);
    //println!("operation 1: {} ns", now.elapsed().as_nanos());
    //println!("{}", sum);
    op1_times.push(now.elapsed().as_micros().try_into().unwrap());
    
    let now = Instant::now();
    let _sum = sum_difference(&data);
    //println!("{}", sum);
    //println!("operation2: {} ns", now.elapsed().as_nanos());
    op2_times.push(now.elapsed().as_micros().try_into().unwrap());

    //println!("total: {} ns", start.elapsed().as_nanos());
    total_times.push(start.elapsed().as_micros().try_into().unwrap());
    }
    println!("ran {} tests", NUM_OF_TESTS);
    println!("load times:  {} μs", (load_times.iter().sum::<u128>())/NUM_OF_TESTS);
    println!("op1 times:   {} μs", (op1_times.iter().sum::<u128>())/NUM_OF_TESTS);
    println!("op2 times:   {} μs", (op2_times.iter().sum::<u128>())/NUM_OF_TESTS);
    println!("total times: {} μs", (total_times.iter().sum::<u128>())/NUM_OF_TESTS);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn puzzle1_works() {
        let result = sum_difference(INPUT.to_string());
        assert_eq!(result, 11);
    }

    #[test]
    fn puzzle2_works() {
        let result = similarity_score_sum(INPUT.to_string());
        assert_eq!(result, 31)
    }

}

