(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (func (;0;) (type 0)
    nop)
  (func (;1;) (type 1) (result i32)
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
  (table (;0;) 1 1 funcref)
  (memory (;0;) 256 256)
  (export "a" (memory 0))
  (export "b" (func 0))
  (export "c" (func 1))
  (export "d" (table 0))
  (data (;0;) (i32.const 1024) "\ff\00\00\ff"))
