use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct List(Vec<i32>);

#[derive(Copy, Clone, Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"({} {}) \n({} {})",self.0 , self.1, self.2 , self.3)
    }

}

fn transpose(m: Matrix) -> Matrix {
    return Matrix(m.1,m.0,m.3,m.2)
}

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    let v = List(vec![1, 2, 3, 4]);

    // Pretty print
    println!("{:?}", peter);
    println!("{:?}",v);
    
    let m = Matrix(1.0,2.0,3.0,4.0);
    println!("{}",m);
    println!("{}",transpose(m));
    println!("{}",m);




}