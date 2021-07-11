pub(crate) trait Calculator<T>: Fn(Vec<T>) -> T {
    fn clone_box<'a>(&self) -> Box<dyn 'a + Calculator<T>> where Self: 'a;
}

impl<T, F: Fn(Vec<T>) -> T + Clone> Calculator<T> for F {
    fn clone_box<'a>(&self) -> Box<dyn 'a + Calculator<T>> where Self: 'a, {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a> Clone for Box<dyn 'a + Calculator<T>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}


pub trait BinaryCalculator<T>: Fn(T, T) -> T {
    fn clone_box<'a>(&self) -> Box<dyn 'a + BinaryCalculator<T>> where Self: 'a;
}

impl<T, F: Fn(T, T) -> T + Clone> BinaryCalculator<T> for F {
    fn clone_box<'a>(&self) -> Box<dyn 'a + BinaryCalculator<T>> where Self: 'a, {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a> Clone for Box<dyn 'a + BinaryCalculator<T>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}


pub trait UnaryCalculator<T>: Fn(T) -> T {
    fn clone_box<'a>(&self) -> Box<dyn 'a + UnaryCalculator<T>> where Self: 'a;
}

impl<T, F: Fn(T) -> T + Clone> UnaryCalculator<T> for F {
    fn clone_box<'a>(&self) -> Box<dyn 'a + UnaryCalculator<T>> where Self: 'a, {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a> Clone for Box<dyn 'a + UnaryCalculator<T>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}