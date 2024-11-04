use super::calc::{calc_date, duration_since};
use std::time::SystemTime;

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
    let (year, _, _) = calc_date(utc_ts_sec());
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
    let (_, month, _) = calc_date(utc_ts_sec());
    month
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
    let (_, _, day) = calc_date(utc_ts_sec());
    day
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
