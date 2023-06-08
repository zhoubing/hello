pub fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    return *i + *j;
}