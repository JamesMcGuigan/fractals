(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32) (result i32)))
  (func (;0;) (type 1)
    nop)
  (func (;1;) (type 0) (result i32)
    (local i32 i32 i32 i32)
    i32.const 1024
    i32.load
    local.set 2
    loop  ;; label = @1
      local.get 0
      i32.const 800
      i32.mul
      local.set 3
      i32.const 0
      local.set 1
      loop  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        i32.const 2
        i32.shl
        i32.const 1040
        i32.add
        local.get 2
        i32.store
        local.get 1
        i32.const 1
        i32.add
        local.tee 1
        i32.const 800
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 0
      i32.const 1
      i32.add
      local.tee 0
      i32.const 400
      i32.ne
      br_if 0 (;@1;)
    end
    i32.const 1040)
  (func (;2;) (type 0) (result i32)
    global.get 0)
  (func (;3;) (type 2) (param i32)
    local.get 0
    global.set 0)
  (func (;4;) (type 3) (param i32) (result i32)
    global.get 0
    local.get 0
    i32.sub
    i32.const -16
    i32.and
    local.tee 0
    global.set 0
    local.get 0)
  (func (;5;) (type 0) (result i32)
    i32.const 1281040)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 256 256)
  (global (;0;) (mut i32) (i32.const 6523936))
  (export "memory" (memory 0))
  (export "__wasm_call_ctors" (func 0))
  (export "render" (func 1))
  (export "__indirect_function_table" (table 0))
  (export "__errno_location" (func 5))
  (export "stackSave" (func 2))
  (export "stackRestore" (func 3))
  (export "stackAlloc" (func 4))
  (data (;0;) (i32.const 1024) "\ff\00\00\ff"))
