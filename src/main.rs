enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) {
    match light {
        // i am new to match but for your qustion i will use my brain and try to tell what i understand
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Slowdown"),
        TrafficLight::Green => println!("Go"),
    }
}
fn main() {
    action(TrafficLight::Red);
    action(TrafficLight::Yellow);
    action(TrafficLight::Green);
}
