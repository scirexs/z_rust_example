use tokio::{join, sync::{mpsc::{self, Receiver, Sender}, oneshot}, task::spawn};

struct Pack {
    num: i32,
    res: oneshot::Sender<bool>,
}

#[tokio::main]
async fn main() {
    let mut share_int = Box::new(0);
    let (s1, s2, mut r) = get_mpsc_channel_w2sender(16);
    let server = spawn(async move { get_server(&mut r, &mut share_int).await });
    let task1 = spawn(async move { send_msg(s1, 1).await });
    let task2 = spawn(async move { send_msg(s2, 2).await });

    let (_, _) = join!(task1, task2);
    let _ = server.await.unwrap();

    println!("Finish program");
}

fn get_mpsc_channel_w2sender<T>(buffer: usize) -> (Sender<T>, Sender<T>, Receiver<T>) {
    let (sender1, receiver) = mpsc::channel(buffer);
    let sender2 = sender1.clone();
    (sender1, sender2, receiver)
}

async fn get_server(r: &mut Receiver<Pack>, share_int: &mut Box<i32>) {
    while let Some(Pack{num, res}) = (*r).recv().await {
        println!("Set int: {} -> {}", share_int, num);
        **share_int = num;
        let _ = res.send(true); // Ignore error
    }
}

async fn send_msg(s: Sender<Pack>, num: i32) {
    let (res_s, res_r) = oneshot::channel();
    let pack = Pack{num, res: res_s};
    if s.send(pack).await.is_err() {
        eprintln!("Conn shutdown");
        return;
    }
    if res_r.await.unwrap() {
        println!("Success set int.");
    } else {
        println!("Fail set int");
    }
}