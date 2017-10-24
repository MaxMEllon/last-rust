fn main() -> () {
    let list: Vec<u32> = vec![1, 2, 3, 4, 5];
    fn f(v: &u32) -> bool { 3.eq(v) }
    let func: fn(&u32) -> bool = f
    let result = find::<u32>(&list, func).unwrap();
    assert!(3.eq(result));
}

fn find<T>(xs: &Vec<T>, f: fn(&T) -> bool) -> Option<&T> {
    // for x in xs {
    //     if f(x) {
    //         return Some(x);
    //     }
    // }
    // return None
    xs.iter().find(|&x| f(x))
}
