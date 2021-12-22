# Output
```lua
function x()
    notafunction()
end
x()
```

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/min-short-mlua-traceback 'function x() notafunction() end; x()'`
Without thread:
[src/main.rs:16] lua.load(script).set_name("init.lua").unwrap().exec() = Err(
    RuntimeError(
        "[string \"init.lua\"]:1: attempt to call a nil value (global 'notafunction')\nstack traceback:\n\t[string \"init.lua\"]:1: in function 'x'\n\t[string \"init.lua\"]:1: in main chunk",
    ),
)
With thread:
[src/main.rs:29] thread.resume::<_, mlua::Value>(()) = Err(
    RuntimeError(
        "[string \"init.lua\"]:1: attempt to call a nil value (global 'notafunction')\nstack traceback:\n\t[string \"init.lua\"]:1: in main chunk",
    ),
)
stack traceback:
	[string "init.lua"]:1: in function 'x'
	[string "init.lua"]:1: in main chunk
```

```lua
notafunction()
```

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/min-short-mlua-traceback 'notafunction()'`
Without thread:
[src/main.rs:16] lua.load(script).set_name("init.lua").unwrap().exec() = Err(
    RuntimeError(
        "[string \"init.lua\"]:1: attempt to call a nil value (global 'notafunction')\nstack traceback:\n\t[string \"init.lua\"]:1: in main chunk",
    ),
)
With thread:
[src/main.rs:29] thread.resume::<_, mlua::Value>(()) = Err(
    RuntimeError(
        "[string \"init.lua\"]:1: attempt to call a nil value (global 'notafunction')\nstack traceback:",
    ),
)
stack traceback:
	[string "init.lua"]:1: in main chunk
```
