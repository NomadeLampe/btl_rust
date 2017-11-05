fn main() {
    //    println!("Hello, world!");
    //    mutable();
    //    pattern_matching();
    //    if_else();
    //    boucle();
    //    exemple_fonctions();
    utilisation_lib_externe();
}


///
/// Par défaut les variables sont immutable
///
fn mutable() {
    let ma_variable_immutable = 33;

    //    Ne compile pas
    //    ma_variable_immutable = 34;

    let mut ma_variable_mutable = 33;
    ma_variable_mutable = 34;
}


///
/// Pattern matching équivalent au switch mais tous les cas doivent être gérés.
/// Le premier match est
///
fn pattern_matching() {
    let nombre = 11;


    match nombre {
        // Match une seule valeur
        1 => println!("Un !"),
        // Match plusieurs valeurs
        2 | 3 | 5 | 7 | 11 => println!("Nombre premier"),
        // Match une portée de nombres inclusif
        8 ... 19 => println!("[13;19]"),
        // Obligatoire pour les nombres pour les cas non gérés
        _ => println!("Un autre nombre"),
    }

    let my_boolean = true;

    // pas besoin de _=>
    let binary = match my_boolean {
        false => 0,
        true => 1
    };

    println!("résultat pattern matching boolean -> {}", binary);
}

///
/// Pas besoin de parenthèse
///
fn if_else() {
    let n = 5;

    if n < 0 {
        print!("{} est negatif", n);
    } else if n > 0 {
        print!("{} est positif", n);
    } else {
        print!("{} == zero", n);
    }
}

fn boucle() {
    let mut i = 0;
    loop {
        if i == 10 {
            println!("Terminé : {}", i);
            break;
        } else {
            println!("Pas encore : {}", i);
            i += 1;
        }
    }


    let mut n = 1;

    while n < 10 {
        println!("{}", n);
        n += 1;
    }

    for m in 1..10 {
        println!("{}", m);
    }
}

///
/// Le return est facultatif
///
fn addition_court(nb1: i32, nb2: i32) -> i32 {
    nb1 + nb2
}

fn addition_long(nb1: i32, nb2: i32) -> i32 {
    return nb1 + nb2;
}

fn methode_sans_retour() {
    println!("Je ne fais rien")
}

fn methode_comme_java_void() -> () {
    println!("Comme java void")
}

fn exemple_fonctions() {
    let res_a = addition_court(1, 2);
    let res_b = addition_long(1, 2);

    println!("Résultat des additions : ");
    println!("addition_court -> {}", res_a);
    println!("addition_long -> {}", res_b);

    methode_sans_retour();
    methode_comme_java_void();
}


extern crate time;

fn utilisation_lib_externe() {
    println!("Il est {:?}h{:?}m depuis la lib externe Time", time::now().tm_hour, time::now().tm_min);
}

