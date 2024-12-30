// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn char_counter<T:  AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using `as_mut()`.
// TODO: Add the appropriate trait bound.
fn num_sq<T:  AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
     let num = arg.as_mut();
     *num = *num * *num;
}

fn double_chars<T: AsMut<String>>(arg: &mut T) {
    // TODO: Implement this function
    // Hint: Get mutable reference using as_mut()
    // Hint: Consider collecting chars into a new string
    let mut string = String::new();
    let arg = arg.as_mut();
    for c in arg.chars() {
        string.push(c);
        string.push(c);
    }
    *arg = string;
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
    #[test]

    fn test_double_chars() {
        let mut s = Box::new(String::from("abc"));
        double_chars(&mut s);
        assert_eq!(*s, "aabbcc");
    }

    #[test]
    fn test_empty_string() {
        let mut s = Box::new(String::new());
        double_chars(&mut s);
        assert_eq!(*s, "");
    }
}
