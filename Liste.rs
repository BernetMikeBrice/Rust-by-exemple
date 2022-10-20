use std::fmt; // Importe le module `fmt`.

// Définit une structure nommée `List` contenant un `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extraire la valeur en utilisant l'indexation de tuple,
        // et créer une référence à `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Itération sur `v` dans `vec` tout en énumérant l'itération
        // compte dans `count`.
    for (count, v) in vec.iter().enumerate() {
        // Pour chaque élément sauf le premier, ajoutez une virgule.
        // Utilisez le ? opérateur pour revenir en cas d'erreur.
            if count !=0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        
        // Fermez la parenthèse ouverte et renvoie une valeur fmt::Result.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}