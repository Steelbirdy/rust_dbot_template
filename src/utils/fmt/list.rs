use std::fmt::Display;

/// Formats a list nicely for discord.
/// `sep` is the string that will separate the items in the list.
/// For example, separating ['a', 'b', 'c'] by ', ' would result in "a, b, c".
///
/// `conjunction` is the string that will separate the last item from the rest of the list.
/// Using the same example above with the conjunction ', or ' would result is "a, b, or c"
pub fn format_list<T>(list: &[T], sep: &str, conjunction: Option<&str>) -> Option<String>
where
    T: Display,
{
    let conjunction = conjunction.unwrap_or(sep);
    let len = list.len();

    // NOTE: Once intersperse is stable, use that
    match len {
        0..=2 => list
            .iter()
            .map(ToString::to_string)
            .reduce(|a, b| format!("{}{}{}", a, sep, b)),
        _ => {
            let last = list.last().unwrap();
            let ret = list[..len - 1]
                .iter()
                .map(ToString::to_string)
                .reduce(|a, b| format!("{}{}{}", a, sep, b))
                .expect("Unreachable");

            Some(format!("{}{}{}", ret, conjunction, last))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Vec::<u8>::new(),         ":",  None          => None                              ; "empty list")]
    #[test_case(vec!["a"],                ":",  Some(";")     => Some("a".to_string())             ; "single-item list")]
    #[test_case(vec!["a", "b"],           ":",  Some(";")     => Some("a:b".to_string())           ; "short list no conjunction")]
    #[test_case(vec![192u8, 168, 1, 1],   ".",  None          => Some("192.168.1.1".to_string())   ; "non-string list")]
    #[test_case(vec!["a", "b", "c", "d"], ", ", Some(", or ") => Some("a, b, c, or d".to_string()) ; "long list conjunction")]
    fn fmt_list<T: Display>(list: Vec<T>, sep: &str, conjunction: Option<&str>) -> Option<String> {
        format_list(&list, sep, conjunction)
    }
}
