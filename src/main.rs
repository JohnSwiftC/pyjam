pub mod numbers;
pub mod strings;
fn main() {
    //println!("{}", numbers::bracketify(2147483647));
    //println!("{}", strings::charbychar("hello!".as_bytes()));

    // Cheap full solution for fun, we can just read a python script as a byte array and 
    // throw it in an exec

    let bytes = std::fs::read(std::env::args().last().unwrap()).unwrap();
    println!("class JohnsSuperFunRev:pass");
    println!("obj = JohnsSuperFunRev()");
    println!("fun={}", strings::charbychar(&bytes));
    println!("[+obj for obj.__class__.__pos__ in [\"\".__class__.__subclasses__]]");
    println!("[obj[fun] for obj.__class__.__getitem__ in [exec]]");
}
