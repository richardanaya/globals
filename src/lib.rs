#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::linked_list::LinkedList;
use core::any::Any;
use core::any::TypeId;
use spin::{Mutex, MutexGuard};

static GLOBALS_LIST: Mutex<LinkedList<(TypeId, &'static Mutex<dyn Any + Send + Sync>)>> =
    Mutex::new(LinkedList::new());

/// Get a mutex gaurd handle to globle singleton
pub fn get<T>() -> MutexGuard<'static, T>
where
    T: 'static + Default + Send + core::marker::Sync,
{
    {
        let mut globals = GLOBALS_LIST.lock();
        let id = TypeId::of::<T>();
        let p = globals.iter().find(|&r| r.0 == id);
        if let Some(v) = p {
            let m = unsafe { &*(v.1 as *const Mutex<dyn Any + Send + Sync> as *const Mutex<T>) };
            return m.lock();
        }
        let v = Box::new(Mutex::new(T::default()));
        let handle = Box::leak(v);
        globals.push_front((id, handle));
    }
    get()
}

#[cfg(test)]
mod test {
    use crate::*;
    use alloc::vec::Vec;

    #[derive(Default)]
    struct Foo {
        x: u32,
    }

    #[derive(Default)]
    struct Bar {
        x: u32,
    }

    #[test]
    fn basic() {
        let f = get::<Foo>();
        let b = get::<Bar>();
        assert_eq!(0, f.x);
        assert_eq!(0, b.x);
    }

    #[test]
    fn basic_2() {
        {
            let mut f = get::<Vec<u32>>();
            let _b = get::<Vec<u8>>();
            f.resize(2_000_000, 0);
        }
        {
            let mut f = get::<Vec<u32>>();
            assert_eq!(2_000_000, f.len());
            f.resize(100, 0);
        }
        {
            let f = get::<Vec<u32>>();
            assert_eq!(100, f.len());
        }
    }

    #[test]
    fn basic3() {
        get::<Foo>().x = 100;
        let f = get::<Foo>();
        assert_eq!(100, f.x);
    }
}
