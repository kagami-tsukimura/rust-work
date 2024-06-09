use crate::models;
use chrono::{Datelike, NaiveDate};
use std::collections::{BTreeMap, BTreeSet};

fn get_target_dates(data: &Vec<models::Item>) -> BTreeSet<NaiveDate> {
    let target_dates: BTreeSet<_> = data.iter().map(|item| item.get_first_day()).collect();
    target_dates
}

fn get_filtered_data(data: &Vec<models::Item>, filter_date: NaiveDate) -> Vec<&models::Item> {
    let filtered_data: Vec<&models::Item> = data
        .iter()
        .filter(|item| {
            (item.get_year() == filter_date.year()) && (item.get_month() == filter_date.month())
        })
        .collect();
    filtered_data
}
