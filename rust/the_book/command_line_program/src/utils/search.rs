/*
    Unlike build method on Config struct, there is at least one &str type in parameters and
    the &str in the return type can relate to one of them.
    So a lifetime specifier should be declared after function name, and then it should be
    used to show the relationship between parameters and return type.
*/
pub fn search_query_in_content<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];

    content
        .lines()
        .filter(|&l| l.contains(query))
        .for_each(|s| res.push(s));

    res
}

pub fn search_query_in_content_case_insensitive<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];
    let lower_cased_query = query.to_lowercase();

    content
        .lines()
        .filter(|&l| l.to_lowercase().contains(&lower_cased_query))
        .for_each(|s| res.push(s));

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let content = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";

        assert_eq!(
            vec!["Are you nobody, too?"],
            search_query_in_content(content, "too")
        );
    }

    #[test]
    fn case_insensitive_search() {
        let content = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";

        assert_eq!(
            vec!["Are you nobody, too?"],
            search_query_in_content_case_insensitive(content, "tOo")
        );
    }
}
