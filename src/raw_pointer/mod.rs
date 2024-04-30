#[allow(dead_code)]
pub fn run() {
    const_pointer();
    mut_pointer();
    pointer_to_array();
    array_by_slice();
    pointer_to_vec();
}

fn const_pointer() {
    let u = 1;
    let pu: *const i32 = &u;
    println!("u = {}", unsafe { *pu });
}

fn mut_pointer() {
    let mut u = 1;
    let pu: *mut u8 = &mut u;
    println!("u = {}", unsafe { *pu });
    unsafe { *pu = 2 };
    println!("u = {}", unsafe { *pu });
}

fn pointer_to_array() {
    let a = [1, 2, 3, 4, 5];
    let mut p: *const i32 = &a[0];
    println!("v[0] = {}", unsafe { *p });
    println!("v[1] = {}", unsafe { *p.offset(1) });
    unsafe {
        p = p.offset(2);
    }
    println!("v[2] = {}", unsafe { *p });
    println!("v[1] = {}", unsafe { *p.offset(-1) });
}

fn array_by_slice() {
    let a = [1, 2, 3, 4, 5];
    let mut s = &a[..];
    let e = s[0];
    println!("v[0] = {}", e);
    s = &s[1..];
    let e = s[0];
    println!("v[1] = {}", e);
}

fn pointer_to_vec() {
    let v = vec![1, 2, 3, 4, 5];
    let p: *const i32 = &v[0];
    println!("v[0] = {}", unsafe { *p });
}
