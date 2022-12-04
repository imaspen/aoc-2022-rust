pub(crate) fn read_day(day: u8) -> String {
    let is_test = std::env::var("IS_TEST").unwrap_or("false".to_string()) == "true";
    let str = std::fs::read_to_string(format!(
        "assets/{}{:02}.txt",
        if is_test { "test/" } else { "" },
        day
    ));
    return str.unwrap();
}

pub(crate) fn read_day_lines(day: u8) -> Vec<String> {
    let str = read_day(day);
    return str.lines().map(String::from).collect::<Vec<_>>();
}

pub(crate) fn read_day_csv_lines(day: u8) -> Vec<Vec<String>> {
    let str = read_day(day);
    return str
        .lines()
        .map(|line| line.split(",").map(String::from).collect())
        .collect();
}

pub(crate) fn read_day_grouped_lines(day: u8) -> Vec<Vec<String>> {
    let lines = read_day_lines(day);
    return lines
        .split(|line| line == "")
        .map(Vec::from)
        .collect::<Vec<_>>();
}

pub(crate) fn read_day_grouped_ints(day: u8) -> Vec<Vec<i32>> {
    let groups = read_day_grouped_lines(day);
    return groups
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .map(|str| str.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
}
