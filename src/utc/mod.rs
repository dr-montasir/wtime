use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// ### utc_now()
///
/// Retrieves the current UTC time as a `SystemTime`.
///
/// This function returns the current system time in Coordinated Universal Time (UTC)
/// as a `SystemTime` object. It can be used for logging events, measuring elapsed time,
/// or in any other situation where the current time in UTC is required.
///
/// ### Example
///
/// ```
/// use wtime::utc::utc_now;
///
/// let now = utc_now();
/// println!("Current UTC time: {:?}", now);
/// ```
///
/// ### Panics
///
/// This function does not panic, but it's important to note that when converting the
/// returned `SystemTime` to another time representation, such as a DateTime, it may fail
/// if the system time is unrepresentable.
///
/// ### Note
///
/// The return value is based on the system's current clock and may be affected by system
/// time changes, such as adjustments from network time protocols.
///
/// <small>End Fun Doc</small>
pub fn utc_now() -> SystemTime {
    SystemTime::now()
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
/// use wtime::utc::duration_since;
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

/// ### utc_ts_sec()
///
/// Retrieves the current UTC time as a UNIX timestamp.
///
/// This function calculates the number of seconds that have elapsed since the UNIX epoch
/// (January 1, 1970) and returns it as a `u64`. This can be useful for applications
/// that need to store or transmit timestamps in a standardized format.
///
/// ### Example
///
/// ```
/// use wtime::utc::utc_ts_sec;
///
/// let current_timestamp = utc_ts_sec();
/// println!("Current UTC Timestamp in Seconds: {}", current_timestamp);
/// ```
///
/// ### Returns
///
/// Returns the current time as a `u64` representing the number of seconds since the UNIX epoch.
///
/// ### Panics
///
/// This function does not panic, but if it were to be called in an environment where
/// system time is not available or has gone backward, it could yield incorrect results.
///
/// <small>End Fun Doc</small>
pub fn utc_ts_sec() -> u64 {
    duration_since().as_secs()
}

/// ### utc_ts_millis()
///
/// Retrieves the current UTC time as a UNIX timestamp.
///
/// This function calculates the number of milli-seconds that have elapsed since the UNIX epoch
/// (January 1, 1970) and returns it as a `u128`. This can be useful for applications
/// that need to store or transmit timestamps in a standardized format.
///
/// ### Example
///
/// ```
/// use wtime::utc::utc_ts_millis;
///
/// let current_timestamp = utc_ts_millis();
/// println!("Current UTC Timestamp in Milli-seconds: {}", current_timestamp);
/// ```
///
/// ### Returns
///
/// Returns the current time as a `u128` representing the number of seconds since the UNIX epoch.
///
/// ### Panics
///
/// This function does not panic, but if it were to be called in an environment where
/// system time is not available or has gone backward, it could yield incorrect results.
///
/// <small>End Fun Doc</small>
pub fn utc_ts_millis() -> u128 {
    duration_since().as_millis()
}

/// ### utc_ts_nanos()
///
/// Retrieves the current UTC time as a UNIX timestamp.
///
/// This function calculates the number of nano-seconds that have elapsed since the UNIX epoch
/// (January 1, 1970) and returns it as a `u128`. This can be useful for applications
/// that need to store or transmit timestamps in a standardized format.
///
/// ### Example
///
/// ```
/// use wtime::utc::utc_ts_nanos;
///
/// let current_timestamp = utc_ts_nanos();
/// println!("Current UTC Timestamp in Nano-seconds: {}", current_timestamp);
/// ```
///
/// ### Returns
///
/// Returns the current time as a `u128` representing the number of seconds since the UNIX epoch.
///
/// ### Panics
///
/// This function does not panic, but if it were to be called in an environment where
/// system time is not available or has gone backward, it could yield incorrect results.
///
/// <small>End Fun Doc</small>
pub fn utc_ts_nanos() -> u128 {
    duration_since().as_nanos()
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
/// use wtime::utc::{is_leap_year, get_year};
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

/// ### calculate_date(total_seconds: u64) -> (u64, u64, u64)
///
/// Calculates the date (year, month, day) from total seconds since the UNIX epoch.
///
/// This function takes a count of seconds since the UNIX epoch and computes the corresponding
/// calendar date.
///
/// ### Example
///
/// ```
/// use wtime::utc::calculate_date;
///
/// let seconds = 1_600_000_000; // Example total seconds since UNIX epoch
/// let (year, month, day) = calculate_date(seconds);
/// println!("Date: {}-{}-{}", year, month, day);
/// assert_eq!(calculate_date(1728933069), (2024, 10, 14));
/// ```
///
/// ### Returns
///
/// Returns a tuple containing the year, month, and day extracted from the total seconds.
///
/// <small>End Fun Doc</small>
pub fn calculate_date(total_seconds: u64) -> (u64, u64, u64) {
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

/// ### get_year() -> u64
///
/// Retrieves the current year.
///
/// This function calculates the current year based on the current UTC timestamp.
///
/// ### Example
///
/// ```
/// use wtime::utc::get_year;
///
/// let year = get_year();
/// println!("Current year: {}", year);
/// ```
///
/// ### Returns
///
/// Returns the current year as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_year() -> u64 {
    let (year, _, _) = calculate_date(utc_ts_sec());
    year
}

/// ### get_month() -> u64
///
/// Retrieves the current month.
///
/// This function calculates the current month based on the current UTC timestamp.
///
/// ### Example
///
/// ```
/// use wtime::utc::get_month;
///
/// let month = get_month();
/// println!("Current month: {}", month);
/// ```
///
/// ### Returns
///
/// Returns the current month as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_month() -> u64 {
    let (_, month, _) = calculate_date(utc_ts_sec());
    month
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
/// use wtime::utc::{get_month_name, get_month};
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

/// ### get_day() -> u64
///
/// Retrieves the current day of the month.
///
/// This function calculates the current day based on the current UTC timestamp.
///
/// ### Example
///
/// ```
/// use wtime::utc::get_day;
///
/// let day = get_day();
/// println!("Current day: {}", day);
/// ```
///
/// ### Returns
///
/// Returns the current day of the month as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_day() -> u64 {
    let (_, _, day) = calculate_date(utc_ts_sec());
    day
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
/// use wtime::utc::{get_day_name, get_day};
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

/// ### get_hour() -> u64
///
/// Retrieves the current hour of the day.
///
/// This function calculates the current hour based on the current UTC timestamp.
/// The hour is returned in 24-hour format (0-23).
///
/// ### Example
///
/// ```
/// use wtime::utc::get_hour;
///
/// let hour = get_hour();
/// println!("Current hour: {}", hour);
/// ```
///
/// ### Returns
///
/// Returns the current hour of the day as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_hour() -> u64 {
    let hour = ((utc_ts_sec() / 3600) % 24 + 24) % 24; // Handle wrap around for negative hours
    hour
}

/// ### get_minute() -> u64
///
/// Retrieves the current minute of the hour.
///
/// This function calculates the current minute based on the current UTC timestamp.
///
/// ### Example
///
/// ```
/// use wtime::utc::get_minute;
///
/// let minute = get_minute();
/// println!("Current minute: {}", minute);
/// ```
///
/// ### Returns
///
/// Returns the current minute of the hour as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_minute() -> u64 {
    let minute = (utc_ts_sec() / 60) % 60;
    minute
}

/// ### get_second() -> u64
///
/// Retrieves the current second of the minute.
///
/// This function calculates the current second based on the current UTC timestamp.
///
/// ### Example
///
/// ```
/// use wtime::utc::get_second;
///
/// let second = get_second();
/// println!("Current second: {}", second);
/// ```
///
/// ### Returns
///
/// Returns the current second of the minute as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_second() -> u64 {
    let second = utc_ts_sec() % 60;
    second
}

/// ### get_millis() -> u64
///
/// Retrieves the current milliseconds based on the elapsed time since the UNIX epoch.
///
/// This function calculates the current elapsed milliseconds since the UNIX epoch
/// and returns only the millisecond component (0-999).
///
/// ### Example
///
/// ```
/// use wtime::utc::get_millis;
///
/// let millis = get_millis();
/// println!("Current milliseconds: {}", millis);
/// ```
///
/// ### Returns
///
/// Returns the current milliseconds as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_millis() -> u64 {
    let millis = duration_since().as_millis() % 1000;
    millis as u64
}

/// ### get_nanos() -> u64
///
/// Retrieves the current nanoseconds based on the elapsed time since the UNIX epoch.
///
/// This function calculates the current elapsed nanoseconds since the UNIX epoch
/// and returns only the nanosecond component (0-999,999).
///
/// ### Example
///
/// ```
/// use wtime::utc::get_nanos;
///
/// let nanos = get_nanos();
/// println!("Current nanoseconds: {}", nanos);
/// ```
///
/// ### Returns
///
/// Returns the current nanoseconds as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_nanos() -> u64 {
    let nanos = duration_since().as_nanos() % 1_000_000;
    nanos as u64
}

/// ### format_utc_ts()
///
/// Retrieves the current UTC timestamp formatted as a string.
///
/// This function generates a formatted timestamp string in the structure
/// `year-month-day-hour-minute-second-millis-nanos`, adhering to the
/// following specifications for each component:
/// - Year: 4 digits (e.g., `2024`)
/// - Month: 2 digits, zero-padded (01-12)
/// - Day: 2 digits, zero-padded (01-31)
/// - Hour: 2 digits, zero-padded (00-23)
/// - Minute: 2 digits, zero-padded (00-59)
/// - Second: 2 digits, zero-padded (00-59)
/// - Milliseconds: 3 digits, zero-padded (000-999)
/// - Nanoseconds: 6 digits, zero-padded (000000-999999)
///
/// This function does not perform error handling or validation beyond the necessary
/// calculations for the timestamp components. It is assumed that the underlying
/// time retrieval functions are functioning correctly and return valid values.
///
/// ### Usage for Creating Unique Identifiers (IDs)
///
/// The formatted timestamp generated by this function can be used as part of unique
/// identifiers (IDs). You can create a timestamp-based ID that is likely to be unique
/// within your application's context, either by combining it with a unique suffix or
/// using it by itself.
///
/// Here's an example using a unique suffix:
/// ```rust
/// use wtime::utc::format_utc_ts;
///
/// let timestamp = format_utc_ts();
///     
/// // Simulate generating a unique ID with a suffix
/// let unique_id_with_suffix = format!("ID-{}-{}", format_utc_ts(), "random_suffix"); // Replace with actual random suffix function
/// println!("Generated Unique ID with Suffix: {}", unique_id_with_suffix);
/// ```
///
/// And here's an example using the timestamp by itself:
/// ```rust
/// use wtime::utc::format_utc_ts;
///
/// let timestamp = format_utc_ts();
///
/// // Simulate generating a unique ID without a suffix
/// let unique_id_without_suffix = format!("ID-{}", format_utc_ts());
/// println!("Generated Unique ID without Suffix: {}", unique_id_without_suffix);
/// ```
///
/// ### Usage in Blog Posts
///
/// You can also use the formatted timestamp for logging or identifying blog posts.
/// By generating a unique UTC timestamp string for each blog post, you ensure that
/// there is a clear and systematic way to track each post's creation time.
///
/// Example:
/// ```rust
/// use wtime::utc::format_utc_ts;
///
/// let blog_post_timestamp = format!("BlogPost Timestamp: {}", format_utc_ts());
/// println!("Generated Blog Post UTC Timestamp: {}", blog_post_timestamp);
/// ```
///
/// ### Example
///
/// ```rust
/// use wtime::utc::format_utc_ts;
///
/// let timestamp = format_utc_ts();
/// println!("Formatted UTC Timestamp: {}", timestamp);
/// ```
///
/// ### Returns
///
/// Returns a `String` representing the current UTC timestamp formatted in the specified
/// manner. The length of the returned string is guaranteed to be 30 characters if all
/// components are valid.
///
/// ### Note
///
/// - Ensure that any dependencies or underlying functionality that this function relies
///   upon (such as time retrieval functions) are functioning correctly to avoid unexpected
///   results. No panic conditions are raised in this function; any potential issues should
///   be managed by the developer as appropriate for their use case.
///
/// <small>End Fun Doc</small>
pub fn format_utc_ts() -> String {
    let year = get_year();
    let month = get_month();
    let day = get_day();
    let hour = get_hour();
    let minute = get_minute();
    let second = get_second();
    let millis = get_millis();
    let nanos = get_nanos();

    // Create the formatted string with updated formatting
    format!(
        "{:04}-{:02}-{:02}-{:02}-{:02}-{:02}-{:03}-{:06}",
        year, month, day, hour, minute, second, millis, nanos,
    )
}
