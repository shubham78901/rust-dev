

 struct Employee{

    name: String,
    age: u32,
    salary: f64,
    Company: String,
    Hobby:Hobby
 }
 struct Hobby{
    Hobby: String,
    Years: u32
 }
fn main() {
    let hub=Hobby{
        Hobby: String::from("coding"),
        Years: 5,
      };
  let mut U1=Employee{
    name: String ::from("shubham"),
    age: 30,
    salary: 5000.0,
    Company:String::from("smartledger"),
    Hobby:hub,
  };
 
  U1.Company = "smartledgere".to_string();
  println!("Employee details:");
  println!("Name:{}",U1.name);
  println!("Age:{}",U1.age);
  println!("Salary:{}",U1.salary);
  println!("Company:{}",U1.Company);
    println!("Hello, world!");
    println!("Hobby: {}",U1.Hobby.Hobby);
    U1.Hobby=Hobby{
        Hobby: String::from("reading"),
        Years: 3,
      };
    
    println!("Hobby: {}",U1.Hobby.Hobby);
}
