use paste::paste;

macro_rules! declare_some_struct {
    ( $ name: ident) => {
        // Conventional declarative macro.
        struct $name {
            value: String,
        }

        impl $name {
            pub fn greet(&self) {
                println!("Hello from {}, {}!", stringify!($name), self.value);
            }
        }

        // Slightly less conventional declarative macro, making use of https://docs.rs/paste to
        // concatenate identifiers (extremely useful when doing more advanced code generation!).
        paste! {
            struct [<$name Bar>] {
                value: String,
            }

            impl [<$name Bar>] {
                pub fn greet(&self) {
                    println!("Hello from {}, {}!", stringify!([<$name Bar>]), self.value);
                }
            }
        }
    };
}

declare_some_struct!(Foo);

fn main() {
    // This works perfectly well with the IntelliJ Rust plugin.
    // Cmd+clicking on Foo brings me to the macro call that defines it.
    // It even autocompletes .greet() when using the new engine to expand declarative macros!
    let foo = Foo { value: "Bob".to_owned() };
    foo.greet();

    // This doesn't work so well on the other hand.
    let bar = FooBar { value: "Jane".to_owned() };
    bar.greet();
}
