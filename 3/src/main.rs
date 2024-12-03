use std::fs;

use regex::Regex;

fn mul_finder(hay: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let mults: Vec<&str> = re.find_iter(hay).map(|m| m.as_str()).collect();

    return mults;
}

fn mul_cruncher(muls: Vec<&str>) -> u64 {
    let nums: Vec<&str> = muls.iter().map(|s| { 
        s.trim_start_matches("mul(").trim_end_matches(")")
    }).collect();

    let mut acc: u64 = 0;

    nums.iter().for_each(|pair| {
        acc += pair.split(",").map(|n| { n.parse::<u64>().expect("failed str -> number conversion!") }).product::<u64>() });

    return acc;
}

fn mul_finder_plus(hay: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let mults: Vec<&str> = re.find_iter(hay).map(|m| m.as_str()).collect();

    return mults;
}

fn mul_cruncher_plus(muls: Vec<&str>) -> u64 {
    let nums: Vec<&str> = muls.iter().map(|s| { 
        s.trim_start_matches("mul(").trim_end_matches(")")
    }).collect();

    let mut acc: u64 = 0;

    let mut do_work: bool = true;

    nums.iter().for_each(|comm| {
        //println!("-> {comm}");
        match comm {
            &"do(" => { /*println!("do");*/ do_work = true; },
            &"don't(" => { /*println!("dont");*/ do_work = false; },
            _ => { if do_work { /*println!("math time");*/ acc += comm.split(",").map(|n| { n.parse::<u64>().expect("failed str -> number conversion!") }).product::<u64>(); } }
        }
    });
    return acc;
}
    
fn main() {
    let data = fs::read_to_string("data.txt").expect("file missing!");
    let muls = mul_finder_plus(&data);
    let sum = mul_cruncher_plus(muls);
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    
    #[test]
    fn muls_found() {
        let res = mul_finder(INPUT);
        dbg!(&res);
        assert_eq!(res, vec![
            "mul(2,4)",
            "mul(5,5)",
            "mul(11,8)",
            "mul(8,5)",
        ]);
    }

    #[test]
    fn muls_processed() {
        let muls = mul_finder(INPUT);
        let res = mul_cruncher(muls);
        assert_eq!(res, 161);
    }

    #[test]
    fn muls_found2() {
        let res = mul_finder_plus(INPUT2);
        dbg!(&res);
        assert_eq!(res, vec![
            "mul(2,4)",
            "don't()",
            "mul(5,5)",
            "mul(11,8)",
            "do()",
            "mul(8,5)",
        ]);
    }

    #[test]
    fn muls_processed2() {
        let muls = mul_finder_plus(INPUT2);
        let res = mul_cruncher_plus(muls);
        assert_eq!(res, 48 as u64);
    }
}
