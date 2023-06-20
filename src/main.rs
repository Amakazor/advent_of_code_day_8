use std::fs;

fn visible_trees(fields: &Vec<Vec<u32>>) -> usize {
    let mut visible = fields.len() * 2 + fields[0].len() * 2 - 4;
    for x in 1..fields.len() - 1 {
        'next: for y in 1..fields[x].len() - 1 {
            let current_size = fields[x][y];
            for minus_x in 0..x  {
                if fields[minus_x][y] >= current_size {
                    break;
                } else if minus_x == x - 1 {
                    visible += 1;
                    continue 'next;
                }
            }
            for plus_x in x+1..fields.len()  {
                if fields[plus_x][y] >= current_size {
                    break;
                } else if plus_x == fields.len() - 1 {
                    visible += 1;
                    continue 'next;
                }
            }
            for minus_y in 0..y  {
                if fields[x][minus_y] >= current_size {
                    break;
                } else if minus_y == y - 1 {
                    visible += 1;
                    continue 'next;
                }
            }
            for plus_y in y+1..fields.len()  {
                if fields[x][plus_y] >= current_size {
                    break;
                } else if plus_y == fields.len() - 1 {
                    visible += 1;
                    continue 'next;
                }
            }
        }
    };
    
    return visible;
}

fn highest_scenic_score(fields: &Vec<Vec<u32>>) -> u32 {
    let mut current_highest_scenic_score = 0;

    for x in 1..fields.len() - 1 {
        for y in 1..fields[x].len() - 1 {
            let current_size = fields[x][y];
            let mut trees_scenic_score:u32 = 1;
            
            let mut seen_trees_in_direction:u32 = 1;
            for coord in (0..x).rev()  {
                if fields[coord][y] >= current_size || coord == 0 || coord == fields.len() - 1 {
                    break;
                }
                seen_trees_in_direction += 1;
            }
            trees_scenic_score *= seen_trees_in_direction ;
            seen_trees_in_direction = 1;
            for coord in x+1..fields.len()  {
                if fields[coord][y] >= current_size || coord == 0 || coord == fields.len() - 1 {
                    break;
                }
                seen_trees_in_direction += 1;
            }
            trees_scenic_score *= seen_trees_in_direction ;
            seen_trees_in_direction = 1;
            for coord in (0..y).rev()  {
                if fields[x][coord] >= current_size || coord == 0 || coord == fields[y].len() - 1 {
                    break;
                }
                seen_trees_in_direction += 1;
            }
            trees_scenic_score *= seen_trees_in_direction ;
            seen_trees_in_direction = 1;
            for coord in y+1..fields.len()  {
                if fields[x][coord] >= current_size || coord == 0 || coord == fields[y].len() - 1 {
                    break;
                }
                seen_trees_in_direction += 1;
            }
            trees_scenic_score *= seen_trees_in_direction ;
        
            if trees_scenic_score > current_highest_scenic_score {
                current_highest_scenic_score = trees_scenic_score;
            }
        }
    };
    
    return current_highest_scenic_score;
}

fn main() {
    let data = fs::read_to_string("test-data.txt").unwrap();
    let fields:Vec<Vec<u32>> = data.lines().map(|line| line.chars().map(|char| char.to_digit(10).unwrap_or(0)).collect()).collect();
    
    println!("Visible trees: {}", visible_trees(&fields));
    println!("Highest scenic score: {}", highest_scenic_score(&fields))
}
