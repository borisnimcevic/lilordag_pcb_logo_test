struct Resistor {
    x: f32,
    y: f32,
}

struct Led {
    x: f32,
    y: f32,
}

struct Combo {
    res: Resistor,
    led: Led,
}

const NUM_OF_LED: u8 = 9;

fn main() {
    let first_resistor = Resistor { x: 43.1, y: 33.5 };

    let first_led = Led { x: 43.6, y: 35.25 };

    let x_difference = first_led.x - first_resistor.x;

    let first_combo = Combo {
        res: first_resistor,
        led: first_led,
    };

    let last_led = Led { x: 110.4, y: 35.25 };

    let led_difference = (last_led.x - first_combo.led.x) / (NUM_OF_LED - 1) as f32;
    println!("step betwen leds = {}", led_difference);

    for led_num in 0..NUM_OF_LED {
        let x = first_combo.led.x + (led_num as f32 * led_difference);
        println!("led{} - x:{} , y:{}", led_num, x, first_combo.led.y);
        let res_x = x - x_difference;
        println!("res{} - x:{} , y:{}", led_num, res_x, first_combo.res.y);
        println!("");
    }
}
