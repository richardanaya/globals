#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::linked_list::LinkedList;
use core::any::Any;
use core::any::TypeId;
use spin::Mutex;

static mut GLOBALS_LIST: Option<
    Mutex<LinkedList<(TypeId, &'static Mutex<dyn Any + Send + Sync>)>>,
> = None;

pub fn get<T>() -> &'static Mutex<T>
where
    T: 'static + Default + Send + core::marker::Sync,
{
    {
        let mut globals = unsafe {
            if GLOBALS_LIST.is_none() {
                GLOBALS_LIST = Some(Mutex::new(LinkedList::new()));
            }
            GLOBALS_LIST.as_ref().unwrap().lock()
        };
        let id = TypeId::of::<T>();
        let p = globals.iter().find(|&r| r.0 == id);
        if let Some(v) = p {
            return unsafe { &*(v.1 as *const Mutex<dyn Any + Send + Sync> as *const Mutex<T>) };
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
        let f = get::<Foo>().lock();
        let b = get::<Bar>().lock();
        assert_eq!(0, f.x);
        assert_eq!(0, b.x);
    }

    #[test]
    fn basic_2() {
        {
            let mut f = get::<Vec<u32>>().lock();
            f.resize(2_000_000, 0);
        }
        {
            let mut f = get::<Vec<u32>>().lock();
            assert_eq!(2_000_000, f.len());
            f.resize(100, 0);
        }
        {
            let f = get::<Vec<u32>>().lock();
            assert_eq!(100, f.len());
        }
    }
}
