fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let a = 42;
    print_typename(a);
    let b = |x: i32| { x * 2 };
    print_typename(b);
    let c = (1..10).skip(2);
    print_typename(c);
    let d = [1, 2, 3];
    print_typename(d);
    let e = (1, '2', "3");
    print_typename(e);
    let f = (0,);
    print_typename(f);
    let g = (0);
    print_typename(g);
}
