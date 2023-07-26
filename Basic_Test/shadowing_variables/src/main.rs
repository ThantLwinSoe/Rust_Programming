fn main() {
    println!("Hello, Shadowing Varibales!");

    let x = 5 ;
    let x = x + 1;

    {
    	let x = x *2 ;
    	println!("The value of x : {x}");
    }

    println!("The value of x : {x}");


    let spaces = "     ";
    let spaces = spaces.len();

    println!("Length of spaces : {spaces} ");
}
