(module $basic-530df7d62946cc87.wasm
  (type (;0;) (func (param i32 i32) (result externref)))
  (type (;1;) (func (param externref)))
  (type (;2;) (func))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func (param i32)))
  (type (;5;) (func (result i32)))
  (type (;6;) (func (param i32 i32) (result i32)))
  (type (;7;) (func (param i32) (result i32)))
  (type (;8;) (func (param externref) (result i32)))
  (type (;9;) (func (param i32) (result externref)))
  (import "js_bindgen" "memory" (memory (;0;) 17))
  (import "js_sys" "string_decode" (func $js_sys.import.string_decode (;0;) (type 0)))
  (import "js_sys" "externref.table" (table (;0;) 0 externref))
  (import "web_sys" "console.log" (func $web_sys.import.console.log (;1;) (type 1)))
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048624)
  (global (;2;) i32 i32.const 1048616)
  (export "foo" (func $foo))
  (export "__heap_base" (global 1))
  (export "js_sys.externref.next" (func $js_sys.externref.next))
  (export "__data_end" (global 2))
  (start $__wasm_init_memory)
  (func $__wasm_init_memory (;2;) (type 2)
    i32.const 1048596
    i32.const 0
    i32.const 20
    memory.fill
  )
  (func $foo (;3;) (type 2)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    i32.const 1048576
    i32.const 13
    call $js_sys.string_decode
    local.tee 1
    call $web_sys.console.log
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 1
          i32.lt_s
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1048596
          br_if 2 (;@1;)
          i32.const 0
          i32.const -1
          i32.store offset=1048596
          i32.const 0
          i32.load offset=1048592
          local.set 2
          block ;; label = @4
            i32.const 0
            i32.load offset=1048600
            i32.const 0
            i32.load offset=1048604
            local.tee 3
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 4
            i32.add
            local.get 3
            local.get 2
            local.get 3
            i32.const 1
            i32.shl
            local.tee 4
            i32.const 4
            local.get 4
            i32.const 4
            i32.gt_u
            select
            local.tee 4
            call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h4cf4f56d2c6e3149E
            local.get 0
            i32.load offset=4
            br_if 2 (;@2;)
            i32.const 0
            local.get 0
            i32.load offset=8
            local.tee 2
            i32.store offset=1048592
            i32.const 0
            local.get 4
            i32.store offset=1048600
          end
          i32.const 0
          local.get 3
          i32.const 1
          i32.add
          i32.store offset=1048604
          local.get 2
          local.get 3
          i32.const 2
          i32.shl
          i32.add
          local.get 1
          i32.store
          local.get 1
          call $js_sys.externref.remove
          i32.const 0
          i32.const 0
          i32.load offset=1048596
          i32.const 1
          i32.add
          i32.store offset=1048596
        end
        local.get 0
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 22
      call $_ZN4core6result13unwrap_failed17hf1b7344c4a305aa0E
      unreachable
    end
    i32.const 43
    call $_ZN4core6result13unwrap_failed17hf1b7344c4a305aa0E
    unreachable
  )
  (func $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h4cf4f56d2c6e3149E (;4;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 1073741823
          i32.gt_u
          br_if 0 (;@3;)
          local.get 3
          i32.const 2
          i32.shl
          local.tee 3
          i32.const 2147483645
          i32.lt_u
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 0
        i32.store offset=4
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 1
                      i32.eqz
                      br_if 0 (;@9;)
                      i32.const 0
                      i32.load offset=1048608
                      local.tee 4
                      i32.eqz
                      br_if 1 (;@8;)
                      br 4 (;@5;)
                    end
                    i32.const 0
                    i32.load offset=1048608
                    local.tee 1
                    i32.eqz
                    br_if 1 (;@7;)
                    br 2 (;@6;)
                  end
                  memory.size
                  local.set 5
                  i32.const 0
                  i32.const 0
                  i32.const 1048624
                  i32.sub
                  local.tee 4
                  i32.store offset=1048608
                  i32.const 0
                  i32.const 1
                  local.get 5
                  i32.const 16
                  i32.shl
                  i32.sub
                  i32.store offset=1048612
                  br 2 (;@5;)
                end
                memory.size
                local.set 2
                i32.const 0
                i32.const 0
                i32.const 1048624
                i32.sub
                local.tee 1
                i32.store offset=1048608
                i32.const 0
                i32.const 1
                local.get 2
                i32.const 16
                i32.shl
                i32.sub
                i32.store offset=1048612
              end
              local.get 1
              i32.const -4
              i32.and
              local.tee 4
              local.get 3
              i32.lt_u
              br_if 2 (;@3;)
              block ;; label = @6
                i32.const 0
                i32.load offset=1048612
                local.tee 2
                local.get 4
                local.get 3
                i32.sub
                local.tee 1
                i32.const 1
                i32.or
                i32.le_u
                br_if 0 (;@6;)
                local.get 2
                local.get 1
                i32.sub
                i32.const -2
                i32.add
                i32.const 16
                i32.shr_u
                i32.const 1
                i32.add
                local.tee 5
                memory.grow
                i32.const -1
                i32.eq
                br_if 3 (;@3;)
                i32.const 0
                local.get 2
                local.get 5
                i32.const 16
                i32.shl
                i32.sub
                i32.store offset=1048612
              end
              i32.const 0
              local.get 1
              i32.store offset=1048608
              local.get 0
              i32.const 8
              i32.add
              local.set 1
              local.get 0
              i32.const 4
              i32.add
              local.set 2
              local.get 4
              i32.eqz
              br_if 3 (;@2;)
              i32.const 0
              local.get 4
              i32.sub
              local.set 4
              br 1 (;@4;)
            end
            local.get 4
            i32.const -4
            i32.and
            local.tee 4
            local.get 3
            i32.lt_u
            br_if 1 (;@3;)
            block ;; label = @5
              i32.const 0
              i32.load offset=1048612
              local.tee 6
              local.get 4
              local.get 3
              i32.sub
              local.tee 5
              i32.const 1
              i32.or
              i32.le_u
              br_if 0 (;@5;)
              local.get 6
              local.get 5
              i32.sub
              i32.const -2
              i32.add
              i32.const 16
              i32.shr_u
              i32.const 1
              i32.add
              local.tee 7
              memory.grow
              i32.const -1
              i32.eq
              br_if 2 (;@3;)
              i32.const 0
              local.get 6
              local.get 7
              i32.const 16
              i32.shl
              i32.sub
              i32.store offset=1048612
            end
            i32.const 0
            local.get 5
            i32.store offset=1048608
            local.get 4
            i32.eqz
            br_if 1 (;@3;)
            i32.const 0
            local.get 4
            i32.sub
            local.set 4
            block ;; label = @5
              local.get 1
              i32.const 2
              i32.shl
              local.tee 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              local.get 2
              local.get 1
              memory.copy
            end
            local.get 0
            i32.const 8
            i32.add
            local.set 1
            local.get 0
            i32.const 4
            i32.add
            local.set 2
          end
          local.get 1
          local.get 3
          i32.store
          local.get 2
          local.get 4
          i32.store
          local.get 0
          i32.const 0
          i32.store
          return
        end
        local.get 0
        i32.const 8
        i32.add
        local.set 1
        local.get 0
        i32.const 4
        i32.add
        local.set 2
      end
      local.get 1
      local.get 3
      i32.store
      local.get 2
      i32.const 4
      i32.store
    end
    local.get 0
    i32.const 1
    i32.store
  )
  (func $_ZN4core6result13unwrap_failed17hf1b7344c4a305aa0E (;5;) (type 4) (param i32)
    call $_ZN4core9panicking9panic_fmt17hcb6b2b4be1f4be38E
    unreachable
  )
  (func $_ZN4core9panicking9panic_fmt17hcb6b2b4be1f4be38E (;6;) (type 2)
    unreachable
  )
  (func $js_sys.externref.next (;7;) (type 5) (result i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load offset=1048596
          br_if 0 (;@3;)
          i32.const 0
          i32.const -1
          i32.store offset=1048596
          i32.const 0
          i32.load offset=1048604
          local.tee 0
          br_if 1 (;@2;)
          block ;; label = @4
            i32.const 1
            call $js_sys.externref.grow
            local.tee 0
            i32.const -1
            i32.eq
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1048596
            i32.const 1
            i32.add
            local.set 1
            br 3 (;@1;)
          end
          unreachable
        end
        i32.const 43
        call $_ZN4core6result13unwrap_failed17hf1b7344c4a305aa0E
        unreachable
      end
      i32.const 0
      local.set 1
      i32.const 0
      local.get 0
      i32.const -1
      i32.add
      local.tee 0
      i32.store offset=1048604
      i32.const 0
      i32.load offset=1048592
      local.get 0
      i32.const 2
      i32.shl
      i32.add
      i32.load
      local.set 0
    end
    i32.const 0
    local.get 1
    i32.store offset=1048596
    local.get 0
  )
  (func $js_sys.string_decode (;8;) (type 6) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $js_sys.import.string_decode
    call $js_sys.externref.insert
  )
  (func $js_sys.externref.grow (;9;) (type 7) (param i32) (result i32)
    ref.null extern
    local.get 0
    table.grow 0
  )
  (func $js_sys.externref.insert (;10;) (type 8) (param externref) (result i32)
    (local i32)
    call $js_sys.externref.next
    local.tee 1
    local.get 0
    table.set 0
    local.get 1
  )
  (func $js_sys.externref.get (;11;) (type 9) (param i32) (result externref)
    local.get 0
    table.get 0
  )
  (func $js_sys.externref.remove (;12;) (type 4) (param i32)
    local.get 0
    ref.null extern
    table.set 0
  )
  (func $web_sys.console.log (;13;) (type 4) (param i32)
    local.get 0
    call $js_sys.externref.get
    call $web_sys.import.console.log
  )
  (data $.rodata (;0;) (i32.const 1048576) "Hello, World!")
  (data $.data (;1;) (i32.const 1048592) "\04\00\00\00")
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.92.0 (ded5c06cf 2025-12-08)")
  )
  (@custom "target_features" (after data) "\08+\0bbulk-memory+\0fbulk-memory-opt+\16call-indirect-overlong+\0amultivalue+\0fmutable-globals+\13nontrapping-fptoint+\0freference-types+\08sign-ext")
)
