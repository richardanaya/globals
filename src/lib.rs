#![no_std]
#[macro_use]
extern crate lazy_static;
extern crate alloc;
use alloc::boxed::Box;
use core::any::Any;
use core::any::TypeId;
use hashbrown::HashMap;
use spin::Mutex;

lazy_static! {
    static ref GLOBALS: Mutex<HashMap<TypeId, Mutex<Box<dyn Any + Send>>>> =
        { Mutex::new(HashMap::new()) };
}

pub fn get<T>() -> &'static Mutex<Box<T>>
where
    T: 'static + Default + Send,
{
    {
        let id = TypeId::of::<T>();
        let mut globals = GLOBALS.lock();
        if let Some(v) = globals.get(&id) {
            let m = unsafe { &*(v as *const Mutex<Box<dyn Any + Send>> as *const Mutex<Box<T>>) };
            return m;
        }
        let v = T::default();
        globals.insert(id, Mutex::new(Box::new(v)));
    }
    get::<T>()
}
