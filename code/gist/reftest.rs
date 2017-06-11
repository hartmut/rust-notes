#[derive(Debug)]
struct Astruct {
    vala: i32,
}

fn test1(b: &Astruct) {
    println!("test1 before mut {:?}", b);
}

fn test2(b: &Astruct) {
    println!("test2 after mut {:?}", b);
}

fn testmut(b: &mut Astruct) {
    b.vala = 24;
}

fn main() {
    let mut a: Astruct = Astruct { vala: 42 };
    test1(&a);
    testmut(&mut a);
    test2(&a);
}
