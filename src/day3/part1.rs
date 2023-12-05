
use crate::utils;


fn is_symbol(ch: char) -> bool{
    return ch != '.' && !ch.is_digit(10);
}


fn has_proximity(matrix: &Vec<Vec<char>>, start_idx: usize, end_idx: usize) -> bool {
    
    for mat in matrix {
        if start_idx > 0 {
            if is_symbol(mat[start_idx-1]) {
                return true;
            }
        }
        for n in start_idx..=end_idx {
            if is_symbol(mat[n]) {
                return true;
            }
        }
        if end_idx < mat.len() - 1 {
            if is_symbol(mat[end_idx+1]) {
                return true;
            }
        }
    }
    return false;
}

fn find_parts(matrix: &Vec<Vec<char>>) -> Vec<i32>{
    let mut temp: String = "".to_string();
    let mut idcs: Vec<usize> = vec![];
    let mut parts: Vec<i32> = vec![];
    for (i, ch) in matrix[1].iter().enumerate() {
        if !ch.is_digit(10) || i == matrix[1].len() - 1 {
            if ch.is_digit(10) {
                temp.push(*ch);
                idcs.push(i);
            }
            if temp != "" {         
                if has_proximity(&matrix, idcs[0], idcs[idcs.len()-1]) {
                    parts.push(temp.parse::<i32>().unwrap());
                }
            }
            temp = "".to_string();
            idcs = vec![];
        }
        else {
            temp.push(*ch);
            idcs.push(i);
        }
    }
    return parts;
}

pub fn solve(file_path: &str) {

    println!("In file {}", file_path);
    let row_size: usize = 140;

    let mut _sum: i32 = 0;
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        matrix.push(vec!['.'; row_size]);
        
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                matrix.push(row.chars().collect());
                if i == 0 {
                    continue;
                }
                let parts = find_parts(&matrix);
                _sum += parts.iter().sum::<i32>();
                
                if matrix.len() == 3 {
                    matrix.remove(0);
                }
            }
        }
        matrix.push(vec!['.'; row_size]);
        let parts = find_parts(&matrix);
        _sum += parts.iter().sum::<i32>();
        println!("{}", _sum);
    }
}
