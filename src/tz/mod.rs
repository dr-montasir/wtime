use chrono::Local;

/// ### tz_string()
///
/// Retrieves the current local timezone offset as a string.
///
/// This function obtains the local timezone offset from the `chrono` crate and formats it
/// as a string. It provides a human-readable representation of the timezone offset, which
/// can be useful for logging or displaying the user's local time settings.
///
/// ### Example
///
/// ```
/// use wtime::tz::tz_string;
///
/// let offset_string = tz_string();
/// println!("Current timezone offset: {}", offset_string);
/// ```
///
/// ### Returns
///
/// Returns the current timezone offset as a `String`.
///
/// <small>End Fun Doc</small>
pub fn tz_string() -> String {
    let offset_string = Local::now().offset().to_string();
    format!("{}", offset_string)
}

/// ### tz_number()
///
/// Retrieves the local timezone offset as an `i64` in hours.
///
/// This function calculates the local timezone offset and returns it as an integer value
/// representing the number of hours offset from UTC. This can be useful for calculating
/// time differences or adjusting timestamps to local time.
///
/// ### Example
///
/// ```
/// use wtime::tz::tz_number;
///
/// let offset_number = tz_number();
/// println!("Current timezone offset in hours: {}", offset_number);
/// ```
///
/// ### Returns
///
/// Returns the local timezone offset as an `i64` representing the total number of hours from UTC.
/// If there is an error in parsing, it defaults to returning `0`.
///
/// <small>End Fun Doc</small>
pub fn tz_number() -> i64 {
    // Get the local timezone offset as a string
    let offset_str = Local::now().offset().to_string();
    // Split the string into hours and minutes
    let parts: Vec<&str> = offset_str.split(':').collect();
    if parts.len() == 2 {
        // Get the sign for the offset
        let sign = if parts[0].starts_with('+') { 1 } else { -1 };
        // Parse the hours (skip the sign)
        if let Ok(hours) = parts[0][1..].parse::<i64>() {
            return sign * hours; // Return the total offset in hours
        }
    }
    0 // Default return value if something goes wrong
}
