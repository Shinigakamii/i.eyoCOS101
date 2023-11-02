<<<<<<< HEAD
fn main(){
    let P:f64 = 520000000.0;
    let R:f64 = 10.0;
    let T:f64 = 5.0;
    let A:f64 = P*(1.0+((R/100.0).powf(T)));
    let CI:f64 = A - P;
    println!("Your compound interest is {} and your amount is {}", CI, A);
=======
fn main(){
    let P:f64 = 520000000.0;
    let R:f64 = 10.0;
    let T:f64 = 5.0;
    let A:f64 = P*(1.0+((R/100.0).powf(T)));
    let CI:f64 = A - P;
    println!("Your compound interest is {} and your amount is {}", CI, A);
>>>>>>> 528500c973e42d9bcebed98287a0cc0fa1074069
    }