

pub type Supplier<T> = fn() -> T;

pub type Consumer<T> = fn(T);
pub type BiConsumer<T1, T2> = fn(T1, T2);

pub type UnaryOperator<T> = fn(T) -> T;