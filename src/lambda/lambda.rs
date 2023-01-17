pub fn operation<T> (a: i8, b: i8, func: T) -> i8 where T: Fn (i8, i8) -> i8 {
    info!("Operation called!");

    func(a, b)
}