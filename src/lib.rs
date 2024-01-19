// enums

#[allow(non_camel_case_types)]
pub enum ValVerCase {
    kebab__case,
    Title__Kebab__Case,
    SCREAMING__KEBAB__CASE,
    snake_case,
    SCREAMING_SNAKE_CASE,
    camelCase,
    PascalCase,
}

impl ValVerCase {
    /// Returns a Capitalised version of the given `target_string`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use valver::ValVerCase;
    /// assert_eq!("Hello", ValVerCase::capitalise_first("hello"));
    /// ```
    ///
    /// ## Credits
    ///
    /// Thanks to Nick Groenen (https://nick.groenen.me/) for their solution:
    ///
    /// "Capitalize a string in Rust"
    /// (https://nick.groenen.me/notes/capitalize-a-string-in-rust/) -
    /// © 2024 Nick Groenen, Content licensed CC BY-NC 4.0
    /// (https://creativecommons.org/licenses/by-nc/4.0/).
    pub fn capitalise_first(target_string: &str) -> String {
        let mut target_string_characters = target_string.chars();

        match target_string_characters.next() {
            None => String::new(),
            Some(first_character) => {
                first_character.to_uppercase().collect::<String>()
                    + target_string_characters.as_str()
            }
        }
    }

    /// Returns a Capitalised version of every string in the given `vector`
    /// from the index of `index` on.
    ///
    /// ## Examples
    ///
    /// ```
    /// use valver::ValVerCase;
    ///
    /// let test_vector: Vec<&str> = "hello world".split(" ").collect();
    ///
    /// let capitalised_vector: Vec<String> = ValVerCase
    ///     ::capitalise_first_for_each_in_vector(0, &test_vector);
    ///
    /// assert_eq!(vec!["Hello", "World"], capitalised_vector);
    /// ```
    pub fn capitalise_first_for_each_in_vector(index: usize, vector: &Vec<&str>) -> Vec<String> {
        if index >= vector.len() {
            return vector.iter().map(|s| String::from(s.to_owned())).collect();
        }
        let mut new_vector: Vec<&str> = vector.clone();
        let new_string: &str = &ValVerCase::capitalise_first(new_vector[index]);
        new_vector[index] = new_string;
        ValVerCase::capitalise_first_for_each_in_vector(index + 1, &new_vector)
    }
}

// structs /|_/^\_|\ <- TIE Interceptor (from above)

// functions d|o|o|o|o| <- finty gauntl fingys

/// Returns a case-enforced version of the given `string`.
///
/// ## Example
///
/// ```
/// use valver::{ValVerCase, enforce_case};
/// assert_eq!("hello-world", enforce_case(ValVerCase::kebab__case, "Hello World"));
/// assert_eq!("Hello-World", enforce_case(ValVerCase::Title__Kebab__Case, "Hello World"));
/// assert_eq!("HELLO-WORLD", enforce_case(ValVerCase::SCREAMING__KEBAB__CASE, "Hello World"));
/// assert_eq!("hello_world", enforce_case(ValVerCase::snake_case, "Hello World"));
/// assert_eq!("HELLO_WORLD", enforce_case(ValVerCase::SCREAMING_SNAKE_CASE, "Hello World"));
/// assert_eq!("helloWorld", enforce_case(ValVerCase::camelCase, "Hello World"));
/// assert_eq!("HelloWorld", enforce_case(ValVerCase::PascalCase, "Hello World"));
/// ```
pub fn enforce_case(case: ValVerCase, string: &str) -> String {
    match case {
        ValVerCase::kebab__case => {
            // EXAMPLE ("Hello World")
            // 1. Replace ' ' with '-': "Hello-World"
            // 2. Set lowercase: "hello-world"
            string.replace(" ", "-").to_lowercase()
        }
        ValVerCase::Title__Kebab__Case => {
            // EXAMPLE ("Hello world")
            // 1. Split at ' ': ["Hello", "world"]
            // 2. Capitalise each: ["Hello", "World"]
            // 3. Join with '-': "Hello-World"
            let split_string: Vec<&str> = string.split(' ').collect();
            let capitalised_split_string: Vec<String> =
                ValVerCase::capitalise_first_for_each_in_vector(0, &split_string);
            capitalised_split_string.join("-")
        }
        ValVerCase::SCREAMING__KEBAB__CASE => {
            // EXAMPLE ("Hello world")
            // 1. Replace ' ' with '-': "Hello-world"
            // 2. Set uppercase: "HELLO-WORLD"
            string.replace(" ", "-").to_uppercase()
        }
        ValVerCase::snake_case => {
            // EXAMPLE ("Hello World")
            // 1. Replace ' ' with '_': "Hello_World"
            // 2. Set lowercase: "hello_world"
            string.replace(" ", "_").to_lowercase()
        }
        ValVerCase::SCREAMING_SNAKE_CASE => {
            // EXAMPLE ("Hello world")
            // 1. Replace ' ' with '_': "Hello_world"
            // 2. Set uppercase: "HELLO_WORLD"
            string.replace(" ", "_").to_uppercase()
        }
        ValVerCase::camelCase => {
            // EXAMPLE ("Hello world")
            // 1. Split at ' ': ["Hello", "world"]
            // 2. Capitalise each, minus first: ["Hello", "World"]
            // 3. Set first lowercase: ["hello", "World"]
            // 4. Join with '': "helloWorld"
            let split_string: Vec<&str> = string.split(' ').collect();
            let mut capitalised_split_string: Vec<String> =
                ValVerCase::capitalise_first_for_each_in_vector(1, &split_string);
            let lower_first_word: &String = &capitalised_split_string[0].to_lowercase();
            capitalised_split_string[0] = lower_first_word.to_owned();
            capitalised_split_string.join("")
        }
        ValVerCase::PascalCase => {
            // EXAMPLE ("hello world")
            // 1. Split at ' ': ["hello", "world"]
            // 2. Capitalise each: ["Hello", "World"]
            // 3. Join with '': "HelloWorld"
            let split_string: Vec<&str> = string.split(' ').collect();
            let capitalised_split_string =
                ValVerCase::capitalise_first_for_each_in_vector(0, &split_string);
            capitalised_split_string.join("")
        }
    }
}

// tests _-^-_-^-_ <- fattcatt

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalise_first() {
        assert_eq!("Hello", ValVerCase::capitalise_first("hello"));
    }

    #[test]
    fn test_capitalise_first_for_each_in_vector() {
        let test_vector: Vec<&str> = "hello world".split(" ").collect();
        let capitalised_vector: Vec<String> =
            ValVerCase::capitalise_first_for_each_in_vector(0, &test_vector);
        assert_eq!(vec!["Hello", "World"], capitalised_vector);
    }

    #[test]
    fn test_enforce_case() {
        assert_eq!(
            "hello-world",
            enforce_case(ValVerCase::kebab__case, "Hello World")
        );
        assert_eq!(
            "Hello-World",
            enforce_case(ValVerCase::Title__Kebab__Case, "Hello World")
        );
        assert_eq!(
            "HELLO-WORLD",
            enforce_case(ValVerCase::SCREAMING__KEBAB__CASE, "Hello World")
        );
        assert_eq!(
            "hello_world",
            enforce_case(ValVerCase::snake_case, "Hello World")
        );
        assert_eq!(
            "HELLO_WORLD",
            enforce_case(ValVerCase::SCREAMING_SNAKE_CASE, "Hello World")
        );
        assert_eq!(
            "helloWorld",
            enforce_case(ValVerCase::camelCase, "Hello World")
        );
        assert_eq!(
            "HelloWorld",
            enforce_case(ValVerCase::PascalCase, "Hello World")
        );
    }
}
