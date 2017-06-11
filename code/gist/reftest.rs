#[derive(Debug)]
struct Astruct {
    vala: i32,
}

#[derive(Debug)]
struct Bstruct<'a> {
    revvala: &'a Astruct,
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

fn save_in_structure(b: &Astruct) -> Bstruct {
    let c: Bstruct = Bstruct { revvala: b };
    c
}

fn main() {
    let mut a: Astruct = Astruct { vala: 42 };
    test1(&a);
    testmut(&mut a);
    test2(&a);
    let mut b = save_in_structure(&a);
    println!("Structure with reference {:?}", b);
}
