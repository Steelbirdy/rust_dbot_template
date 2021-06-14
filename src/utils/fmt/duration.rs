use std::time::Duration;

/// Format a duration (in days, hours, minutes, and seconds)
/// nicely for discord.
pub fn format_duration(duration: Duration, compact: bool) -> String {
    let div_mod = |a: u64, b: u64| (a / b, a % b);

    let (minutes, seconds) = div_mod(duration.as_secs(), 60);
    let (hours, minutes) = div_mod(minutes, 60);
    let (days, hours) = div_mod(hours, 24);

    if compact {
        if days > 0 {
            format!("{:02}:{:02}:{:02}:{:02}", days, hours, minutes, seconds)
        } else if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    } else {
        let mut parts = vec![];

        let format_part = |x: u64, s: &str| {
            let suffix = if x == 1 { "" } else { "s" };
            format!("{} {}{}", x, s, suffix)
        };

        if days > 0 {
            parts.push(format_part(days, "day"));
        }
        if hours > 0 {
            parts.push(format_part(hours, "hour"));
        }
        if minutes > 0 {
            parts.push(format_part(minutes, "minute"));
        }
        if seconds > 0 {
            parts.push(format_part(seconds, "second"));
        }

        if parts.is_empty() {
            String::from("0 seconds")
        } else {
            parts.join(", ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0                       => "0 seconds".to_string()              ; "zero seconds")]
    #[test_case(20                      => "20 seconds".to_string()             ; "zero minutes")]
    #[test_case(18 * 60 + 32            => "18 minutes, 32 seconds".to_string() ; "zero hours")]
    #[test_case(5 * 60 * 60 + 7         => "5 hours, 7 seconds".to_string()     ; "zero days")]
    #[test_case((11 * 24 + 9) * 60 * 60 => "11 days, 9 hours".to_string()       ; "nonzero days")]
    fn fmt_duration(secs: u64) -> String {
        let duration = Duration::from_secs(secs);
        format_duration(duration, false)
    }

    #[test_case(20                      => "00:20".to_string()             ; "zero minutes")]
    #[test_case(18 * 60 + 32            => "18:32".to_string()             ; "zero hours")]
    #[test_case(5 * 60 * 60 + 7         => "05:00:07".to_string()          ; "zero days")]
    #[test_case((11 * 24 + 9) * 60 * 60 => "11:09:00:00".to_string()       ; "nonzero days")]
    fn fmt_duration_compact(secs: u64) -> String {
        let duration = Duration::from_secs(secs);
        format_duration(duration, true)
    }
}
