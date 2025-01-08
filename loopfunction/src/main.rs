fn main() {
    //while//for
    // println!("Hello, world!");
    // loopf();

//second()
//fourth()

fifth()
 
}


// fn loopf(){
//     let mut x=0;
//     loop{
//        x+=1;
//        println!("{}",x);
//        if x==10{
//         println!("breaking from loop");
//            break;
//        }
//     }

// }

// fn second(){

//     let  mut num=10;
//     while num !=0{
//   println!("{}",num);
//         num-=1
//     }
// }

fn fourth(){
    let arr=[1,2,3,4,5,6,7,8,9,10];
    println!("{:?}",arr);
  let len=arr.len();
    for i in 0..len{
        if i%2==0{
            println!("{} number is even",arr[i]);
        }else{
            println!("{} number is odd",arr[i]);
        }
     
    }
}
fn fifth(){

    for i in 0..4{
        if i==2{
            continue;
        }else{
            println!("{} shubham",i);
        }
       
    }
}
