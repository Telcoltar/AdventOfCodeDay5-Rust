use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use log::{info, debug};

fn read_input_data(file_name: &str) -> io::Result<Vec<String>>{
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    let mut codes:Vec<String> = Vec::new();

    for line in f.lines() {
        codes.push(line.unwrap());
    }

    Ok(codes)
}

fn decode(code: &str) -> (i32, i32) {
    let row_str = &code[0..7];
    let column_str = &code[7..];
    debug!("Row-String: {}, Column-String: {}", row_str, column_str);
    let row: i32 = convert_str_to_int(row_str, 2, 'B');
    debug!("Row: {}", row.to_string());
    let column: i32 = convert_str_to_int(column_str, 2, 'R');
    debug!("Column: {}", column.to_string());
    return (row, column);
}

fn convert_str_to_int(repr: &str, base: i32, one_indicator: char) -> i32 {
    let rev_repr:String = repr.chars().rev().collect();
    debug!("Reversed String: {}", rev_repr);
    let mut result = 0;
    let mut current_exponent = 0;
    for char in rev_repr.chars() {
        if char == one_indicator {
            result += base.pow(current_exponent);
        }
        current_exponent += 1
    }
    return result;
}

fn build_seat_ids_array(file_name: &str) -> Vec<i32> {
    let codes: Vec<String> = read_input_data(file_name).unwrap();
    let mut seat_ids:Vec<i32> = Vec::new();
    for code in codes {
        let (row, column) = decode(&code);
        seat_ids.push(row*8 + column);
    }
    return seat_ids;
}

fn solution_part_1(file_name: &str) -> i32 {
    let mut seat_ids:Vec<i32> = build_seat_ids_array(file_name);
    seat_ids.sort_unstable();
    return seat_ids.last().unwrap().clone();
}

fn solution_part_2(file_name: &str) -> i32 {
    let mut seat_ids:Vec<i32> = build_seat_ids_array(file_name);
    seat_ids.sort_unstable();
    let mut current_index = 0;
    let mut current_id = seat_ids[current_index];
    while current_id+1 == seat_ids[current_index+1] {
        current_index += 1;
        current_id += 1;
    }
    debug!("Neigbors: {}, {}, {}",
           seat_ids[current_index-1],
           seat_ids[current_index],
           seat_ids[current_index+1]);
    return current_id+1;
}

fn main() {
    env_logger::init();
    info!("Highest seat ID: {}",solution_part_1("inputData.txt"));
    info!("My seat ID is {}", solution_part_2("inputData.txt"));
}
