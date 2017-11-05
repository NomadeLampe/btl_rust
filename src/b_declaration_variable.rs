fn main() {
    // Déclaration d'un int32 immutable
    let i: i32 = 22;

    // Déclaration équivalente
    let j = 22;

    // Ou encore
    let k = 22i32;

    if (i == j) && (j == k) {
        println!("Les variables sont identiques");
    } else {
        println!("Les vairbales ne sont pas identiques");
    }

    // Pour obtenir une variable mutable il faut ajouter le préfix mut
    let mut l = 22;
    l = 35;
}

// 2 déclarations de constantes

const MA_CONSTANTE_A: String = "Aloha";
static MA_CONSTANTE_B: i32 = 42;