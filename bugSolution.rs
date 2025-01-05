use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    
    {   
        let y = x.clone();
        let mut y_lock = y.lock().unwrap();
        *y_lock = 10;
    }
    
    {   
        let z = x.clone();
        let mut z_lock = z.lock().unwrap();
        *z_lock = 15;
    }
    
    println!("x = {}", *x.lock().unwrap());
} 