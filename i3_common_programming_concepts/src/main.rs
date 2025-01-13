fn main() {
    // Questo programma è possibile solo se x è dichiarata mutabile
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The new value of x is {x}");

    // Le costante sono immutabili per default, e il loro valore deve essere dato prima del runtime
    // Scritte in maiuscolo
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing: andare a riassegnare un valore per una variabile il cui scopo è finito
    let x = 5;
    let x = x + 1;
    {
        // Qui lo shadowing è solo annidato, una volta finito l'annidamento lo shadowing finisce
        let x = x * 2;
        println!("The value of x in the innerscope is {x}");
    }

    println!("The value of x is {x}");

    // La differenza con il mut è che non stiamo assegnando un nuovo valore ma creando una nuova variabile
    // che può essere anche di tipo diverso, ma riusando un nome che non serve più.

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Gli spazi sono {spaces}");

    // Rust è un statically typed language, cioè deve conoscere il tipo di variabile alla compilazione
    // Spesso il tipo è automatico, ma a volte può essere ambiguo (come un numero in int o stringa)
    // In questo caso definiamo il tipo, sennò lo farà il compilatore con un'errore in compilazione
    let guess : u32 = "42".parse().expect("Not a number!");

    // Rust ha 4 dati scalari, cioè che rappresentano un singolo valore
    // int, float, bool, characters
    // int puoi essere i(con segno) o u(senza segno) come i16 o u32, il numero è la lunghezza in bit

    // float, vengono definiti automaticamente rispetto l'archiettura (qui f64), sennò definiti a mano
    let x = 2.0; // f64
    let y : f32 = 2.0;  // f32

    // Bool, anche qui sono riconosciuti automaticamente
    let t = true; // t è bool
    let f: bool = false; // with explicit type annotation

    // Character
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Operazioni
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Rust ha tipi di dato compound, cioè hanno multipli valori: tuple e array
    // Tuple, collezioni di vario tipo di variabili ma con lunghezza finita. Una volta assegnata, non è cambiabile
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Accesso all valore della tupla tramite indice
    let five_hundre = tup.0
    let six_point_four = tup.1

    // Decostruzione di una tupla: usare questo pattern, spezza il compound in variabili separate
    let (x, y, z) = tup;
    println!("The value of y is {y}");

    //Array, collezione di stessa tipologia, si può usare questa definizione per dire che tutti e 3 i valori sono i32
    let a : [i32,3]= [1,2,3];

    // Accesso al valore singolo dell'array con indice
    let x = a[0];

    // 3.3 Functions
    // si usa lo snake_case
    // si passa al progetto functions
    //l'ordine di dichiarazione delle funzione non è importante
}
