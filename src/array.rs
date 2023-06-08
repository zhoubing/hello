pub fn handle_array() {
    let array5: [i32; 5] = [1, 2, 3, 4, 5];
    let array4: [i32; 4] = [1, 2, 3, 4];
    //array4和array5是不同的两个类型的数据。也就是说数组的类型是包括长度的

    for a in &array5 {
        println!("{}", a);
    }
}