use std::fs;

fn safety_check(input: &str) -> u32 {
    let nums: Vec<Vec<u32>> = input.lines().map(|line| {
        line.split(" ").map(|num| { num.parse::<u32>().expect("failed number conversion") }).collect()
    }).collect();
    
    /*
    let filtered_nums: Vec<Vec<u32>> = nums.iter().filter(|line| {
        line.iter().is_sorted_by(|a,b| a > b) || line.iter().is_sorted_by(|a,b| a < b)
    }).map(|line| line.clone())
    .collect::<Vec<Vec<u32>>>();

    let mut acc: u32 = 0;
    
    //filtered_nums.iter().map(|line| line[1..].iter().zip(line[..line.len()]));
    
    'rows: for row in filtered_nums{
        for i in 0..(row.len() - 1) {
            if (row[i] as i32 - row[i+1] as i32).abs() > 0 && (row[i] as i32 - row[i+1] as i32).abs() <= 3 { continue 'rows; }
        }
        acc += 1;
    }
    */

    let mut acc: u32 = 0;

    'lines: for line in nums {
        if !(line.iter().is_sorted_by(|a, b| a > b ) || line.iter().is_sorted_by(|a, b| a < b)) { continue; }

        for i in 0..(line.len() - 1) {
            let j = i + 1;
            if !((line[i] as i32- line[j] as i32).abs() > 0 && (line[i] as i32 - line[j] as i32).abs() <= 3) {continue 'lines;}
        }

        acc += 1;
    }
    
    return acc;
}

fn dampened_safety_check(input: &str) -> u32 {
    let nums: Vec<Vec<u32>> = input.lines().map(|line| {
        line.split(" ").map(|num| { num.parse::<u32>().expect("failed number conversion") }).collect()
    }).collect();

    let mut acc: u32 = 0;

    /*'lines: for line in nums {
        println!("\n\n\n");
        dbg!(&line);
        if !(line.iter().is_sorted_by(|a, b| a > b ) || line.iter().is_sorted_by(|a, b| a < b)) { println!("unsorted");continue; }

        for skip in -1..(line.len() as isize) {
            println!("skip {skip}");
            let mut j: usize;
            for i in 0..(line.len() - 1) {
                if i as isize == skip { println!("skipped"); continue; }
                if (i + 1) as isize == skip { j = i + 2; }
                else { j = i + 1; } 
                println!("i: {} j: {}", line[i], line[j]);
                if !((line[i] as i32 - line[j] as i32).abs() > 0 && (line[i] as i32 - line[j] as i32).abs() <= 3) {println!("big jump"); continue 'lines;}
            }
            println!("WERE THROUGH");
            acc += 1;
            continue 'lines;
        } 

    } */

    for line in nums {
        /*println!("\n\n\n");
        dbg!(&line);*/
        'sublines: for skip in -1..(line.len() as isize) {
            let mut subline: Vec<u32> = line.clone();
            if skip != -1 { subline.remove(skip as usize); }
            dbg!(&subline);
            if !(subline.iter().is_sorted_by(|a,b| a > b) || subline.iter().is_sorted_by(|a,b| a < b)) { /*println!("unsorted");*/ continue; }
            for i in 0..(subline.len() - 1) {
                let j = i + 1;
                if  !((subline[i] as i32 - subline[j] as i32).abs() > 0 && 
                    (subline[i] as i32 - subline[j] as i32).abs() <= 3) { continue 'sublines; }
            }
            acc += 1;
            break;
        }
    }
    
    return acc;
}

fn main() {
    let data: String = fs::read_to_string("data.txt").expect("file missing!");
    let sum = dampened_safety_check(&data);
    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn puzzle1_works() {
        let result: u32 = safety_check(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn puzzle2_works() {
        let result: u32 = dampened_safety_check(INPUT);
        assert_eq!(result, 4);
    }
}
