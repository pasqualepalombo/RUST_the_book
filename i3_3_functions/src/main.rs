fn main() {
    println!("Hello, world!");
    
    another_function();
    second_function(5);
    print_labeled_measurement(5,'h');
    calling_macro();
    let x = five();
    println!("The value of x is {x}");
}

fn another_function() {
    println!("Another function");
}

//parameters
//bisogna sempre definire il tipo di parametro cosi da migliorare performance e gestione errori
fn second_function(x: i32){
    println!("The value of x is {x}");
}

//multiple parameters: separati da virgola
fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

//statement: in rust gli statement (tipo let x=6) non ritornano valori, perciò non si può fare tipo let x = y = 6
//expression: chiamare una funzione è un expression, ma anche chiamare una macro come:
fn calling_macro(){
    let y = {
        let x = 3;
        x + 1
        //x+1 non ha il punto e virgola perchè sennò diventa uno statement e non un espressione, e perciò non ritornerebbe valori.
    };

    println!("The value of y is: {y}");
}

//ritorni di valore
//non vengono chiamati, ma devono essere definiti il tipo
fn five() -> i32 {
    5
}