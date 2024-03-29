// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.



trait AppendBar {
    fn append_bar(&mut self);
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(&mut self){
        self.push_str("Bar");
    }
}

fn main() {
    let mut s = String::from("Foo");
    s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        let mut s = String::from("Foo");
        s.append_bar();
        assert_eq!(s, String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        let mut s = String::from("");
        s.append_bar();
        s.append_bar();
        assert_eq!(
            s,
            String::from("BarBar")
        );
    }
}
