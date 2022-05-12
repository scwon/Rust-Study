use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

// fn main() {
//     // 스레드 생성
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     // 해당 스레드가 종료될 때까지 메인 스레드 중지
//     // handle.join().unwrap();


//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     //메인 스레드가 멈추면 끝

//     // 해당 스레드가 종료될 때까지 기다린다.
//     // handle.join().unwrap();
// }



// fn main() {
//     let v = vec![1, 2, 3];

//     // move 를 통해 소유권을 가져감
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         // val을 보냈는데 더이상 뭘 하려고 하면 혼난다.
//         // println!("val is {}", val);
//     });

//     // recv 로 send가 올 때까지 메인 스레드 대기
//     // try_recv 의 경우 블록 안함 
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     // 송신기가 두개 필요하니 복사...
//     let tx1 = mpsc::Sender::clone(&tx);
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
    
//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
    
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];
    
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
    
//     // 잘 기다린다.
//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

 
// fn main() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     let handle = thread::spawn(move || {
//         let mut num = counter.lock().unwrap();

//         *num += 1;
//     });
//     handles.push(handle);

//     // ERROR ! 여러 소유자가 counter 를 소유하려함
//     let handle2 = thread::spawn(move || {
//         let mut num2 = counter.lock().unwrap();

//         *num2 += 1;
//     });
//     handles.push(handle2);

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

fn main() {
    // 아토믹 참조자로 뮤텍스를 감싸면
    // 여러 스레드에서 뮤텍스를 공유할 수 있다...
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}