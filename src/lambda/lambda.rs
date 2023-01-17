pub fn operation<T> (a: i8, b: i8, func: T) -> i8 where T: Fn(i8, i8) -> i8 {
    info!("Operation called!");

    func(a, b)
}

pub fn something<T> (func: T) where T: Fn() {
    println!("Something called!");

    func();
}
pub fn logging_levels<T: Fn()> (func: T) {
    func();
}