fn main() {
    let redLight = TrafficLight::Red;
    let greenLight = TrafficLight::Green;
    let yellowLight = TrafficLight::Yellow;
    println!("Red light is {}", redLight.time());
    println!("Green light is {}", greenLight.time());
    println!("Yellow light is {}", yellowLight.time());
}


enum TrafficLight {
    Red,
    Green,
    Yellow
}

impl TrafficLight {
    
    fn time(&self) -> u8 {
        // 10
        match &self {
            TrafficLight::Red => 10,
            TrafficLight::Green => 20,
            TrafficLight::Yellow => 30,
        }
    }
}