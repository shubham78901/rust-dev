fn main() {

 
    // second_fn(45) ;
    // then_fn(3,'H');
    let x=returningfunction(6);
    println!("the returned value from function is ={}",x);
}


fn first_fn() {
    println!("This is the first function");
}

fn second_fn(x :i32) {
    println!("This is the second function ={}",x);

}
fn then_fn(x :i32,y: char) {

    println!("This is the second function ={} and y={}",x,y);


}

fn returningfunction(x :i32) -> i32{
    return x+1;
    
}