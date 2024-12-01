use std::fs;

fn sum_difference(input: String) -> i32 { //puzzle 1
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

fn simmularity_score_sum(input: String) -> i32 { //puzzle 2
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
    let data: String = fs::read_to_string("data.txt").expect("file missing!");
    let sum = simmularity_score_sum(data);
    println!("{}", sum);
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
        let result = simmularity_score_sum(INPUT.to_string());
        assert_eq!(result, 31)
    }

}

