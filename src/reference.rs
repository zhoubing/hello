pub fn ref_demo() {
    let a = 10;
    let b = &a;
    let c = a + *b;
    println!("{}", c);

    let arr = [1 ,2 ,3, 4, 5, 6];
    for n in &arr {
        println!("{}", *n);
        println!("{}", n);
        println!("{}", *n == 2);
    }
}