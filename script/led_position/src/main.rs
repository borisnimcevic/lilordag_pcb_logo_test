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
    let first_resistor = Resistor { x: 121.5, y: 43.0 };

    let first_led = Led { x: 122.0, y: 44.75 };

    let y_difference = first_led.y - first_resistor.y;

    let first_combo = Combo {
        res: first_resistor,
        led: first_led,
    };

    let last_led = Led {
        x: 112.0,
        y: 115.25,
    };

    let led_difference = (last_led.y - first_combo.led.y) / (NUM_OF_LED - 1) as f32;
    println!("step betwen leds = {}", led_difference);

    for led_num in 0..NUM_OF_LED {
        let y = first_combo.led.y + (led_num as f32 * led_difference);
        println!("led{} - x:{} , y:{}", led_num, first_combo.led.x, y);
        let res_y = y - y_difference;
        println!("res{} - x:{} , y:{}", led_num, first_combo.res.x, res_y);
        println!("");
    }
}
