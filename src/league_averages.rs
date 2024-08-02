use std::collections::HashMap;
use std::iter::zip;
use regex::{Regex};

macro_rules! batting_url {
    () => {
        ("https://www.baseball-reference.com/leagues/majors/bat.shtml",
        vec!["Tms", "#Bat", "BatAge", "R/G", "G", "PA", "AB", "R", "H", "1B", "2B", "3B", "HR", "RBI", "SB", "CS", "BB",
        "SO", "BA", "OBP", "SLG", "OPS", "TB", "GDP", "HBP", "SH", "SF", "IBB", "BIP"])
    };
}

macro_rules! pitching_url {
    () => {
        ("https://www.baseball-reference.com/leagues/majors/pitch.shtml",
        vec!["Tms", "#P", "PAge", "R/G", "ERA", "G", "GF", "CG", "SHO", "tSHO", "SV", "IP", "H", "R", "ER", "HR",
        "BB", "IBB", "SO", "HBP", "BK", "WP", "BF", "WHIP", "BAbip", "H9", "HR9", "BB9", "SO9", "SO/W", "E"])
    };
}

pub(crate) fn get_league_averages(is_batting: bool) -> reqwest::Result<HashMap<u32, HashMap<String, f64>>> {
    let (url, stat_categories) = if is_batting { batting_url!() }
    else { pitching_url!() };

    let stats = reqwest::blocking::get(url)?.text()?;

    let table_regex = Regex::new(r"<tbody>[\S\s]*?League Year-By-Year").unwrap();
    let row_regex = Regex::new(r"<tr >(<th.*?</th>).*?</tr>").unwrap();
    let col_regex = Regex::new(r">([\d.]*)</(td|a)>").unwrap();

    let mut stat_averages: HashMap<u32, HashMap<String, f64>> = HashMap::new();

    for row in row_regex.captures_iter(table_regex.find(stats.as_str()).unwrap().as_str()) {
        let mut stat_values = col_regex.captures_iter(&row[0]);
        let year = stat_values.next().unwrap()[1].to_string().parse::<u32>().unwrap();
        let mut year_stats = HashMap::new();

        for (category, value) in zip(stat_categories.iter(), stat_values) {
            year_stats.insert(category.to_string(), value[1].to_string().parse::<f64>().unwrap_or(0.0));
        }
        stat_averages.insert(year, year_stats);
    }
    Ok(stat_averages)
}