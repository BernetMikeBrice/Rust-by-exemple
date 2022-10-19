// Dérivez l'implémentation `fmt::Debug` pour `Structure`. "Structure"
// est une structure qui contient un seul `i32`.
#[derive(Debug)]
struct Structure(i32);

// Placer une `Structure` à l'intérieur de la structure `Deep`. Rendez-le imprimable aussi
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // L'impression avec `{:?}` est similaire à celle avec `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor}:? name.",
             "Slater",
             "Christian",
             actor = "actor's");

    println!("Now {:?} will print", Structure(3)); 
    
    
    println!("Now {:?} will print", Deep(Structure(7)));
            


}