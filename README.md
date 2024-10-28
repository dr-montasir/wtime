<div style="text-align: center;">
  <a href="https://crates.io/crates/wtime"><img src="logo.svg" alt="LOGO" /></a>
</div>

<div style="text-align: center;">
  <a href="https://github.com/dr-montasir/wtime"><img src="https://img.shields.io/badge/github-dr%20montasir%20/%20wtime-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24" style="margin-top: 10px;" alt="github" /></a> <a href="https://crates.io/crates/wtime"><img src="https://img.shields.io/crates/v/wtime.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24" style="margin-top: 10px;" alt="crates.io"></a> <a href="https://docs.rs/wtime"><img src="https://img.shields.io/badge/docs.rs-wtime-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24" style="margin-top: 10px;" alt="docs.rs"></a> <a href="https://choosealicense.com/licenses/apache-2.0"><img src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="24" style="margin-top: 10px;" alt="license"></a> <a href="https://choosealicense.com/licenses/mit"><img src="https://img.shields.io/badge/license-mit-4a98f7.svg?style=for-the-badge&labelColor=555555" height="24" style="margin-top: 10px;" alt="license"></a>
</div>

# WTIME

**WTIME** provides a variety of functions for obtaining the current UTC and local times, as well as generating customizable timestamps to suit your needs.

---

## Table of Contents

- [Installation](#installation)
- [Features](#features)
- [Usage](#usage)
- [Documentation](#documentation)
- [License](#license)
- [Contributing](#contributing)
- [Author](#author)

## Installation

**Run the following Cargo command in your project directory:**

```terminal
cargo add wtime
```

**Or add `wtime` to your `Cargo.toml` file:**

```toml
[dependencies]
wtime = "MAJOR.MINOR.PATCH" # Replace with the latest version
```

## Features

1. ### Current Time Retrieval

   - **UTC and Local Time**: WTIME offers seamless access to both current UTC and local times, making it easy to work with time zones and coordinate time-related functionalities.

2. ### Customizable Timestamps

   - **Flexible Formatting**: Generate timestamps in various formats to meet your specific requirements, whether for logging, database entries, or user interfaces.
   - **Configuration Options**: Tailor timestamp outputs to include or exclude specific components (e.g., date, time, time zone) based on your project needs.

3. ### Time Zone Support

   - **Multiple Time Zones**: Easily convert between different time zones, allowing for applications that require global time management.
   - **Daylight Saving Time Adjustments**: Automatic adjustments for daylight saving time ensure that your applications reflect accurate local time.

4. ### Easy Integration

   - **User-Friendly API**: The WTIME API is designed with simplicity in mind, allowing developers to implement time-related functions with minimal effort.
   - **Comprehensive Documentation**: Well-documented functions and examples help users quickly understand how to leverage the crate effectively.

5. ### High Performance

   - **Efficiency**: WTIME is optimized for performance, ensuring that time retrieval and manipulation operations are fast and resource-friendly.
   - **Lightweight**: The crate is designed to be lightweight, minimizing overhead while providing extensive functionality.

## Usage

```rust
use wtime::utc::{calculate_date, get_day, get_day_name};

fn main() {
    let day = get_day();
    println!("Current day: {}", day);

    let day_name = get_day_name(1_670_000_000);
    println!("Day name: {}", day_name); // Day name: Friday
    println!("Day name: {}", get_day_name(day));

    let (year, month, day) = calculate_date(1728933069);
    println!("Date: {}-{}-{}", year, month, day); // Date: 2024-10-14
}
```

## Documentation

### [UTC](https://docs.rs/wtime/latest/wtime/utc/index.html)

|                                    Function                                     |                                    Function                                     |                                    Function                                     |
| :-----------------------------------------------------------------------------: | :-----------------------------------------------------------------------------: | :-----------------------------------------------------------------------------: |
| [calculate_date](https://docs.rs/wtime/latest/wtime/utc/fn.calculate_date.html) | [duration_since](https://docs.rs/wtime/latest/wtime/utc/fn.duration_since.html) |        [get_day](https://docs.rs/wtime/latest/wtime/utc/fn.get_day.html)        |
|   [get_day_name](https://docs.rs/wtime/latest/wtime/utc/fn.get_day_name.html)   |       [get_hour](https://docs.rs/wtime/latest/wtime/utc/fn.get_hour.html)       |     [get_millis](https://docs.rs/wtime/latest/wtime/utc/fn.get_millis.html)     |
|     [get_minute](https://docs.rs/wtime/latest/wtime/utc/fn.get_minute.html)     |      [get_month](https://docs.rs/wtime/latest/wtime/utc/fn.get_month.html)      | [get_month_name](https://docs.rs/wtime/latest/wtime/utc/fn.get_month_name.html) |
|      [get_nanos](https://docs.rs/wtime/latest/wtime/utc/fn.get_nanos.html)      |     [get_second](https://docs.rs/wtime/latest/wtime/utc/fn.get_second.html)     |       [get_year](https://docs.rs/wtime/latest/wtime/utc/fn.get_year.html)       |
|   [is_leap_year](https://docs.rs/wtime/latest/wtime/utc/fn.is_leap_year.html)   |        [utc_now](https://docs.rs/wtime/latest/wtime/utc/fn.utc_now.html)        |  [utc_ts_millis](https://docs.rs/wtime/latest/wtime/utc/fn.utc_ts_millis.html)  |
|   [utc_ts_nanos](https://docs.rs/wtime/latest/wtime/utc/fn.utc_ts_nanos.html)   |     [utc_ts_sec](https://docs.rs/wtime/latest/wtime/utc/fn.utc_ts_sec.html)     |                                        -                                        |

## License

This project is licensed under either of the following licenses:

- MIT License
- Apache License, Version 2.0

You may choose either license for your purposes.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any feature requests or bug reports.

## Author

[Dr. Montasir Mirghani](https://github.com/dr-montasir)
