//! Squeeze consecutive, repeating characters together into a single one
//!
//! This crate provides an extension trait, [`SqueezeExt`], which when brought
//! into scope will provide two methods on [`String`]s, `String::squeeze` and
//! `String::squeeze_in_place`.
//!
//! The methods are implemented with a standalone function, `squeeze`, which may
//! be used imported separately and used instead when preferred.
//!
//! Note that multiple occurences of consecutive characters are also replaced.
//! For example, given the input "heeeleee", the output would be "hele".
//!
//! Additionally, variable cased consecutive characters will not be removed.
//! For example, given the input "hEeElEeE", the output will be unchanged.
//!
//! # Examples
//!
//! ```
//! use squeeze::{squeeze, SqueezeExt};
//!
//! // With standalone function
//! let mut my_string = String::from("heeelo");
//! squeeze(&mut my_string, 'e');
//!
//! assert_eq!(my_string, "helo");
//!
//! // With method
//! let mut squeezable = String::from("heeelo");
//! let squeezed = squeezable.squeeze('e');
//!
//! assert_eq!(squeezed, "helo");
//!
//! // With method, but operate in place
//! let mut squeezable_in_place = String::from("heeelo");
//! squeezable_in_place.squeeze_in_place('e');
//!
//! assert_eq!(squeezable_in_place, "helo");
//! ```

/// Squeeze consecutive, repeating characters together into a single one
/// Standalone version to be used when the trait extension is not preferred.
///
/// # Arguments
///
/// * `string` - the input string to squeeze
/// * `needle` - the `char` to squeeze
///
/// # Examples
///
/// ```
/// use squeeze::squeeze;
///
/// let mut my_string = String::from("heeelo");
/// squeeze(&mut my_string, 'e');
///
/// assert_eq!(my_string, "helo");
/// ```
pub fn squeeze(string: &mut String, needle: char) {
    let mut prev: Option<char> = None;
    string.retain(|curr| {
        if curr != needle || Some(curr) != prev {
            prev = Some(curr);
            true
        } else {
            false
        }
    })
}

/// Squeeze consecutive, repeating characters in a `String` into a single one
///
/// The extension trait defines two methods: `squeeze` and `squeeze_in_place`.
/// The first variant returns a new string, the second operates in place.
///
/// # Example
///
/// ```
/// use squeeze::SqueezeExt;
///
/// let mut my_string = String::from("heeelo");
/// let squeezed = my_string.squeeze('e');
///
/// assert_eq!(squeezed, "helo");
///
/// // or if you prefer to do it in place, it's provided as a convenience
/// // the advantage is that this does not require a call to `clone`.
/// let mut my_string_in_place = String::from("heeelo");
/// my_string_in_place.squeeze_in_place('e');
///
/// assert_eq!(my_string_in_place, "helo");
/// ```
pub trait SqueezeExt {
    /// Squeeze consecutive, repeating characters together into a single one
    ///
    /// # Arguments
    ///
    /// * `needle` - the `char` to squeeze
    ///
    /// # Examples
    ///
    /// ```
    /// use squeeze::SqueezeExt;
    ///
    /// let mut my_string = String::from("heeelo");
    /// my_string.squeeze_in_place('e');
    ///
    /// assert_eq!(my_string, "helo");
    /// ```
    fn squeeze_in_place(&mut self, needle: char);
    /// Squeeze consecutive, repeating characters together into a single one
    /// Returns a new, independent `String`
    ///
    /// # Arguments
    ///
    /// * `needle` - the `char` to squeeze
    ///
    /// # Examples
    ///
    /// ```
    /// use squeeze::SqueezeExt;
    ///
    /// let mut my_string = String::from("heeelo");
    /// let squeezed = my_string.squeeze('e');
    ///
    /// assert_eq!(squeezed, "helo");
    /// ```
    fn squeeze(&self, needle: char) -> String;
}

impl SqueezeExt for String {
    fn squeeze_in_place(&mut self, needle: char) {
        squeeze(self, needle);
    }

    fn squeeze(&self, needle: char) -> String {
        let mut this = self.clone();
        squeeze(&mut this, needle);
        this
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_remove_consecutive_characters_in_string() {
        let mut case_1 = String::from("goodbye");
        squeeze(&mut case_1, 'o');
        assert_eq!(case_1, "godbye");

        let mut case_2 = String::from("goodbyegoodbye");
        squeeze(&mut case_2, 'o');
        assert_eq!(case_2, "godbyegodbye");

        let mut case_3 = String::from("heeeeeeeelo world");
        squeeze(&mut case_3, 'e');
        assert_eq!(case_3, "helo world");

        let mut case_4 = String::from("heeeeeeeeleeeeeeeee");
        squeeze(&mut case_4, 'e');
        assert_eq!(case_4, "hele");

        let mut case_5 = String::from("hEEElEEE");
        squeeze(&mut case_5, 'E');
        assert_eq!(case_5, "hElE");
    }
}
