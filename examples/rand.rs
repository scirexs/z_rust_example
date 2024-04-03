use rand::prelude::*;

fn main() {
    // random<T>() is shortcut of thread_rng().gen()
    if random() { // generates a boolean
        let _ = random::<f64>(); // generates a float between 0 and 1
    }

    let mut rng1: ThreadRng = thread_rng();
    let _foo: u32 = rng1.gen_range(4_u32..9);
    let _bar: i32 = rng1.gen_range(-5..=5);
    let _baz: f64 = rng1.gen_range(1.0..2.0);
    let _qux: bool = rng1.gen_ratio(2_u32, 3); // probability of true is about 67%

    let seed: [u8; 32] = [0; 32];
    let mut rng2: StdRng = StdRng::from_seed(seed);
    let _ = rng2.gen::<f64>();

    let mut vec: Vec<i32> = (1..10).collect();
    vec.shuffle(&mut rng1);
    println!("{:?}", vec);
}