// Un attribut pour masquer les avertissements pour le code inutilisé.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Une structure d'unité
struct Unit;

// Une structure tuple
struct Pair(i32, f32);

// Une structure avec deux champs
struct Point {
    x: f32,
    y: f32,
}

// Les structures peuvent être réutilisées comme champs d'une autre structure
struct Rectangle {
    // Un rectangle peut être spécifié par où le haut à gauche et le bas à droite
    // les coins sont dans l'espace.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Crée une structure avec un raccourci d'initialisation de champ
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    // Affiche la structure de débogage
    println!("{:?}", peter);

    // Instancie un `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Accéder aux champs du point
    println!("point coordinates {}, {}",point.x, point.y);

    // Faites un nouveau point en utilisant la syntaxe struct update pour utiliser les champs de notre
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` sera le même que `point.y` car nous avons utilisé ce champ
    // à partir du `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Déstructurer le point en utilisant une liaison `let`
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // l'instanciation de struct est aussi une expression
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instancie une structure d'unité
    let _unit = Unit;

    // Instancie une structure tuple
    let pair = Pair(1, 0.1);

    // Accéder aux champs d'une structure tuple
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Déstructurer une structure tuple
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

}