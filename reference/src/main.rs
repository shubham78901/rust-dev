fn main() {

   println!("reference example");

    let mut x=10;
    let   y=& mut x;
    *y+=1;
    println!("value of y is {}",*y);
    println!("value of x is {}",x);


}
