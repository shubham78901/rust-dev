fn main() {
    println!("Hello, world!");
    first();
}


fn first(){
    // let x=90;
    // println!("{}",x);
    // println!("{:p}",&x);
    // let y=x;
    // println!("{}",y);
    // println!("{:p}",&y);


//     let a =String :: from ("shubha gautam");

let b=a.clone();
    println!("{}",a);
    println!("{}",b);
    let x =["a", "b", "c"];
    println!("{:?}",x);
    let y=x.clone();
    println!("{:?}",y);
let a = String::from("shubha gautam");
let b = a; // Ownership transfer
println!("{}", b); // Error: "value borrowed here after move"

let val=10;
let vl2=val;
println!("val is {}",vl2);

}