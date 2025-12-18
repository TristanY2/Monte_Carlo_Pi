use rand::Rng;
use std::f32::consts::PI;
use std::time::Instant;

#[unsafe(no_mangle)]
pub extern "C" fn monte_carlo_rust(n: u32) -> f64 {
    let mut dentro: u32 = 0;
    let rang: u32 = n;

    let start = Instant::now();

    let mut rng = rand::thread_rng();
    for _i in 0..rang {
        //let x: f64 = rand::thread_rng().gen_range(-1.0..=1.0);
        //let y: f64 = rand::thread_rng().gen_range(-1.0..=1.0);
        let x: f64 = rng.gen_range(-1.0..=1.0);
        let y: f64 = rng.gen_range(-1.0..=1.0);
        if (x * x + y * y) < 1.0 {
            dentro += 1;
        }
    }

    let per: f64 = dentro as f64 / (rang - 1) as f64;
    let per = per * 4.0;
    let dur = start.elapsed();

    println!("Dentro del circulo hubo {}-{}%", dentro, &per * 100.0 / 4.0);
    println!("Aproximacion de pi: {}", &per);
    println!("Valor real de pi:   {PI}");
    println!("Error de {}%", 1.0 - per / std::f64::consts::PI);
    println!("\nCalculado en {:#?}", dur);
    return per;
}
