use chimera::repl::start_repl;
fn main() {
    println!("Hello {}! This is the Chimera programming language", whoami::username());
    println!("Feel free to type in commands");

    start_repl();

}
