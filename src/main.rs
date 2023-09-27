use std::ffi::{c_char, c_void};

extern "C" {
    fn luaL_newstate() -> *mut c_void;
    fn luaL_loadstring(state: *mut c_void, s: *const c_char) -> i32;
    fn lua_close(state: *mut c_void);
    fn lua_pcall(state: *mut c_void, nargs: i32, nresults: i32, errfunc: i32) -> i32;
}

fn main() {
    unsafe {
        thread_local! {
            static TL_VAR: i32 = 0;
        }

        // Not crashing if we don't use the thread local variable
        TL_VAR.with(|_| {});

        let state = luaL_newstate();
        luaL_loadstring(state, b"error('hello')\0".as_ptr() as _);
        let ret = lua_pcall(state, 0, 0, 0);
        println!("ret = {ret}");
        lua_close(state);
    }
}

#[cfg(test)]
#[test]
fn test() {
    main();
}
