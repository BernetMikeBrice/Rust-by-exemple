use std::fmt;

// Une structure contenant deux nombres. `Debug` sera dérivé afin que les résultats puissent
// être mis en contraste avec `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implémenter `Display` pour `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Utilisez `self.number` pour faire référence à chaque point de données de position.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Définit une structure où les champs sont nommables pour comparaison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     
    write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };
    
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Erreur. `Debug` et `Display` ont été implémentés, mais `{:b}`
    // nécessite l'implémentation de `fmt::Binary`. Cela ne fonctionnera pas
    // println!("A quoi ressemble Point2D en binaire : {:b}?", point);
}