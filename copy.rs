use std::f64::consts::PI;
use std::io;

fn main () {
// Start een timer voor een race tegen Python
// let start = std::time::Instant::now();

    let n = 1 as f64;

    println!("Voer het totaal aantal iteraties in.");
    let mut iterations = String::new();
    io::stdin().read_line(&mut iterations).expect("Failed to read line");
    let mut iterations: i64 = iterations.trim().parse().expect("Please type a number!");

    // Waarschuwing als iterations > 10000000
    if iterations > 100_000_000 {
        println!("Waarschuwing: werken met meer dan 1 000 000 000 iteraties kan lang duren en gebruikt veel werkgeheugen, of crasht je computer. Doorgaan op eigen risico!");
        println!("Nieuwe waarde invoeren? (Y/n)");

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        let answer = answer.trim().to_lowercase();

        match answer.as_str() {

            "n" => println!("Ok, we gaan door met {} iteraties.", iterations),

            "y" | _ => {
                println!("Voer een nieuwe waarde in:");
                let mut new_iterations = String::new();
                io::stdin().read_line(&mut new_iterations).expect("Failed to read line");
                let new_iterations: i64 = new_iterations.trim().parse().expect("Please type a number!");
                iterations = new_iterations;
            }
        }
    }
    let lower_bound = -1 * iterations;
    let upper_bound = iterations;

    let mut vec_x : Vec<i64> = Vec::new();
    let mut vec_a : Vec<f64> = Vec::new();
    let mut vec_v : Vec<f64> = Vec::new();

    for x in lower_bound..=upper_bound{
        let temp = (1-5*x) as f64;
        let y = temp/PI as f64;
        let a = y.round() as f64;
        let x = x as f64;
        let v = 5.0*x+a*PI as f64;
        let x = x as i64;
        vec_x.push(x);
        vec_a.push(a);
        vec_v.push(v);

    }
    // Vind de waarde van v die het dichtst bij n komt
    let mut closest = 0.0;
    let mut difference = f64::INFINITY;
    
    for value in vec_v.iter() {
        let d = f64::abs(value - n);
        if d < difference {
            closest = *value;
            difference = d;
        }
    }    
    println!("Het nummer het dichtste bij 1 is {}", closest);


    //  Vind de bijbehorende index in vec_x van closest en print de waardes in vec_x en vec_a
    let mut index = 0;
    for value in vec_v.iter() {
        if *value == closest {
            println!("De bijbehorende waarde in in vec_x is {}", vec_x[index]);
            println!("De bijbehorende waarde in vec_a is {}", vec_a[index]);
        }
        index += 1;
    }
    // Einde timer
    // let duration = start.elapsed();
    // println!("Time elapsed is: {:?}", duration);
}

