use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let result = read_file_into_two_arrays("./day1.txt");

    // I need to sum the differences between the two arrays

    match result {
        Ok((mut array1, mut array2)) => {
            array1.sort();
            array2.sort();

            let differences: i32 = array1
                .iter()
                .zip(array2.iter())
                .map(|(a, b)| (a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap()).abs())
                .sum();

            println!("Differences: {:?}", differences);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn read_file_into_two_arrays(path: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut array1: Vec<String> = Vec::new();
    let mut array2: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line: String = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        array1.push(parts[0].to_string());
        array2.push(parts[1].to_string());
    }

    return Ok((array1, array2));
}
