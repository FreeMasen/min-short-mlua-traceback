use mlua::{Lua, LuaOptions};
fn main() {
    let stdlib = mlua::StdLib::ALL_SAFE | mlua::StdLib::DEBUG;
    let opts = LuaOptions::new().catch_rust_panics(true);
    let l = unsafe { Lua::unsafe_new_with(stdlib, opts) };
    l.load("notafunction()
        --local function level0()
        --    local function level1()
        --        local function level2()
        --            notafunction()
        --        end
        --        level2()
        --    end
        --    level1()
        --end
        --level0()
    ").exec().unwrap();
}
