pub trait OperationExecutor<T>: Fn(Vec<T>) -> T {
    fn clone_box<'a>(&self) -> Box<dyn 'a + OperationExecutor<T>> where Self: 'a;
}

impl<T, F: Fn(Vec<T>) -> T + Clone> OperationExecutor<T> for F {
    fn clone_box<'a>(&self) -> Box<dyn 'a + OperationExecutor<T>> where Self: 'a, {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a> Clone for Box<dyn 'a + OperationExecutor<T>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}


pub trait BinaryOperationExecutor<T>: Fn(T, T) -> T {
    fn clone_box<'a>(&self) -> Box<dyn 'a + BinaryOperationExecutor<T>> where Self: 'a;
}

impl<T, F: Fn(T, T) -> T + Clone> BinaryOperationExecutor<T> for F {
    fn clone_box<'a>(&self) -> Box<dyn 'a + BinaryOperationExecutor<T>> where Self: 'a, {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a> Clone for Box<dyn 'a + BinaryOperationExecutor<T>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}


pub trait UnaryoperationExecutor<T>: Fn(T) -> T {
    fn clone_box<'a>(&self) -> Box<dyn 'a + UnaryoperationExecutor<T>> where Self: 'a;
}

impl<T, F: Fn(T) -> T + Clone> UnaryoperationExecutor<T> for F {
    fn clone_box<'a>(&self) -> Box<dyn 'a + UnaryoperationExecutor<T>> where Self: 'a, {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a> Clone for Box<dyn 'a + UnaryoperationExecutor<T>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}