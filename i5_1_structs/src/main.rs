struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someuser@example.com"),
        sign_in_count: 1,
    };

    //guarda la funzione build_user

    let mut user2 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someuser@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("anothermail@example.com");

    //Struct Update Syntax: creare istanze su altre già definite per aggiornare solo alcuni valori
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheremail@email.com"),
        sign_in_count: user1.sign_in_count,
    };

    //Struct Update Syntax: con .. si definisce che gli altri valori rimangono uguali rispetto a user1, scrivendo meno codice
    let user4 = User {
        email: String::from("anotheranothereemail@email.com"),
        ..user1
    };

    //Tuple Structs
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    //Unit-like Structs: non hanno campi
    struct AlwaysEquals;

    let subject = AlwaysEquals;
}

//funzione che restituisce un user, si usa il field init shorthand per evitare ripetizioni. Poichè si passano email e user, non serve rispecificarle.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
