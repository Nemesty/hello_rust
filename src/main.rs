// Fonction sans paramètre
fn say_hello() {
    println!("Bonjour !");
}

// Fonction avec paramètre
// Ici la valeur passée est une référence et non un déplacement de la valeure
fn title(titre: &str) {
    println!("\n⭐ {} :", titre);
}

// Fonction avec paramètre et retour
fn sum(num1: f64, num2: f64) -> f64 {
    // La dernière expression est retourner
    // Il ne faut pas mètre de point virgule pour indiquer une expression de retour
    num1 + num2
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

    // Fonction utilisation
    {
        title("Fonction utilisation");
        say_hello();
        let number1 = 10.0;
        let number2 = 5.0;
        let result = sum(number1, number2);
        println!("{} + {} = {}", number1, number2, result);
    }

    // If & else
    {
        title("If & else");
        let weight = 75.0;
        let height = 172.0;
        let bmi = (weight / (height * 2.0)) * 100.0;
        if bmi >= 40.0 {
            println!("Votre IMC est de {:.2} => Obésité morbide.", bmi);
        } else if bmi >= 30.0 {
            println!("Votre IMC est de {:.2} => Obésité.", bmi);
        } else if bmi >= 25.0 {
            println!("Votre IMC est de {:.2} => Surpoids.", bmi)
        } else if bmi >= 18.5 {
            println!("Votre IMC est de {:.2} => Poids normal.", bmi);
        } else {
            println!("Votre IMC est de {:.2} => Poids insuffisant.", bmi );
        }
    }

    // Loop
    {
        title("Loop");
        let mut count = 0;
        // loop créé une boucle infinie, doit être quitté avec le mot clé break
        'main_loop: loop {
            println!("Dans la boucle principale count vaut : {}", count);
            loop {
                count += 1;
                println!("Dans la sous-boucle count vaut : {}", count);
                if count == 5 {
                    println!("Ici on quitte la boucle principale en utilisant son label.");
                    break 'main_loop;
                }
            }
        }
    }

    // While
    {
        title("While");
        let mut count_down = 3;
        while count_down != 0 {
            println!("{} !", count_down);
            count_down -= 1;
            if count_down == 0 {
                println!("Décollage ! 🚀")
            }
        }
    }

    // For
    {
        title("For");
        const DAYS_OF_WEEK: [&str; 7] = ["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche"];
        let mut day_number = 0;
        for day in DAYS_OF_WEEK {
            day_number += 1;
            println!("Le jour {} est {}", day_number, day);
        }
    }

    // Ownership
    {
        let tag = "Ownership";
        // Ici je n'envoie pas la valeur à la fonction, mais seulement la référence
        title(&tag);
        // Créaiton d'une variable stockée sur le heap
        let username = String::from("Nemesty");
        // Changement de propriétaire
        let pseudo = username;
        // Impossible de faire appel à username, car la valeur a été déplacé :
        // println!("username {}", username);
        println!("pseudo : {}", pseudo);
    } // Fin du scope, donc la variable pseudo est ici libérée

    // Slice
    {
        title("Slice");
        let url = String::from("www.domaine.com");
        let url = &url[3..];
        println!("Le domaine est : {}", url);
    }
}
