fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", num);
    println!("Vector is now {:?}", v);

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2);
}