use {types, Type, Error};

use std::io::prelude::*;
use std;

pub type SizeType = u32;

impl<T: Type> Type for Vec<T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(Array::<SizeType, T>::read(read)?.elements)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        Array::<SizeType, T>::new(self.clone()).write(write)
    }
}

/// An array type with a custom size prefix type.
#[derive(Clone, Debug)]
pub struct Array<S: types::Integer, T: Type>
{
    pub elements: Vec<T>,
    _a: std::marker::PhantomData<S>,
}

impl<S: types::Integer, T: Type> Array<S,T>
{
    pub fn new(elements: Vec<T>) -> Self {
        Array { elements: elements, _a: std::marker::PhantomData }
    }
}

impl<S: types::Integer, T: Type> Type for Array<S, T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let size = S::read(read)?;
        let size: usize = size.try_into()?;

        let mut elements = Vec::new();

        for _ in 0..size {
            elements.push(T::read(read)?);
        }

        Ok(Self::new(elements))
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        S::try_from(self.elements.len())?.write(write)?;

        for element in self.elements.iter() {
            element.write(write)?;
        }

        Ok(())
    }
}

