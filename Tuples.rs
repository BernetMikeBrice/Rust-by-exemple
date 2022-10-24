// Les tuples peuvent être utilisés comme arguments de fonction et comme valeurs de retour
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` peut être utilisé pour lier les membres d'un tuple à des variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// La structure suivante est pour l'activité.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // Un tuple avec un tas de types différents
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                    'a', true);

 // Les valeurs peuvent être extraites du tuple en utilisant l'indexation de tuple                   
println!("long tuple first value: {}", long_tuple.0);
println!("long tuple second value: {}", long_tuple.1);

// Les tuples peuvent être des membres de tuple
let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

// Les tuples sont imprimables
println!("tuple of tuples: {:?}", tuple_of_tuples);

// Mais les tuples longs (plus de 12 éléments) ne peuvent pas être imprimés
// soit too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
 // println!("tuple trop long : {:?}", too_long_tuple);
 // TODO ^ Décommentez les 2 lignes ci-dessus pour voir l'erreur du compilateur

let pair = (1, true);
println!("pair is {:?}", pair);

println!("the reversed pair is {:?}", reverse(pair));

 // Pour créer des tuples à un élément, la virgule est nécessaire pour les différencier
 // à partir d'un littéral entouré de parenthèses
 
println!("one element tuple: {:?}", (5u32,));
println!("just an interger: {:?}", (5u32));
 //les tuples peuvent être déstructurés pour créer des liaisons
let tuple = (1, "hello", 4.5, true);

let (a, b, c, d) = tuple;
println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
println!("{:?}", matrix);





}