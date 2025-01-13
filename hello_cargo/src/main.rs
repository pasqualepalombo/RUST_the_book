fn main() {
    // $ cargo new hello_cargo
    // se è da aggiungere ad un progetto già creato, spostare main.rs in src e poi $cargo init
    // per il build: // $ cargo build e crea di default in /target/debug/
    // oppure usare $ cargo run e cosi compila e esegue già
    // $ cargo check controlla il codice senza creare l'eseguibile
    // $ cargo build --release crea la build finale e la mette in /target/release/

    //se si prende un codice di qualcun altro cargo è comodo perchè
    // $ git clone example.org/something
    // cd something
    // cargo build
    
    println!("Hello, world!");
}
