(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param f64)))
  (type (;4;) (func (param f64) (result f64)))
  (type (;5;) (func (param i32)))
  (type (;6;) (func (param i32) (result i32)))
  (func (;0;) (type 1)
    nop)
  (func (;1;) (type 2) (param i32 i32) (result i32)
    i32.const 1028
    local.get 1
    i32.store
    i32.const 1024
    local.get 0
    i32.store
    i32.const 1032
    local.get 0
    local.get 1
    i32.mul
    i32.store
    i32.const 1036
    local.get 1
    i32.const 1
    i32.shr_s
    local.tee 1
    i32.store
    i32.const 1040
    local.get 0
    i32.const 1
    i32.shr_s
    local.tee 0
    i32.store
    i32.const 1048
    local.get 1
    local.get 1
    i32.mul
    local.get 0
    local.get 0
    i32.mul
    i32.add
    f64.convert_i32_s
    f64.sqrt
    f64.store
    i32.const 1056)
  (func (;2;) (type 3) (param f64)
    (local f64 f64 f64 f64 f64 f64 f64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 1028
    i32.load
    local.tee 13
    i32.const 0
    i32.gt_s
    if  ;; label = @1
      block (result i32)  ;; label = @2
        local.get 0
        f64.const 0x1.4p+3 (;=10;)
        f64.div
        f64.const 0x1.f4p+10 (;=2000;)
        f64.add
        f64.floor
        local.tee 0
        f64.abs
        f64.const 0x1p+31 (;=2.14748e+09;)
        f64.lt
        if  ;; label = @3
          local.get 0
          i32.trunc_f64_s
          br 1 (;@2;)
        end
        i32.const -2147483648
      end
      f64.convert_i32_s
      local.set 5
      i32.const 1048
      f64.load
      local.set 6
      i32.const 1040
      i32.load
      local.set 14
      i32.const 1036
      i32.load
      local.set 15
      i32.const 1024
      i32.load
      local.tee 12
      i32.const 0
      i32.le_s
      local.set 16
      loop  ;; label = @2
        local.get 16
        i32.eqz
        if  ;; label = @3
          local.get 8
          local.get 12
          i32.mul
          local.set 17
          local.get 15
          local.get 8
          i32.sub
          local.tee 11
          local.get 11
          i32.mul
          local.set 18
          local.get 11
          f64.convert_i32_s
          local.set 2
          i32.const 0
          local.set 9
          loop  ;; label = @4
            local.get 14
            local.get 9
            i32.sub
            local.tee 10
            local.get 10
            i32.const 31
            i32.shr_s
            local.tee 19
            i32.xor
            local.get 19
            i32.sub
            f64.convert_i32_s
            f64.const 0x1.b7cdfd9d7bdbbp-34 (;=1e-10;)
            f64.add
            local.set 0
            local.get 9
            local.get 17
            i32.add
            i32.const 2
            i32.shl
            i32.const 1056
            i32.add
            block (result i32)  ;; label = @5
              f64.const 0x1p+0 (;=1;)
              local.get 10
              local.get 10
              i32.mul
              local.get 18
              i32.add
              f64.convert_i32_s
              local.tee 1
              f64.sqrt
              local.tee 7
              local.get 6
              f64.div
              f64.sub
              local.tee 3
              block (result f64)  ;; label = @6
                local.get 11
                i32.const 0
                i32.ge_s
                if  ;; label = @7
                  f64.const 0x1.921fb54442d18p-1 (;=0.785398;)
                  local.set 4
                  local.get 2
                  local.get 0
                  f64.sub
                  local.get 0
                  local.get 2
                  f64.add
                  f64.div
                  br 1 (;@6;)
                end
                f64.const 0x1.2d97c7f3321d2p+1 (;=2.35619;)
                local.set 4
                local.get 0
                local.get 2
                f64.add
                local.get 0
                local.get 2
                f64.sub
                f64.div
              end
              local.tee 0
              local.get 0
              f64.const 0x1.9205bc01a36e3p-3 (;=0.1963;)
              f64.mul
              f64.mul
              local.get 0
              f64.mul
              local.get 0
              f64.const -0x1.f6a161e4f766p-1 (;=-0.9817;)
              f64.mul
              f64.add
              local.get 4
              f64.add
              local.tee 0
              f64.neg
              local.get 0
              local.get 10
              i32.const 0
              i32.lt_s
              select
              f64.const 0x1.921fb54442d18p+2 (;=6.28319;)
              f64.div
              f64.const 0x1.2cp+8 (;=300;)
              f64.mul
              local.get 1
              f64.const 0x1.9p+8 (;=400;)
              f64.div
              local.get 7
              f64.add
              local.get 5
              f64.sub
              f64.add
              f64.abs
              local.tee 0
              f64.const 0x1.9p+6 (;=100;)
              f64.div
              f64.floor
              f64.const -0x1.9p+6 (;=-100;)
              f64.mul
              local.get 0
              f64.add
              f64.const -0x1.9p+6 (;=-100;)
              f64.div
              f64.const 0x1p+0 (;=1;)
              f64.add
              local.tee 0
              local.get 0
              local.get 0
              f64.const 0x1.ep+6 (;=120;)
              f64.mul
              f64.mul
              f64.mul
              f64.mul
              call 3
              local.tee 1
              f64.abs
              f64.const 0x1p+31 (;=2.14748e+09;)
              f64.lt
              if  ;; label = @6
                local.get 1
                i32.trunc_f64_s
                br 1 (;@5;)
              end
              i32.const -2147483648
            end
            i32.const 8
            i32.shl
            block (result i32)  ;; label = @5
              local.get 0
              f64.const 0x1p+0 (;=1;)
              f64.add
              local.get 3
              f64.const 0x1.999999999999ap-1 (;=0.8;)
              f64.mul
              f64.const 0x1.999999999999ap-3 (;=0.2;)
              f64.add
              local.get 0
              f64.const 0x1.ep+7 (;=240;)
              f64.mul
              f64.mul
              f64.mul
              f64.const 0x1p-1 (;=0.5;)
              f64.mul
              call 3
              local.tee 1
              f64.abs
              f64.const 0x1p+31 (;=2.14748e+09;)
              f64.lt
              if  ;; label = @6
                local.get 1
                i32.trunc_f64_s
                br 1 (;@5;)
              end
              i32.const -2147483648
            end
            block (result i32)  ;; label = @5
              local.get 3
              local.get 0
              f64.const 0x1.9p+5 (;=50;)
              f64.mul
              f64.mul
              call 3
              local.tee 0
              f64.abs
              f64.const 0x1p+31 (;=2.14748e+09;)
              f64.lt
              if  ;; label = @6
                local.get 0
                i32.trunc_f64_s
                br 1 (;@5;)
              end
              i32.const -2147483648
            end
            i32.const 16
            i32.shl
            i32.or
            i32.or
            i32.const -16777216
            i32.or
            i32.store
            local.get 9
            i32.const 1
            i32.add
            local.tee 9
            local.get 12
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 8
        i32.const 1
        i32.add
        local.tee 8
        local.get 13
        i32.ne
        br_if 0 (;@2;)
      end
    end)
  (func (;3;) (type 4) (param f64) (result f64)
    (local f64 i64 i32)
    local.get 0
    i64.reinterpret_f64
    local.tee 2
    i64.const 52
    i64.shr_u
    i32.wrap_i64
    i32.const 2047
    i32.and
    local.tee 3
    i32.const 1074
    i32.le_u
    if (result f64)  ;; label = @1
      local.get 3
      i32.const 1021
      i32.le_u
      if  ;; label = @2
        local.get 0
        f64.const 0x0p+0 (;=0;)
        f64.mul
        return
      end
      block (result f64)  ;; label = @2
        local.get 0
        local.get 0
        f64.neg
        local.get 2
        i64.const 0
        i64.ge_s
        select
        local.tee 0
        f64.const 0x1p+52 (;=4.5036e+15;)
        f64.add
        f64.const -0x1p+52 (;=-4.5036e+15;)
        f64.add
        local.get 0
        f64.sub
        local.tee 1
        f64.const 0x1p-1 (;=0.5;)
        f64.gt
        if  ;; label = @3
          local.get 0
          local.get 1
          f64.add
          f64.const -0x1p+0 (;=-1;)
          f64.add
          br 1 (;@2;)
        end
        local.get 0
        local.get 1
        f64.add
        local.tee 0
        local.get 1
        f64.const -0x1p-1 (;=-0.5;)
        f64.le
        i32.eqz
        br_if 0 (;@2;)
        drop
        local.get 0
        f64.const 0x1p+0 (;=1;)
        f64.add
      end
      local.tee 0
      local.get 0
      f64.neg
      local.get 2
      i64.const 0
      i64.ge_s
      select
    else
      local.get 0
    end)
  (func (;4;) (type 0) (result i32)
    global.get 0)
  (func (;5;) (type 5) (param i32)
    local.get 0
    global.set 0)
  (func (;6;) (type 6) (param i32) (result i32)
    global.get 0
    local.get 0
    i32.sub
    i32.const -16
    i32.and
    local.tee 0
    global.set 0
    local.get 0)
  (func (;7;) (type 0) (result i32)
    i32.const 8001056)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 256 256)
  (global (;0;) (mut i32) (i32.const 13243952))
  (export "memory" (memory 0))
  (export "__wasm_call_ctors" (func 0))
  (export "init" (func 1))
  (export "render" (func 2))
  (export "__indirect_function_table" (table 0))
  (export "__errno_location" (func 7))
  (export "stackSave" (func 4))
  (export "stackRestore" (func 5))
  (export "stackAlloc" (func 6)))
