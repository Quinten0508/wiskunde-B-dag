use std::f64::consts::PI;

fn main () {
// Start een timer voor een race tegen Python
// let start = std::time::Instant::now();

    let n = 1 as f64;
    let lower_bound = -10000000;
    let upper_bound = 10000000;

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
    println!("The closest number to 1 in the vector is {}", closest);


    //  Vind de bijbehorende index in vec_x van closest en print de waardes in vec_x en vec_a
    let mut index = 0;
    for value in vec_v.iter() {
        if *value == closest {
            println!("The corresponding value in vec_x is {}", vec_x[index]);
            println!("The corresponding value in vec_a is {}", vec_a[index]);
        }
        index += 1;
    }
    // Einde timer
    // let duration = start.elapsed();
    // println!("Time elapsed is: {:?}", duration);
}

