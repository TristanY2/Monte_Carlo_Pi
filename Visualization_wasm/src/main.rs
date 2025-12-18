use macroquad::prelude::*;
use std::collections::VecDeque;
//use ::rand::prelude::*;

struct Punto {
    x: f32,
    y: f32,
    es_dentro: bool,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Monte Carlo Pi".to_owned(),
        window_resizable: true,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    let mut historial: VecDeque<Punto> = VecDeque::with_capacity(10001); 
    
    let mut total_count: u64 = 0;
    let mut inside_count: u64 = 0;

    loop {
        clear_background(BLACK);

        let width = screen_width();
        let height = screen_height();
        let radius = height.min(width);

        // Puntos por frame
        for _ in 0..10000 {

            let x = rand::gen_range(-1.0, 1.0);
            let y = rand::gen_range(-1.0, 1.0);
            let es_dentro = (x*x + y*y) <= 1.0;

            total_count += 1;
            if es_dentro { inside_count += 1; }

            historial.push_back(Punto { x, y, es_dentro });

            if historial.len() > 10000 {
                historial.pop_front();
            }
        }
        // Dibujar borde del círculo
        draw_circle_lines(radius/2.0, radius/2.0, radius/2.0, 2.0, WHITE);

        // Dibujar los puntos del historial
        for p in &historial {
            let screen_x = (p.x + 1.0) * 0.5 * radius;
            let screen_y = (p.y + 1.0) * 0.5 * radius;

            let color = if p.es_dentro { GREEN } else { RED };
            
            draw_circle(screen_x, screen_y, 2.0, color);
        }

        // 4. HUD
        let pi_aprox = (inside_count as f64 / total_count as f64) * 4.0;
        let error = (pi_aprox - std::f64::consts::PI).abs() / std::f64::consts::PI;
        draw_text(&format!("Puntos: {}", total_count), 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Pi: {:.6}", pi_aprox), 20.0, 60.0, 30.0, YELLOW);
        draw_text(&format!("Error: {:.7}%", error*100.0), width - 240.0, 30.0, 30.0, WHITE);

        next_frame().await;
    }
}