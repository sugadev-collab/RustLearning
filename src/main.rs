#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new(value: i32) -> Self {
        Self { value }
    }

    fn show(&self) {
        print!("Counter: {:?}", self);
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn consume(self) {
        println!("Your are deleting the : {:?} by calling this method", self);
    }
}

fn main() {
    let mut counter = Counter::new(0);
    println!("Ititial value: ");
    counter.show();

    counter.increment();
    println!("After Increment: ");
    counter.show();

    counter.consume();

    println!("Consumed....");
    // counter.show(); the counter value moved so this is unavailable.
}
