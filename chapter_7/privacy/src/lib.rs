mod outermost {
    pub fn middle_function() {}

    // private inner function of outermost
    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            // earlier versions of rust use ::outermost instead of super
            super::middle_function();
        }

        fn secret_function() {}
    }
}

// here's how to use the `use` keyword to bring deeply nested names into scope
mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn try_me() {
    outermost::middle_function();
    of::nested_modules();
}
