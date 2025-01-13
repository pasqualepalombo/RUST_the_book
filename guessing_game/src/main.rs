// Standard (std) library per input e output chiamata io
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // variabile immutabile che prende un numero casuale legato ad un thread locale dell'esecuzione,
    // poi viene chiamato gen_range che è un metodo usabile tramite start..=end per definire il range.
    // Per vedere la documentazione inerente al programma usare $ cargo doc --open
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please, input your guess. ");

        // let è per dichiarare le variabili, immutabili una volta dato un valore
        // mut le rende mutabili
        // guess è di tipo stringa e creiamo con la funzione associata new() una nuova stringa
        let mut guess = String::new();

        //uso la funzione stdin che gestisce l'input
        io::stdin()
            // guess come stringa e per essere manipolata e gestita dal metodo .read_line deve essere mutabile
            // il & è per usare guess con reference
            // .read_line restituisci un enum,Result, che è Ok o Err.
            .read_line(&mut guess)
            // il metodo .expect è per gestire gli errori, non è necessario ma in compiling darebbe un warning
            .expect("Failed to read line");

        // prende guess e lo compara a secret_number, se però le due cose sono comparabili.
        // guess è una stringa mentre secret_number un numero, perciò bisogna riconvertirla in numero
        // facendo questo Rush effettua lo Shadowing, riassegnando un valore alla variabile guess
        // trim() è per le stringhe e elimina ogni spazio vuoto prima e dopo. Serve per poterlo confrontare con u32 che è un dato numerico.
        // parse() è un metodo che traduce caratteri in numeri (se logicamente possibile)
        // usiamo il return di parse per gestire l'errore con match
        // il continue è per dire vai al prossimo giro di loop
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // altro modo println!("You guess: {guess}")
        println!("You guess: {}", guess);

        // aggiungere rand alle dipendenze:
        // rand = "0.8.5"
        // cargo userà le API 0.8.5 e se non le trova le cercherà tra le 0.8.5 fino a quella prima di 0.9.0 e non oltre
        // $ cargo update aggiorna le librerie.
        // anche se delle librerie vengono aggiornate online, se non si usa update, le librerie locali rimarranno invariate

        match guess.cmp(&secret_number) {
            // Ordering è un'altro enum con queste caratteristiche
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
