use super::utc::utc_now;
use std::time::{Duration, UNIX_EPOCH};

/// ### calc_date(total_seconds: u64) -> (u64, u64, u64)
///
/// Calculates the date (year, month, day) from total seconds since the UNIX epoch.
///
/// This function takes a count of seconds since the UNIX epoch and computes the corresponding
/// calendar date.
///
/// ### Example
///
/// ```
/// use wtime::calc::calc_date;
///
/// let seconds = 1_600_000_000; // Example total seconds since UNIX epoch
/// let (year, month, day) = calc_date(seconds);
/// println!("Date: {}-{}-{}", year, month, day);
/// assert_eq!(calc_date(1728933069), (2024, 10, 14));
/// ```
///
/// ### Returns
///
/// Returns a tuple containing the year, month, and day extracted from the total seconds.
///
/// <small>End Fun Doc</small>
pub fn calc_date(total_seconds: u64) -> (u64, u64, u64) {
    let mut seconds_remaining = total_seconds;

    // Calculate current year
    let mut year = 1970;
    while seconds_remaining >= 31_536_000 + if is_leap_year(year) { 86_400 } else { 0 } {
        seconds_remaining -= 31_536_000 + if is_leap_year(year) { 86_400 } else { 0 };
        year += 1;
    }

    // Calculate current month and day
    let month_days = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 0;
    while month < 12 {
        let days_in_month = month_days[month];
        let seconds_in_month = days_in_month * 86_400; // 86,400 seconds in a day

        if seconds_remaining < seconds_in_month {
            break;
        }

        seconds_remaining -= seconds_in_month;
        month += 1;
    }

    let day = seconds_remaining / 86_400 + 1; // +1 to convert to 1-based day
    (year, (month + 1) as u64, day) // +1 for 1-based month
}

/// ### calc_week(date: (u64, u64, u64)) -> u64
///
/// Calculates the week number in the year based on a provided date.
///
/// The function takes a date in the format `(year, month, day)` and returns the week number
/// according to the ISO 8601 standard (where the first week of the year is the week
/// containing the first Thursday).
///
/// ### Example
///
/// ```rust
/// use wtime::calc::calc_week;
///
/// let week_number = calc_week((2024, 10, 14));
/// println!("Week number: {}", week_number);
/// ```
///
/// ### Returns
///
/// Returns the week number as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn calc_week(date: (u64, u64, u64)) -> u64 {
    let (year, month, day) = date;

    // Calculate the day of the year
    let month_days = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] // Leap year
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] // Non-leap year
    };

    // Sum the days in the preceding months and add the current day
    let day_of_year = month_days.iter().take((month - 1) as usize).sum::<u64>() + day;

    // Calculate the weekday of January 1 of the given year
    // 0: Saturday, 1: Sunday, ..., 6: Friday (we want 0 for ISO week calculation)
    let weekday_of_first_jan =
        (365 * (year - 1970) + (year - 1970) / 4 - (year - 1970) / 100 + (year - 1970) / 400 + 1)
            % 7;

    // Adjust for the ISO week number
    let first_thursday_in_year = if weekday_of_first_jan <= 3 {
        (1 + (3 - weekday_of_first_jan)) as u64
    } else {
        (8 - weekday_of_first_jan) as u64
    };

    // Calculate the week number
    let week_number = ((day_of_year - first_thursday_in_year + 10) / 7) as u64; // +10 to adjust full weeks starting with Thursday

    week_number
}

/// ### duration_since()
///
/// Returns the duration from the UNIX epoch to the current time.
///
/// This function calculates the amount of time that has elapsed since the UNIX epoch
/// (January 1, 1970) and returns it as a `Duration`. This can be useful for calculations
/// involving specific time intervals.
///
/// ### Example
///
/// ```
/// use wtime::calc::duration_since;
///
/// let duration = duration_since();
/// println!("Seconds since UNIX epoch: {:?}", duration.as_secs());
/// println!("Milli-seconds since UNIX epoch: {:?}", duration_since().as_millis());
/// ```
///
/// ### Panics
///
/// This function will panic if the current system time is before the UNIX epoch.
///
/// <small>End Fun Doc</small>
pub fn duration_since() -> Duration {
    utc_now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

/// ### get_day_name(total_seconds: u64) -> &'static str
///
/// Returns the name of the day of the week corresponding to the total seconds since the UNIX epoch.
///
/// This function calculates the day name based on the total seconds since the UNIX epoch.
///
/// ### Example
///
/// ```
/// use wtime::calc::get_day_name;
/// use wtime::utc::get_day;
///
/// let day = get_day();
/// println!("Current day: {}", day);
///
/// let day_name = get_day_name(1_670_000_000);
/// println!("Day name: {}", day_name); // Day name: Friday
/// println!("Day name: {}", get_day_name(day));
/// ```
///
/// ### Returns
///
/// Returns the name of the day as a static string reference.
///
/// <small>End Fun Doc</small>
pub fn get_day_name(total_seconds: u64) -> &'static str {
    const DAYS: [&str; 7] = [
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
    ]; // Starting from 1970-01-01
    let days_since_epoch = (total_seconds / 86_400) % 7; // Total days since UNIX epoch modulo 7
    DAYS[days_since_epoch as usize]
}

/// ### get_month_name(month: u64) -> &'static str
///
/// Returns the name of the month corresponding to the provided month number.
///
/// This function takes a month number (from 1 to 12) and returns the corresponding month name.
///
/// ### Example
///
/// ```
/// use wtime::calc::get_month_name;
/// use wtime::utc::get_month;
///
/// let month = get_month();
/// println!("Current month: {}", month);
///
/// let month_name = get_month_name(4);
/// println!("Month name: {}", month_name); // "April"
/// println!("Month name: {}", get_month_name(month));
/// ```
///
/// ### Panics
///
/// This function will panic if provided with an invalid month number (not between 1 and 12).
///
/// <small>End Fun Doc</small>
pub fn get_month_name(month: u64) -> &'static str {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    MONTHS[(month - 1) as usize] // -1 to convert from 1-indexed to 0-indexed
}

/// ### is_leap_year(year: u64) -> bool
///
/// Determines if a given year is a leap year.
///
/// This function checks if the provided year is a leap year according to the rules:
/// a year is a leap year if it is divisible by 4 but not divisible by 100, except for years
/// that are divisible by 400.
///
/// ### Example
///
/// ```
/// use wtime::calc::is_leap_year;
/// use wtime::utc::get_year;
///
/// let year = 2024;
/// println!("Is {} a leap year? {}", year, is_leap_year(year)); // true
/// println!("Is {} a leap year? {}", year, is_leap_year(get_year()));
/// ```
///
/// ### Returns
///
/// Returns `true` if the year is a leap year, otherwise returns `false`.
///
/// <small>End Fun Doc</small>
pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
