pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'b>(query: &str, contents: &'b str) -> Vec<&'b str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Duct Tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Trust me";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, contents)
        );
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::{search, search_case_insensitive};

    #[test]
    fn no_matches_and_empty_contents_return_no_lines() {
        for search_fn in [search, search_case_insensitive] {
            assert!(search_fn("missing", "first\nsecond").is_empty());
            assert!(search_fn("query", "").is_empty());
        }
    }

    #[test]
    fn matches_preserve_order_duplicates_and_original_text() {
        let contents = "Rust Rust\r\nno match\r\nRust Rust\r\nRUST";
        assert_eq!(search("Rust", contents), vec!["Rust Rust", "Rust Rust"]);
        assert_eq!(
            search_case_insensitive("rust", contents),
            vec!["Rust Rust", "Rust Rust", "RUST"]
        );
    }

    #[test]
    fn empty_query_matches_every_line() {
        for search_fn in [search, search_case_insensitive] {
            assert_eq!(search_fn("", "first\n\nlast\n"), vec!["first", "", "last"]);
        }
    }

    #[test]
    fn case_insensitive_search_handles_unicode_lowercasing() {
        assert_eq!(
            search_case_insensitive("ÉCOLE", "école\nÉcole\necole"),
            vec!["école", "École"]
        );
    }
}
