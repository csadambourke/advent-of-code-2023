use anyhow::Result;
use std::str::FromStr;
use std::fs;

pub fn read_and_split_file<T>(file_path: &str) -> Result<Vec<T>>
    where T: FromStr, {
        Ok(fs::read_to_string(file_path)?.split("\n")
           .filter_map(|line| line.parse::<T>().ok())
           .collect())
}
