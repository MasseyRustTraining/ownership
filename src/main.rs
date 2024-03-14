fn _refs() {
    let mut x = 5u8;
    let x_ref_1 = &mut x;
    // let x_ref_2 = &x;
    *x_ref_1 = 6;
    // drop(x_ref_1);
    println!("{}", x);
}

fn _ownership() {
    let x: String = 5u8.to_string();
    let x_ref = &x;
    // let y: String = x;
    // x = 6u8.to_string();
    println!("{} {}", *x_ref, x);
}

fn _bump<'b>(count: &'b mut usize, a: &mut [usize; 1]) -> &'b usize {
    a[*count] += 1;
    *count += 1;
    count
}

fn _call_bump() {
    let mut a = [0];
    let mut count = 0;
    let b = _bump(&mut count, &mut a);
    // println!("{:?} {}", a, *b);
    println!("{:?} {}", a, *b);
}

fn max_ptr<'a>(x: &'a String, y: &'a String) -> &'a String {
    if *x > *y {
        x
    } else {
        y
    }
}

fn main() {
    let two = 2.to_string();
    let three = 3.to_string();
    println!("{}", *max_ptr(&two, &three));
}
