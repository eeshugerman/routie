
#[macro_use]
pub mod seq_indexed_store {
    use std::{
        iter::{Enumerate, Map},
        marker::PhantomData,
    };
    pub struct SeqIndexedStore<U, T> {
        index_type: PhantomData<U>,
        data: Vec<T>,
    }

    impl<U: From<usize> + Into<usize>, T> SeqIndexedStore<U, T> {
        pub fn new() -> Self {
            Self {
                index_type: PhantomData,
                data: Vec::new(),
            }
        }
        pub fn push(&mut self, val: T) -> U {
            let id = self.data.len();
            self.data.push(val);
            U::from(id)
        }
        pub fn get(&self, id: U) -> Option<&T> {
            self.data.get(id.into())
        }
        pub fn get_mut(&mut self, id: U) -> Option<&mut T> {
            self.data.get_mut(id.into())
        }
        pub fn len(&self) -> usize {
            self.data.len()
        }
        // TODO: implement IntoIter instead
        pub fn enumerate(&self) -> Map<Enumerate<std::slice::Iter<'_, T>>, fn((usize, &T)) -> (U, &T)> {
            self.data
                .iter()
                .enumerate()
                .map(|val| (U::from(val.0), &val.1))
        }
    }

    macro_rules! define_index_type {
        ($name:ident) => {
            // TODO: can we do something so that values don't need to be referenced? remove Clone?
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
        };
    }
}
