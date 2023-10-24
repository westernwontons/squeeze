pub trait Squeeze {
    fn squeeze_in_place(&mut self, needle: char);
}

impl Squeeze for String {
    fn squeeze_in_place(&mut self, needle: char) {
        let mut prev: Option<char> = None;
        self.retain(|curr| {
            if curr != needle || Some(curr) != prev {
                prev = Some(curr);
                true
            } else {
                false
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_remove_consecutive_characters_in_string() {
        let mut case_1 = String::from("goodbye");
        case_1.squeeze_in_place('o');
        assert_eq!(case_1, "godbye");

        let mut case_2 = String::from("goodbyegoodbye");
        case_2.squeeze_in_place('o');
        assert_eq!(case_2, "godbyegodbye");

        let mut case_3 = String::from("heeeeeeeelo world");
        case_3.squeeze_in_place('e');
        assert_eq!(case_3, "helo world");

        let mut case_4 = String::from("heeeeeeeeleeeeeeeee");
        case_4.squeeze_in_place('e');
        assert_eq!(case_4, "hele");
    }
}
