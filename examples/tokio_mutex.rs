use std::sync::{Arc, Mutex};
use tokio::{task::spawn, time::{sleep, Duration}};

struct SharedInt {
    val: Mutex<i32>,
}
impl SharedInt {
    fn set(&self, int: i32) { // "sync" function
        let mut lock = self.val.lock().unwrap();
        *lock = int;
    }
}
#[tokio::main]
async fn main() {
    let share_box1 = Arc::new(Mutex::new(Box::new(0)));
    let share_box2 = share_box1.clone();

    spawn(async move {
        set_int1(&share_box1, 2).await;
        sleep(Duration::from_millis(500)).await;
        set_int1(&share_box1, 3).await;
    });
    spawn(async move {
        set_int1(&share_box2, 4).await;
        set_int1(&share_box2, 5).await;
    });

    let share_struct1 = Arc::new(SharedInt{val: Mutex::new(0)});
    let share_struct2 = share_struct1.clone();

    spawn(async move {
        set_int2(&share_struct1, 6).await;
        set_int2(&share_struct1, 7).await;
    });
    spawn(async move {
        set_int2(&share_struct2, 8).await;
        set_int2(&share_struct2, 9).await;
    });

    // Hold until end of async func
    hold_program().await;
    println!("Finish program");
}

async fn set_int1(share_int: &Arc<Mutex<Box<i32>>>, int: i32) {
    { // If no blocking the codes, compile error due to living of Mutex at next await.
        let mut lock_int = share_int.lock().unwrap();
        **lock_int = int;
    }
    println!("Set value: {}", int);
    sleep(Duration::from_secs(1)).await;
}
async fn set_int2(share_int: &SharedInt, int: i32) {
    share_int.set(int);
    println!("Set value: {}", int);
}
async fn hold_program() {
    sleep(Duration::from_millis(5000)).await;
}