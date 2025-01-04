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

    // Tuple
    {
        title("Tuple");
        // Les tuples contiennent plusieurs éléments de type différent
        let perso = (32, "Nemesty", 1.72);
        // Pattern matching pour déstructurer le tuple
        let (age, name, height) = perso;
        println!("Bonjour {}, tu as donc {} ans et tu mesures {}m.",name, age, height);
        // Il également possible d'accèder à chaque élément par un point
        let mut age = perso.0;
        age += 1;
        println!("Une année est passée vous avez maintenant {} ans.", age);
    }

    // Array
    {
        title("Array");
        // Les array contiennet plusieurs éléments de même type
        const DAYS_OF_WEEK: [&str; 7] = ["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche"];
        println!("Le premier jour de la semaine est le : {}", DAYS_OF_WEEK[0]);

    }
}
