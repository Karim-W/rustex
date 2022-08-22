use std::thread;
use uuid::Uuid;
pub fn thrd () {
    let mut rng = rand::thread_rng();
    let mut handlers:Vec<thread::JoinHandle<()>> = Vec::new();
    for n in 0..20 {
        let mut handler = thread::spawn(move|| {
            let id = Uuid::new_v4();
            let val = String::from("Thread number:".to_owned()+&n.to_string()+" was given this id: "+&id.hyphenated().to_string().as_str());
            println!("{}",val);
        });
        handlers.push(handler);
    }
    for h in handlers {
        h.join().unwrap();
    }
}
