/**
 * 
 * Create a function which answers the question "Are you playing banjo?".
 * If your name starts with the letter "R" or lower case "r", you are playing banjo!
 * The function takes a name as its only argument, and returns one of the following strings:
 * name + " plays banjo" 
 * name + " does not play banjo"
 * Names given are always valid strings.
 */

fn are_you_playing_banjo(name: &str) -> String {
    match name {
        name if 'R'.eq_ignore_ascii_case(&(name.as_bytes()[0] as char)) => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("randy"), "randy plays banjo");

    }
}