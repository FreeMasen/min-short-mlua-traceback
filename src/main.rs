use mlua::{Lua, LuaOptions};
fn main() {
    let stdlib = mlua::StdLib::ALL_SAFE | mlua::StdLib::DEBUG;
    let opts = LuaOptions::new().catch_rust_panics(true);
    let l = unsafe { Lua::unsafe_new_with(stdlib, opts) };
    let user_threads_table = l.create_table().unwrap();
    l.set_named_registry_value("user_threads", user_threads_table).unwrap();
    let user_func: mlua::Function = l
            .load("notafunction()")
            .set_name("init.lua").unwrap()
            .into_function().unwrap();


    let user_thread = l.create_thread(user_func).unwrap();

    let user_threads_table = l.named_registry_value::<_, mlua::Table>("user_threads").unwrap();
    let new_idx = user_threads_table.raw_len() + 1;
    user_threads_table.raw_set(new_idx, user_thread).unwrap();

    loop {
        let thread: mlua::Thread = user_threads_table.raw_get(new_idx).unwrap();
        dbg!(thread.resume::<_, mlua::Value>(()).unwrap());
    }

}
