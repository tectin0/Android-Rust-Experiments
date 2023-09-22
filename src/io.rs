use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::Path,
};

pub(crate) fn write_to_file(dates: Vec<(i32, u32, u32)>) {
    let mut file = fs::File::create("dates.dat").unwrap();

    for (year, month, day) in dates {
        let date = format!("{} {} {}\n", year, month, day);
        file.write_all(date.as_bytes()).unwrap();
    }
}

pub(crate) fn read_from_file() -> Vec<(i32, u32, u32)> {
    let mut dates = Vec::new();

    if !Path::new("dates.dat").exists() {
        return dates;
    }

    let file = fs::File::open("dates.dat").unwrap();
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
