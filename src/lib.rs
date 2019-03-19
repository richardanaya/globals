extern crate anymap;
use anymap::AnyMap;

pub type Global = usize;
static mut GLOBALS: Option<AnyMap> = None;
pub unsafe fn get_all<T>() -> &'static mut Vec<T> {
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

pub unsafe fn add<T>(item: T) -> Global
where
    T: 'static,
{
    let v = get_all();
    v.push(item);
    v.len() - 1
}

pub unsafe fn get<T>(id: Global) -> &'static T {
    &get_all::<T>()[id]
}

pub unsafe fn get_mut<T>(id: Global) -> &'static mut T {
    &mut get_all::<T>()[id]
}

pub unsafe fn remove<T>(id: Global) -> T
where
    T: 'static,
{
    let v = get_all();
    v.remove(id)
}

pub unsafe fn singleton<T>() -> &'static T
where
    T: std::default::Default,
{
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

pub unsafe fn singleton_mut<T>() -> &'static mut T
where
    T: std::default::Default,
{
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
