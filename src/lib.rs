extern crate anymap;
use anymap::AnyMap;

pub type Global = usize;
static mut GLOBALS: Option<AnyMap> = None;
pub fn get_all<T>() -> &'static mut Vec<T> {
    unsafe {
        if GLOBALS.is_none() {
            GLOBALS = Some(AnyMap::new());
        }
        let globals = GLOBALS.as_mut().unwrap();
        let type_list = globals.get::<Vec<T>>();
        if type_list.is_none() {
            globals.insert::<Vec<T>>(Vec::new());
        }
        globals.get_mut::<Vec<T>>().unwrap()
    }
}

pub fn add<T>(item: T) -> Global  where T:'static{
    let v = get_all();
    v.push(item);
    v.len()-1
}

pub fn get<T>(id: Global) -> &'static T {
    &get_all::<T>()[id]
}

pub fn get_mut<T>(id: Global) -> &'static mut T {
    &mut get_all::<T>()[id]
}

pub fn remove<T>(id: Global) -> T  where T:'static{
    let v = get_all();
    v.remove(id)
}

pub fn singleton<T>() -> &'static T where T:std::default::Default {
    unsafe {
        if GLOBALS.is_none() {
            GLOBALS = Some(AnyMap::new());
        }
        let globals = GLOBALS.as_mut().unwrap();
        let singleton = globals.get::<T>();
        if singleton.is_none() {
            globals.insert::<T>(T::default());
        }
        globals.get::<T>().unwrap()
    }
}

pub fn singleton_mut<T>() -> &'static T where T:std::default::Default {
    unsafe {
        if GLOBALS.is_none() {
            GLOBALS = Some(AnyMap::new());
        }
        let globals = GLOBALS.as_mut().unwrap();
        let singleton = globals.get::<T>();
        if singleton.is_none() {
            globals.insert::<T>(T::default());
        }
        globals.get_mut::<T>().unwrap()
    }
}
