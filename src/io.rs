use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use crate::constants::STORAGE_PATH;

pub(crate) fn write_to_file(dates: Vec<(i32, u32, u32)>) {
    if !Path::new(STORAGE_PATH).exists() {
        fs::create_dir(STORAGE_PATH).unwrap();
    }

    let mut file = fs::File::create(format!("{}/dates.dat", STORAGE_PATH)).unwrap();

    for (year, month, day) in dates {
        let date = format!("{} {} {}\n", year, month, day);
        file.write_all(date.as_bytes()).unwrap();
    }
}

pub(crate) fn read_from_file() -> Vec<(i32, u32, u32)> {
    let mut dates = Vec::new();

    if !Path::new(&format!("{}/dates.dat", STORAGE_PATH)).exists() {
        return dates;
    }

    let file = fs::File::open(format!("{}/dates.dat", STORAGE_PATH)).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();

        let year = split.next().unwrap().parse::<i32>().unwrap();
        let month = split.next().unwrap().parse::<u32>().unwrap();
        let day = split.next().unwrap().parse::<u32>().unwrap();

        dates.push((year, month, day));
    }

    dates
}
