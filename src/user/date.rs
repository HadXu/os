use crate::{kernel, print};
use time::{Duration, OffsetDateTime, UtcOffset};
pub fn main() {
    let seconds = kernel::clock::realtime();
    let nanoseconds = libm::floor(1e9 * (seconds - libm::floor(seconds))) as i64;
    let date = OffsetDateTime::from_unix_timestamp(seconds as i64).to_offset(offset())
        + Duration::nanoseconds(nanoseconds);
    
    let format = "%FT%H:%M:%S";

    match time::util::validate_format_string(format) {
        Ok(()) => {
            print!("{}\n", date.format(format));
        }
        Err(e) => {
            print!("Error: {}\n", e);
        }
    }
}

fn offset() -> UtcOffset {
    UtcOffset::seconds(8 * 3600)
}