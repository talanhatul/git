#[allow(unused_variables)]
fn main() {
    
    let x: i32 = 5;
    let _y: i32;
    
    assert_eq!(x, 5);
    println!("good");
 
    //----
 
    let mut x: i32 = 1;
    x += 2; // 3
    
    assert_eq!(x, 3);
    println!("good2");
    
    //----
    
    
    let a = 10;
    let b = 5;
    {
        println!("a: {},  b: {}", a, b);
    }
    println!("a: {},  b: {}", a, b);
    
    define_x();
    
    let mut x = 1;
    x = 7;
    x += 3;
    
    let y = 4;
    let y = "text";
    
    println!("good3 {}",y);
    
    
    let _x = 1;
    
    let (mut x, y) = (1, 2); 
    x += 2;
    
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    
    println!("good4");
    
    
    let (x, y);
    
    (x,..) = (3, 222);
    [.., y] = [111, 2];
    
    assert_eq!([x,y], [3, 2]);
    
    println!("good5 {}, {}", x, y);
        
}



fn define_x() {
    let x: &str = "hello";
    println!("{} world", x)
}




