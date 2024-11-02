use super::{
    tz::tz_number,
    utc::{utc_ts_millis, utc_ts_nanos, utc_ts_sec},
};

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
