pub fn run() {
    // parent module cannot call any private function, struct, variables
    // defined  in submodules
    // a module can call private objects in the same module or parent module.

    mod_a::a_public_function(String::from("run"));
    mod_a::mod_b::b_public_function(String::from("run"));
    mod_a::mod_b::call_priv_fns(String::from("run"));
    let struct_a = mod_a::StA::init(8);

    println!("{}", struct_a.public_var);

    mod_a::access_private_struct(struct_a);
}

mod mod_a {
    pub struct StA {
        private_var: u8,
        pub public_var: u8,
    }

    // impl cannot be declared as "pub"
    impl StA {
        pub fn init(value: u8) -> StA {
            StA {
                private_var: value,
                public_var: value,
            }
        }
    }

    fn a_private_function(caller: String) {
        println!("{} called a private function in mod_a", caller);
    }

    pub fn a_public_function(caller: String) {
        println!("{} called a public function in mod_a", caller);
    }

    pub fn access_private_struct(st_a: StA) {
        println!("priv: {}, pub: {}", st_a.private_var, st_a.public_var)
    }

    pub mod mod_b {

        fn b_private_function(caller: String) {
            println!("{} called a private function in mod_b", caller);
        }

        pub fn b_public_function(caller: String) {
            println!("{} called a public function in mod_b", caller);
        }

        // this function can call private functions
        // in the same module and in the parent modules
        pub fn call_priv_fns(caller: String) {

            super::a_private_function(caller.clone() + ">call_priv_fns");
            b_private_function(caller + ">call_priv_fns");
        }
    }
}
