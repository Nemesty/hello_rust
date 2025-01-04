fn title(titre: &str) {
    println!("\n⭐ {} :", titre);
}

fn main() {

    // Variables
    {
        title("Variables");
        // Variable non modifiable :
        let x = 5;
        // Variable modifiable :
        let mut y = 10;
        y += 1;
        // Shadow une variable :
        let i = "Nemesty";
        let i = i.len();
        println!("x = {}\ny = {}\ni = {}", x, y, i);
    }

    // Constantes
    {
        title("Constantes");
        // Le type doit être indiqué avec les constantes
        const VALUE_OF_PI: f64 = 3.1415;
        println!("La valeur de Pi est : {}", VALUE_OF_PI);
    }

    
}
