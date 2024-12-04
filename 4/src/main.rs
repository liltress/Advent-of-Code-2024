use std::{collections::VecDeque, fs::read_to_string, string};
use diagonal;

fn string_parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map( |line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>()
}

fn horz_tally(data: Vec<Vec<char>>) -> u32 {
    let mut acc: u32 = 0;
    let mut word: VecDeque<char> = VecDeque::with_capacity(5);
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            word.push_front(data[i][j]);
            if word.len() == 5 { word.pop_back(); }
            //dbg!(&word);
            if word.iter().collect::<String>() == "XMAS".to_string() || word.iter().collect::<String>() == "SAMX".to_string() {
                acc += 1;
            }
        }
        word = VecDeque::with_capacity(5);
    }

    acc
}

fn vert_tally(data: Vec<Vec<char>>) -> u32 {
    let mut acc: u32 = 0;
    let mut word: VecDeque<char> = VecDeque::with_capacity(5);
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            word.push_front(data[j][i]);
            if word.len() == 5 { word.pop_back(); }
            //dbg!(&word);
            if word.iter().collect::<String>() == "XMAS".to_string() || word.iter().collect::<String>() == "SAMX".to_string() {
                acc += 1;
            }
        }
        word = VecDeque::with_capacity(5);
    }

    acc
}

fn diag_tally(data: Vec<Vec<char>>) -> u32 {
    let mut acc: u32 = 0;
    let mut word:VecDeque<char> = VecDeque::with_capacity(5);

    let diags = diagonal::diagonal_pos_pos(&data);
    for chars in diags {
        for c in chars {
            word.push_front(*c);
            if word.len() == 5 { word.pop_back(); }
            if word.iter().collect::<String>() == "XMAS".to_string() || word.iter().collect::<String>() == "SAMX".to_string() {
                acc += 1;
            }
        }
        word = VecDeque::with_capacity(5);
    }
    let diags = diagonal::diagonal_pos_neg(&data);
    for chars in diags {
        for c in chars {
            word.push_front(*c);
            if word.len() == 5 { word.pop_back(); }
            if word.iter().collect::<String>() == "XMAS".to_string() || word.iter().collect::<String>() == "SAMX".to_string() {
                acc += 1;
            }
        }
        word = VecDeque::with_capacity(5);
    }
    acc
}
    
fn tally(data: Vec<Vec<char>>) -> u32 {
    horz_tally(data.clone()) + vert_tally(data.clone()) + diag_tally(data)
}

fn X_tally(data: Vec<Vec<char>>) -> u32 {
    let mut acc: u32 = 0;
    let mut m_acc = 0;
    let mut s_acc = 0;
    let offsets: Vec<(i32,i32)> = vec![(-1, -1), (-1, 1),
                                     (1,  -1), (1,  1)];

    for i in 1..(data.len() - 1) {
        for j in 1..(data[0].len() - 1) {
            if data[i][j] == 'A' {
                
                for ofs in &offsets {
                    if data[(i as i32 + ofs.0) as usize][(j as i32 + ofs.1) as usize] == 'M' { m_acc += 1; }
                    if data[(i as i32 + ofs.0) as usize][(j as i32 + ofs.1) as usize] == 'S' { s_acc += 1; }
                }
                
                if (m_acc == 2) && (s_acc == 2) && 
                data[(i as i32 + 1) as usize][(j as i32 + 1) as usize] != data[(i as i32 - 1) as usize][(j as i32 - 1) as usize]
                { acc += 1; /*println!("{i} {j}");*/ }
                m_acc = 0;
                s_acc = 0;
            }
        }
    }

    acc
}

fn main() {
    let data = read_to_string("data.txt").expect("file missing!");
    let chars = string_parse(&data);
    let count = X_tally(chars);
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = 
"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
    static INPUT2: &str = 
".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

    #[test]
    fn parse_works() {
        let res = string_parse(INPUT);
        dbg!(&res);
        assert_eq!(res, vec![
            ['.','.','.','.','X','X','M','A','S','.'],
            ['.','S','A','M','X','M','S','.','.','.'],
            ['.','.','.','S','.','.','A','.','.','.'],
            ['.','.','A','.','A','.','M','S','.','X'],
            ['X','M','A','S','A','M','X','.','M','M'],
            ['X','.','.','.','.','.','X','A','.','A'],
            ['S','.','S','.','S','.','S','.','S','S'],
            ['.','A','.','A','.','A','.','A','.','A'],
            ['.','.','M','.','M','.','M','.','M','M'],
            ['.','X','.','X','.','X','M','A','S','X'],]
        );
    }

    #[test]
    fn horz_tally_works() {
        let chars = string_parse(INPUT);
        let res = horz_tally(chars);
        assert_eq!(res, 5);
    }

    #[test]
    fn vert_tally_works() {
        let chars = string_parse(INPUT);
        let res = vert_tally(chars);
        assert_eq!(res, 3);
    }

    #[test]
    fn diag_tally_works() {
        let chars = string_parse(INPUT);
        let res = diag_tally(chars);
        assert_eq!(res, 10);
    }

    #[test]
    fn tally_works() {
        let chars = string_parse(INPUT);
        let res = tally(chars);
        assert_eq!(res, 18);
    }

    #[test]
    fn x_tally() {
    let chars = string_parse(INPUT2);
    let res = X_tally(chars);
    assert_eq!(res, 9);
    }
}
