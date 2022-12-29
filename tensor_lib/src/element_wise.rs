use num_traits::Num;

pub trait ElementWise<T: Copy> {
    fn element_wise<F>(&mut self, func: F) where F: Fn(T) -> T;
}

pub trait ElementWiseOperations<T: Copy> {
    fn mul_assign(&mut self, rhs: T);

    fn div_assign(&mut self, rhs: T);
}

pub trait NumberElementWise<T: Copy + Num>: ElementWise<T> + ElementWiseOperations<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.element_wise(|value| value * rhs);
    }

    fn div_assign(&mut self, rhs: T) {
        self.element_wise(|value| value / rhs);
    }
}
