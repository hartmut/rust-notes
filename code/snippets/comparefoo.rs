// from https://stackoverflow.com/questions/37829012/compare-fooi32-and-foou32
// outputs false although both values are identical, rewrite for pass

struct Foo<T> {
    id: usize,
    data: T,
}

impl Foo<i32> {
    fn new_i32(i: i32) -> Foo<i32> {
        Foo { id: 0, data: i }
    }
}

impl Foo<u32> {
    fn new_u32(u: u32) -> Foo<u32> {
        Foo { id: 1, data: u }
    }
}

impl<L, R> PartialEq<Foo<R>> for Foo<L> {
    fn eq(&self, other: &Foo<R>) -> bool {
        self.id == other.id
    }
}

fn main() {
    let a = Foo::new_u32(123);
    let b = Foo::new_i32(123);
    println!("{}", a == b);
}
