# Output

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/min-short-mlua-traceback`
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
