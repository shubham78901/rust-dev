fn main() {
//     println!("Hello, world!");
//    let x= first(6);
//    println!("{}",x)
third();
}

fn first(num :i32)->i32{
   
    if num%8 == 0{
        println!("number is divisible by 8");
        return 1
    }else if num%2 == 0{
  println!("number is divisible by 2");
  return 0
   }else{
        println!("number is not divisible by 2");
        return 0
    }
    
 //   return false
}
fn third(){
    let condition =false;
    let num=if condition==false{7} else {0};
    println!("{}",num);

}