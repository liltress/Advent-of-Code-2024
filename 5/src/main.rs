use std::{collections::HashMap, fs::read_to_string};

fn parse_data(data: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let datas: Vec<&str> = data.split("&").collect::<Vec<&str>>();

    let rules: Vec<Vec<u32>> = datas[0]
        .lines()
        .map(|line| {
            line.split("|")
                .map(|n| n.parse::<u32>().expect("failed number conversion 1"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let rules: Vec<(u32, u32)> = rules.iter().map(|pair| (pair[0], pair[1])).collect();

    let nums_str: &str = &datas[1][1..datas[1].len()];
    let nums: Vec<Vec<u32>> = nums_str
        .lines()
        .map(|line| {
            //dbg!(line);
            line.split(",")
                .map(|n| n.parse::<u32>().expect("failed number conversion 2"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    (rules, nums)
}

fn compress_data(
    data: (Vec<(u32, u32)>, Vec<Vec<u32>>),
) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut hash: HashMap<u32, Vec<u32>> = HashMap::new();

    for pair in data.0 {
        if hash.contains_key(&pair.1) {
            hash.get_mut(&pair.1)
                .expect("failed to get value ref")
                .push(pair.0.clone());
        } else {
            let check = &hash.insert(pair.1, vec![pair.0.clone()]);
            if *check != None {
                println!("key mistakably overwritten");
            }
        }
    }

    (hash, data.1)
}
fn row_obeys(data: (&HashMap<u32, Vec<u32>>, &Vec<u32>)) -> bool {
    let row = data.1;
    for i in 0..row.len() {
        if !data.0.contains_key(&row[i]) {
            continue;
        }
        let requirements: Vec<u32> = data.0.get(&row[i]).expect("failed access").to_vec();
        let predecessors: Vec<u32> = row[0..i].to_vec();
        for req in requirements {
            if !predecessors.contains(&req) && row.contains(&req) {
                return false;
            }
        }
        /*for j in (1..(i)).rev() {
            if !requirements.contains(&row[j]) { continue 'rows; }
        }*/
    }
    return true;
}

fn rule_check(data: (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> Vec<Vec<u32>> {
    let mut ans: Vec<Vec<u32>> = Vec::new();
    for row in &data.1 {
        //dbg!(&row);
        if row_obeys((&data.0, &row.to_vec())) {
            ans.push(row.clone());
        }
    }
    ans
}

fn rule_check_opp(
    data: (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>),
) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut ans: Vec<Vec<u32>> = Vec::new();
    for row in data.1 {
        if !row_obeys((&data.0, &row.to_vec())) {
            ans.push(row.clone());
        }
    }
    (data.0, ans)
}

fn rule_sort(data: (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> Vec<Vec<u32>> {
    let mut ans: Vec<Vec<u32>> = Vec::new();
    let mut new_row: Vec<u32>;
    //dbg!(&data.0);
    for row in data.1 {
        new_row = row.clone();
        while !row_obeys((&data.0, &new_row)) {
            //println!("\n");
            for i in 0..new_row.len() {
                for j in 0..i {
                    //println!("{:?}", &new_row); //[53, 97, 61, 47, 75]
                    //println!("{0}: {2}   {1}: {3}", i, j, &row[i], &row[j]);
                    if new_row[i] == new_row[j] {
                        continue;
                    }
                    if data.0.contains_key(&new_row[j])
                        && data.0
                            .get(&new_row[j])
                            .expect("failed to get value")
                            .contains(&new_row[i])
                    {
                        new_row.swap(i, j);
                        //println!("swap!");
                    }
                }
            }
        }
        ans.push(new_row);
    }
    ans
}

fn mid_sum(data: Vec<Vec<u32>>) -> u32 {
    let mut acc: u32 = 0;
    for row in data {
        acc += row[row.len() / 2];
    }
    acc
}

const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
&
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn main() {
    let data = read_to_string("data.txt").expect("file missing!");
    let data = parse_data(&data);
    let data = compress_data(data);
    let data = rule_check_opp(data);
    let data = rule_sort(data);
    let res = mid_sum(data);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
&
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn parse_works() {
        let res = parse_data(INPUT);
        dbg!(&res);
        let test: (Vec<(u32, u32)>, Vec<Vec<u32>>) = (
            vec![
                (47, 53),
                (97, 13),
                (97, 61),
                (97, 47),
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ],
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
        );
        assert_eq!(res, test);
    }

    #[test]
    fn compression_works() {
        let data = parse_data(INPUT);
        let res = compress_data(data);
        dbg!(&res);

        let mut test1: HashMap<u32, Vec<u32>> = HashMap::new();

        test1.insert(53, vec![47, 75, 61, 97]);
        test1.insert(47, vec![97, 75]);
        test1.insert(13, vec![97, 61, 29, 47, 75, 53]);
        test1.insert(61, vec![97, 47, 75]);
        test1.insert(29, vec![75, 97, 53, 61, 47]);
        test1.insert(75, vec![97]);
        let test2: Vec<Vec<u32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        assert_eq!(res, (test1, test2));
    }

    #[test]
    fn rule_check_works() {
        let data = parse_data(INPUT);
        let data = compress_data(data);
        let res = rule_check(data);
        dbg!(&res);
        assert_eq!(
            res,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
            ]
        );
    }
    #[test]
    fn division_works_how_i_think_it_does() {
        assert_eq!((5 as usize / 2) as u32 + 1, 3);
    }

    #[test]
    fn part1_works() {
        let data = parse_data(INPUT);
        let data = compress_data(data);
        let data = rule_check(data);
        let res = mid_sum(data);

        assert_eq!(res, 143);
    }

    #[test]
    fn reverse_rule_check_works() {
        let data = parse_data(INPUT);
        let data = compress_data(data);
        let res = rule_check_opp(data);
        dbg!(&res);
        assert_eq!(
            res.1,
            vec![
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ]
        );
    }

    #[test]
    fn rule_sort_check() {
        let data = parse_data(INPUT);
        let data = compress_data(data);
        let data = rule_check_opp(data);
        let res = rule_sort(data);
        dbg!(&res);
        assert_eq!(res, 
            vec![
                vec![97,75,47,61,53],
                vec![61,29,13],
                vec![97,75,47,29,13],
            ]
        );
    }

    #[test]
    fn part2_works() {
        let data = parse_data(INPUT);
        let data = compress_data(data);
        let data = rule_check_opp(data);
        let data = rule_sort(data);
        let res = mid_sum(data);
        assert_eq!(res, 123);
    }
}
