use mlua::{Lua, LuaOptions};

fn main() {
    let script = std::env::args().nth(1).unwrap();

    let stdlib = mlua::StdLib::ALL_SAFE | mlua::StdLib::DEBUG;
    let opts = LuaOptions::new().catch_rust_panics(true);
    let mut lua = unsafe { Lua::unsafe_new_with(stdlib, opts) };
    println!("Without thread:");
    run_without_thread(&mut lua, &script);
    println!("With thread:");
    run_with_thread(&mut lua, &script);
}

fn run_without_thread(lua: &mut Lua, script: &str) {
    dbg!(lua.load(script).set_name("init.lua").unwrap().exec()).ok();
}

fn run_with_thread(lua: &mut Lua, script: &str) {
    
    let user_func: mlua::Function = lua
            .load(script)
            .set_name("init.lua").unwrap()
            .into_function().unwrap();


    let thread = lua.create_thread(user_func).unwrap();

    dbg!(thread.resume::<_, mlua::Value>(())).ok();
    let g = lua.globals();
    let debug = g.get::<_, mlua::Table>("debug").unwrap();
    let traceback = debug.get::<_, mlua::Function>("traceback").unwrap();

    eprintln!("{}", traceback.call::<_, String>(thread).unwrap());
}

