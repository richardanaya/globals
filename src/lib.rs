#![no_std]
#[macro_use]
extern crate lazy_static;
extern crate alloc;
use alloc::boxed::Box;
use core::any::TypeId;
use spin::Mutex;
use core::any::Any;
use hashbrown::HashMap;


lazy_static! {
    static ref GLOBALS:Mutex<HashMap<TypeId, Mutex<Box<dyn Any+Send>>>> = {
        Mutex::new(HashMap::new())
    };
}

pub fn get<T>() -> &'static Mutex<Box<T>> where T:'static+Default+Send{
    {
        let id = TypeId::of::<T>();
        let mut globals = GLOBALS.lock();
        if let Some(v) = globals.get(&id) {
            let m = unsafe { &*(v as *const Mutex<Box<dyn Any+Send>> as *const Mutex<Box<T>>) };
            return m;
        }
        let v = T::default();
        globals.insert(id,Mutex::new(Box::new(v)));
    }
    get::<T>()
}


#[derive(Debug)]
struct Foo {
    v:u32
}

impl Default for Foo{
    fn default() -> Self {
        Foo{v:42}
    }
}

#[test]
fn simple(){
    {
        let mut f = get::<Foo>().lock();
        assert_eq!(42,f.v);
        f.v = 50;
        assert_eq!(50,f.v);
    }
    let f = get::<Foo>().lock();
    assert_eq!(50,f.v);
}