// from here
// https://www.tutorialspoint.com/rust/rust_structure.htm

struct Employee {
    name:String,
    company:String,
    age:u32
 }
 fn main() {
   /* let emp_org = Employee {
       company:String::from("TutorialsPoint"),
       name:String::from("Mohtashim"),
       age:50
    };
    */
    
    let emp1: Employee = Employee {company:String::from("TutorialsPoint"),name:String::from("Mohtashim"),age:50 };

    println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
 }