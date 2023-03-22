use std::time::Duration;
#[allow(unused_imports)]
use std::{sync::{Condvar,Mutex, Arc}, thread};

const TASK_ONE_MAX_COUNT:i32 = 200;
fn task_one(){
    // Task:
    // Using two threads, have one print odd numbers and another print even numbers.
    // Output should be in the right order.
    let counter = Arc::new((Mutex::new(0i32),Condvar::new()));
    let mut handles = Vec::new();
    let counter_a = Arc::clone(&counter);
    let counter_b = Arc::clone(&counter);
    // Initiate handle 1.
    let handle_a = thread::spawn(move||{
        let mut i = 0;
        while i <= TASK_ONE_MAX_COUNT{
            println!("i: {}", i);
            let guard_a = counter_a.0.lock().unwrap();
            counter_a.1.wait_while(guard_a, |count| *count %2 ==0 ).unwrap();
            let mut num_a = counter_a.0.lock().unwrap();
            *num_a += 1;
            i = *num_a;
        }

    });

    // Initiate handle 2.
    let handle_b = thread::spawn(move||{
        let mut j = 0;
        while j <= TASK_ONE_MAX_COUNT{
            let guard_b = counter_b.0.lock().unwrap();
            counter_b.1.wait_while(guard_b, |count| *count %2 ==0 ).unwrap();
            println!("j: {}", j);
            let mut num_b = counter_b.0.lock().unwrap();
            *num_b += 1;
            j = *num_b;
            thread::sleep(Duration::from_millis(5));
        }
    });
    handles.push(handle_b);
    handles.push(handle_a);



    // Wait for all threads to conclude.
    for handle in handles{
        match handle.join(){
            Ok(_) => {},
            Err(_) => println!("Failed to join handle!"),
        }
    }
}

fn main() {
    task_one();
}
