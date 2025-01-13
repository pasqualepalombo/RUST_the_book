fn main() {
    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 or 2");
    }
    //se ci sono più di un elseif probabile che bisogna fare refactoring

    //if with let statement
    //poichè if è un expression possiamo usarla con let per assegnare valori
    //però le scelte devono essere della stessa tipologia perciò non 5 e six
    let condition = true;
    let number = if condition {5} else {6};
    println!("the value of number is: {number}");

    //LOOP: while, loop e for

    //loop è infinito e bloccato da break o continua saltando il resto del codice con continue
    //il codice è commentato perchè sarebbe un loop infinito:
    //loop {
    // println!("again!");
    //}

    //loop con ritorno
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the counter value is {result}");

    //Loop labels
    // si può mettere un etichetta al loop cosi da poterla usare nel break per dire quale interrompere
    // le etichette hanno solo un apostrofo
    
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //Looping in a collections with while e for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

