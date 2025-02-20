pub trait Poolable {
    fn new() -> Self;
    fn is_free(&self) -> bool;
}

pub struct Pool<T, const N: usize> {
    pub items: [Option<T>; N],
}

impl<T: Poolable, const N: usize> Pool<T, N> {
    pub fn new() -> Self {
        Self { items: [const { None }; N] }
    }

    pub fn get(&mut self) -> Option<&mut T> {
        for item in self.items.iter_mut() {
            if let Some(item) = item {
                if item.is_free() {
                    return Some(item);
                }
            } else {
                *item = Some(T::new());
                return item.as_mut();
            }
        }

        None
    }
}
