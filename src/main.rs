use rand::prelude::*;

fn main() {
    let mut arr1: Vec<Vec<i32>> = vec![vec![0; 5]; 3];
    let mut arr2: Vec<Vec<i32>> = vec![vec![0; 5]; 3];

    let vecs = vec![&mut arr1, &mut arr2];

    for vec in vecs {
        random_push_to_vec(vec);
        display_vec(vec);
        println!("----------");
        let index_columns = matching_vec(vec);

        if index_columns.is_empty() {
            println!("Positive columns not found! :(");
            continue;
        }

        println!("Positive columns index: {}", index_columns.join(", "));
    }
}

fn random_push_to_vec(vec: &mut Vec<Vec<i32>>) {
    let mut rng = rand::thread_rng();

    for row in vec {
        for column in row {
            let num = rng.gen_range(-10..10);
            *column = num;
        }
    }
}

fn display_vec(vec: &Vec<Vec<i32>>) {
    for row in vec {
        for column in row {
            if *column <= -1 {
                print!("{}\t", column);
                continue;
            }
            print!(" {}\t", column);
        }
        println!();
    }
}

fn matching_vec(vec: &Vec<Vec<i32>>) -> Vec<String> {
    let mut count_positive_columns: Vec<String> = Vec::new();

    for column in 0..vec[0].len() {
        let mut count_positive_numbers = 0;
        for row in vec {
            if row[column] >= 0 {
                count_positive_numbers += 1;
            }
        }
        
        if count_positive_numbers == vec.len() {
            let index = column as i32;
            count_positive_columns.push(index.to_string());
        }
    }
    
   count_positive_columns
}