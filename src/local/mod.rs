use super::{
    calc::{calc_date, duration_since, get_millis, get_minute, get_nanos, get_second},
    tz::tz_number,
    utc::{utc_ts_millis, utc_ts_nanos, utc_ts_sec},
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// ### local_now()
///
/// Retrieves the current local time based on a variable timezone offset.
///
/// This function returns the current local system time as a `SystemTime` object,
/// adjusted from the current UTC time by a timezone offset obtained from a function.
/// This allows for dynamic adjustments based on different timezone settings, which may
/// be particularly useful for applications that need to handle multiple timezones,
/// including daylight saving changes.
///
/// ### Example
///
/// ```
/// use wtime::local::local_now;
///
/// let now = local_now();
/// println!("Current local time: {:?}", now);
/// ```
///
/// ### Panics
///
/// This function does not panic under normal circumstances; however, it may panic
/// if there is an issue with the system clock, such as if the current time is before
/// the Unix epoch (January 1, 1970). It may also panic if the conversion to a
/// `Duration` fails due to time going backwards or an invalid timezone offset.
///
/// ### Note
///
/// The timezone offset is obtained dynamically via the `tz_number()` function,
/// allowing for more flexibility than a hard-coded offset. Ensure that `tz_number()`
/// accurately reflects the intended timezone offset in hours. Calculating the offset
/// in seconds allows for precise adjustment of the UTC time to the local time.
///
/// <small>End Fun Doc</small>
pub fn local_now() -> SystemTime {
    // Timezone offset
    let timezone_offset_hours: i64 = tz_number();

    // Calculate the offset in seconds
    let offset_in_seconds = timezone_offset_hours * 3600;

    // Get the duration since the Unix epoch for the UTC time
    let duration_since_epoch = duration_since();

    // Calculate the new duration for the local time
    let local_duration = duration_since_epoch + Duration::from_secs(offset_in_seconds as u64);

    // Convert back to SystemTime
    UNIX_EPOCH + local_duration
}

/// ### local_ts_sec()
///
/// Retrieves the current local time as a UNIX timestamp in seconds.
///
/// This function calculates the local time in seconds since the UNIX epoch by
/// adding the local timezone offset (in hours) to the current UTC timestamp.
/// This is useful for obtaining a UNIX timestamp that reflects the local time
/// settings.
///
/// ### Example
///
/// ```
/// use wtime::local::local_ts_sec;
///
/// let local_timestamp = local_ts_sec();
/// println!("Current Local Timestamp in Seconds: {}", local_timestamp);
/// ```
///
/// ### Returns
///
/// Returns the current local time as a `u64` representing the number of seconds
/// since the UNIX epoch.
///
/// <small>End Fun Doc</small>
pub fn local_ts_sec() -> u64 {
    utc_ts_sec() + (tz_number() * 60 * 60) as u64
}

/// ### local_ts_millis()
///
/// Retrieves the current local time as a UNIX timestamp in milliseconds.
///
/// This function calculates the local time in milliseconds since the UNIX epoch
/// by adding the local timezone offset (in hours) to the current UTC timestamp.
/// This is useful for obtaining a timestamp that is precise to the millisecond for
/// applications that require high-resolution timing.
///
/// ### Example
///
/// ```
/// use wtime::local::local_ts_millis;
///
/// let local_timestamp_millis = local_ts_millis();
/// println!("Current Local Timestamp in Milliseconds: {}", local_timestamp_millis);
/// ```
///
/// ### Returns
///
/// Returns the current local time as a `u128` representing the number of milliseconds
/// since the UNIX epoch.
///
/// <small>End Fun Doc</small>
pub fn local_ts_millis() -> u128 {
    utc_ts_millis() + (tz_number() * 60 * 60 * 1_000) as u128
}

/// ### local_ts_nanos()
///
/// Retrieves the current local time as a UNIX timestamp in nanoseconds.
///
/// This function calculates the local time in nanoseconds since the UNIX epoch
/// by adding the local timezone offset (in hours) to the current UTC timestamp.
/// This is useful for applications that require extremely high-resolution timestamps.
///
/// ### Example
///
/// ```
/// use wtime::local::local_ts_nanos;
///
/// let local_timestamp_nanos = local_ts_nanos();
/// println!("Current Local Timestamp in Nanoseconds: {}", local_timestamp_nanos);
/// ```
///
/// ### Returns
///
/// Returns the current local time as a `u128` representing the number of nanoseconds
/// since the UNIX epoch.
///
/// <small>End Fun Doc</small>
pub fn local_ts_nanos() -> u128 {
    utc_ts_nanos() + (tz_number() * 60 * 60 * 1_000_000) as u128
}

/// ### get_local_year() -> u64
///
/// Retrieves the current year.
///
/// This function calculates the current year based on the current local timestamp.
///
/// ### Example
///
/// ```
/// use wtime::local::get_local_year;
///
/// let year = get_local_year();
/// println!("Current year: {}", year);
/// ```
///
/// ### Returns
///
/// Returns the current year as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_local_year() -> u64 {
    let (year, _, _) = calc_date(local_ts_sec());
    year
}

/// ### get_local_month() -> u64
///
/// Retrieves the current month.
///
/// This function calculates the current month based on the current local timestamp.
///
/// ### Example
///
/// ```
/// use wtime::local::get_local_month;
///
/// let month = get_local_month();
/// println!("Current month: {}", month);
/// ```
///
/// ### Returns
///
/// Returns the current month as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_local_month() -> u64 {
    let (_, month, _) = calc_date(local_ts_sec());
    month
}

/// ### get_local_day() -> u64
///
/// Retrieves the current day of the month.
///
/// This function calculates the current day based on the current local timestamp.
///
/// ### Example
///
/// ```
/// use wtime::local::get_local_day;
///
/// let day = get_local_day();
/// println!("Current day: {}", day);
/// ```
///
/// ### Returns
///
/// Returns the current day of the month as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_local_day() -> u64 {
    let (_, _, day) = calc_date(local_ts_sec());
    day
}

/// ### get_local_hour() -> u64
///
/// Retrieves the current hour of the day.
///
/// This function calculates the current hour based on the current local timestamp.
/// The hour is returned in 24-hour format (0-23).
///
/// ### Example
///
/// ```
/// use wtime::local::get_local_hour;
///
/// let hour = get_local_hour();
/// println!("Current hour: {}", hour);
/// ```
///
/// ### Returns
///
/// Returns the current hour of the day as a `u64`.
///
/// <small>End Fun Doc</small>
pub fn get_local_hour() -> u64 {
    let hour = ((local_ts_sec() / 3600) % 24 + 24) % 24; // Handle wrap around for negative hours
    hour
}

/// ### format_local_ts()
///
/// Retrieves the current local timestamp formatted as a string.
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
/// use wtime::local::format_local_ts;
///
/// let timestamp = format_local_ts();
///     
/// // Simulate generating a unique ID with a suffix
/// let unique_id_with_suffix = format!("ID-{}-{}", format_local_ts(), "random_suffix"); // Replace with actual random suffix function
/// println!("Generated Unique ID with Suffix: {}", unique_id_with_suffix);
/// ```
///
/// And here's an example using the timestamp by itself:
/// ```rust
/// use wtime::local::format_local_ts;
///
/// let timestamp = format_local_ts();
///
/// // Simulate generating a unique ID without a suffix
/// let unique_id_without_suffix = format!("ID-{}", format_local_ts());
/// println!("Generated Unique ID without Suffix: {}", unique_id_without_suffix);
/// ```
///
/// ### Usage in Blog Posts
///
/// You can also use the formatted timestamp for logging or identifying blog posts.
/// By generating a unique local timestamp string for each blog post, you ensure that
/// there is a clear and systematic way to track each post's creation time.
///
/// Example:
/// ```rust
/// use wtime::local::format_local_ts;
///
/// let blog_post_timestamp = format!("BlogPost Timestamp: {}", format_local_ts());
/// println!("Generated Blog Post Local Timestamp: {}", blog_post_timestamp);
/// ```
///
/// ### Example
///
/// ```rust
/// use wtime::local::format_local_ts;
///
/// let timestamp = format_local_ts();
/// println!("Formatted Local Timestamp: {}", timestamp);
/// ```
///
/// ### Returns
///
/// Returns a `String` representing the current local timestamp formatted in the specified
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
pub fn format_local_ts() -> String {
    let year = get_local_year();
    let month = get_local_month();
    let day = get_local_day();
    let hour = get_local_hour();
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
