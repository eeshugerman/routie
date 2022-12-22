#[macro_use]
pub mod seq_indexed_store {
    use std::marker::PhantomData;
    pub struct SeqIndexedStore<U, T> {
        index_type: PhantomData<U>,
        data: Vec<T>,
    }

    impl<U: From<usize> + Into<usize> + Copy, T> SeqIndexedStore<U, T> {
        pub fn new() -> Self {
            Self { index_type: PhantomData, data: Vec::new() }
        }
        pub fn push(&mut self, val: T) -> U {
            let id = self.data.len();
            self.data.push(val);
            U::from(id)
        }
        pub fn get(&self, id: &U) -> Option<&T> {
            self.data.get(Into::<usize>::into(*id))
        }
        pub fn get_mut(&mut self, id: &U) -> Option<&mut T> {
            self.data.get_mut(Into::<usize>::into(*id))
        }
        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn enumerate(&self) -> impl Iterator<Item = (U, &T)> {
            self.data.iter().enumerate().map(|val| (U::from(val.0), val.1))
        }

        pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (U, &mut T)> {
            self.data.iter_mut().enumerate().map(|val| (U::from(val.0), val.1))
        }
    }

    macro_rules! define_index_type {
        ($name:ident) => {
            #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
            pub struct $name(usize);
            impl From<usize> for $name {
                fn from(id: usize) -> $name {
                    $name(id)
                }
            }
            impl From<$name> for usize {
                fn from(id: $name) -> usize {
                    id.0
                }
            }
            impl From<$name> for u32 {
                fn from(id: $name) -> u32 {
                    id.0 as u32
                }
            }
            impl From<$name> for i32 {
                fn from(id: $name) -> i32 {
                    id.0 as i32
                }
            }
        };
    }
}

pub mod ordered_skip_map {
    use std::{cmp::Ordering, ops::Bound};

    use skiplist::OrderedSkipList;

    pub struct OrderedSkipMap<K, V> {
        null_value_builder: fn() -> V,
        data: OrderedSkipList<(K, V)>,
    }

    impl<K: Copy + PartialOrd, V> OrderedSkipMap<K, V> {
        pub fn new(null_value_builder: fn() -> V) -> Self {
            Self {
                null_value_builder,
                data: unsafe {
                    OrderedSkipList::with_comp(|a: &(K, V), b: &(K, V)| {
                        if a.0 < b.0 {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                },
            }
        }
        pub fn insert(&mut self, key: K, value: V) {
            self.data.insert((key, value));
        }

        // TODO: is this going to work?
        pub fn remove(&mut self, key: K) -> Option<V> {
            match self.data.remove(&(key, (self.null_value_builder)())) {
                Some((_, val)) => Some(val),
                None => None
            }
        }
        pub fn enumerate_range(&self, min: K, max: K) -> impl Iterator<Item = &(K, V)>{
            self.data.range(
                Bound::Included(&(min, (self.null_value_builder)())),
                Bound::Included(&(max, (self.null_value_builder)())),
            )
        }
        pub fn enumerate(&self) -> impl Iterator<Item = &(K, V)> {
            self.data.range(Bound::Unbounded, Bound::Unbounded)
        }
    }
}
