trait TupleUnwrap<T> {
    fn tuple_into(self) -> T;
}

impl<T> TupleUnwrap<T> for T {
    fn tuple_into(self) -> T {
        self
    }
}


impl<T> TupleUnwrap<T> for (T,) {
    fn tuple_into(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_impl() {
        let x = 42;
        let y = 42;
        let s = "Hello!";

        let _x: i32 = x.tuple_into();
        let _y: u32 = y.tuple_into();
        let _s = s.tuple_into();
    }

    #[test]
    fn tuple_impl() {
        let x = (42,);
        let y = (42,);
        let s = ("Hello!",);

        let _x: i32 = x.tuple_into();
        let _y: u32 = y.tuple_into();
        let _s: &str = s.tuple_into();
    }
}
