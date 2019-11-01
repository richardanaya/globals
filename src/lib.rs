#[macro_use]
extern crate lazy_static;
use std::any::TypeId;
use std::sync::Mutex;
use std::collections::HashMap;
use std::any::Any;

lazy_static! {
    static ref GLOBALS:Mutex<HashMap<TypeId, Box<dyn Any+Send>>> = {
        Mutex::new(HashMap::new())
    };
}

pub fn get<T>() -> &'static T where T:'static+Default+Send{
    {
        let id = TypeId::of::<T>();
        let mut globals = GLOBALS.lock().unwrap();
        if let Some(v) = globals.get(&id) {
            return unsafe { &*(v as *const Box<dyn Any+Send> as *const Box<T>) };
        }
        let v = T::default();
        globals.insert(id,Box::new(v));
    }
    get::<T>()
}