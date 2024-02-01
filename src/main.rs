mod visibility_mod;

fn function() {
    println!("Hello, this is a playground for Rust!");
}

fn main() {
    // Calling a function
    function();

    // Cannot call a private function
    // visibility_mod::my_mod::private_function();

    // Calling a public function in a different mod
    visibility_mod::my_mod::function();

    // Calling a public function which calls another function internally
    visibility_mod::my_mod::indirect_access();

    // Calling a nested mod
    visibility_mod::my_mod::nested::function();
}