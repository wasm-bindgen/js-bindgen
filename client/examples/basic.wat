(module $basic-f6a23c98359ac5e9.wasm
  (type (;0;) (func (param i32 i32) (result externref)))
  (type (;1;) (func (param externref)))
  (type (;2;) (func))
  (type (;3;) (func (param externref externref)))
  (type (;4;) (func (param i32)))
  (type (;5;) (func (param i32 i32 i32 i32)))
  (type (;6;) (func (result i32)))
  (type (;7;) (func (param i32 i32) (result i32)))
  (type (;8;) (func (param i32) (result i32)))
  (type (;9;) (func (param externref) (result i32)))
  (type (;10;) (func (param i32) (result externref)))
  (type (;11;) (func (param i32 i32)))
  (import "js_bindgen" "memory" (memory (;0;) 17))
  (import "js_sys" "string_decode" (func $js_sys.import.string_decode (;0;) (type 0)))
  (import "js_sys" "externref.table" (table (;0;) 0 externref))
  (import "web_sys" "console.log" (func $web_sys.import.console.log (;1;) (type 1)))
  (import "web_sys" "console.log0" (func $web_sys.import.console.log0 (;2;) (type 2)))
  (import "web_sys" "console.log2" (func $web_sys.import.console.log2 (;3;) (type 3)))
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048624)
  (global (;2;) i32 i32.const 1048624)
  (export "foo" (func $foo))
  (export "__heap_base" (global 1))
  (export "js_sys.externref.next" (func $js_sys.externref.next))
  (export "__data_end" (global 2))
  (start $__wasm_init_memory)
  (func $__wasm_init_memory (;4;) (type 2)
    i32.const 1048604
    i32.const 0
    i32.const 20
    memory.fill
  )
  (func $foo (;5;) (type 2)
    (local i32 i32)
    call $web_sys.console.log0
    i32.const 1048576
    i32.const 13
    call $js_sys.string_decode
    local.tee 0
    call $web_sys.console.log
    local.get 0
    call $_ZN64_$LT$js_sys..value..JsValue$u20$as$u20$core..ops..drop..Drop$GT$4drop17hcc06f1dfe1f1dee5E
    i32.const 1048589
    i32.const 5
    call $js_sys.string_decode
    local.tee 0
    i32.const 1048594
    i32.const 6
    call $js_sys.string_decode
    local.tee 1
    call $web_sys.console.log2
    local.get 1
    call $_ZN64_$LT$js_sys..value..JsValue$u20$as$u20$core..ops..drop..Drop$GT$4drop17hcc06f1dfe1f1dee5E
    local.get 0
    call $_ZN64_$LT$js_sys..value..JsValue$u20$as$u20$core..ops..drop..Drop$GT$4drop17hcc06f1dfe1f1dee5E
  )
  (func $_ZN64_$LT$js_sys..value..JsValue$u20$as$u20$core..ops..drop..Drop$GT$4drop17hcc06f1dfe1f1dee5E (;6;) (type 4) (param i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.const 2
          i32.lt_s
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1048604
          br_if 2 (;@1;)
          i32.const 0
          i32.const -1
          i32.store offset=1048604
          i32.const 0
          i32.load offset=1048600
          local.set 2
          block ;; label = @4
            i32.const 0
            i32.load offset=1048608
            i32.const 0
            i32.load offset=1048612
            local.tee 3
            i32.ne
            br_if 0 (;@4;)
            local.get 1
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
            call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h68cc2c96f40b31bcE
            local.get 1
            i32.load offset=4
            br_if 2 (;@2;)
            i32.const 0
            local.get 1
            i32.load offset=8
            local.tee 2
            i32.store offset=1048600
            i32.const 0
            local.get 4
            i32.store offset=1048608
          end
          i32.const 0
          local.get 3
          i32.const 1
          i32.add
          i32.store offset=1048612
          local.get 2
          local.get 3
          i32.const 2
          i32.shl
          i32.add
          local.get 0
          i32.store
          local.get 0
          call $js_sys.externref.remove
          i32.const 0
          i32.const 0
          i32.load offset=1048604
          i32.const 1
          i32.add
          i32.store offset=1048604
        end
        local.get 1
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 22
      call $_ZN4core6result13unwrap_failed17h28bb9ae37aca2287E
      unreachable
    end
    i32.const 43
    call $_ZN4core6result13unwrap_failed17h28bb9ae37aca2287E
    unreachable
  )
  (func $_ZN4core9panicking9panic_fmt17h806e647715990138E (;7;) (type 2)
    unreachable
  )
  (func $_ZN4core6result13unwrap_failed17h28bb9ae37aca2287E (;8;) (type 4) (param i32)
    call $_ZN4core9panicking9panic_fmt17h806e647715990138E
    unreachable
  )
  (func $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h68cc2c96f40b31bcE (;9;) (type 5) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.const 1073741823
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 2
            i32.shl
            local.tee 3
            i32.const 2147483645
            i32.lt_u
            br_if 1 (;@3;)
          end
          local.get 0
          i32.const 0
          i32.store offset=4
          br 1 (;@2;)
        end
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
              block ;; label = @6
                i32.const 0
                i32.load offset=1048616
                local.tee 4
                br_if 0 (;@6;)
                memory.size
                local.set 5
                i32.const 0
                i32.const 0
                i32.const 1048624
                i32.sub
                local.tee 4
                i32.store offset=1048616
                i32.const 0
                i32.const 1
                local.get 5
                i32.const 16
                i32.shl
                i32.sub
                i32.store offset=1048620
              end
              local.get 4
              i32.const -4
              i32.and
              local.tee 4
              local.get 3
              i32.lt_u
              br_if 1 (;@4;)
              block ;; label = @6
                i32.const 0
                i32.load offset=1048620
                local.tee 6
                local.get 4
                local.get 3
                i32.sub
                local.tee 5
                i32.const 1
                i32.or
                i32.le_u
                br_if 0 (;@6;)
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
                br_if 2 (;@4;)
                i32.const 0
                local.get 6
                local.get 7
                i32.const 16
                i32.shl
                i32.sub
                i32.store offset=1048620
              end
              i32.const 0
              local.get 5
              i32.store offset=1048616
              local.get 4
              i32.eqz
              br_if 1 (;@4;)
              i32.const 0
              local.get 4
              i32.sub
              local.set 4
              block ;; label = @6
                local.get 1
                i32.const 2
                i32.shl
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
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
              br 4 (;@1;)
            end
            block ;; label = @5
              i32.const 0
              i32.load offset=1048616
              local.tee 1
              br_if 0 (;@5;)
              memory.size
              local.set 2
              i32.const 0
              i32.const 0
              i32.const 1048624
              i32.sub
              local.tee 1
              i32.store offset=1048616
              i32.const 0
              i32.const 1
              local.get 2
              i32.const 16
              i32.shl
              i32.sub
              i32.store offset=1048620
            end
            local.get 1
            i32.const -4
            i32.and
            local.tee 4
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            block ;; label = @5
              i32.const 0
              i32.load offset=1048620
              local.tee 2
              local.get 4
              local.get 3
              i32.sub
              local.tee 1
              i32.const 1
              i32.or
              i32.le_u
              br_if 0 (;@5;)
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
              br_if 1 (;@4;)
              i32.const 0
              local.get 2
              local.get 5
              i32.const 16
              i32.shl
              i32.sub
              i32.store offset=1048620
            end
            i32.const 0
            local.get 1
            i32.store offset=1048616
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
            br_if 1 (;@3;)
            i32.const 0
            local.get 4
            i32.sub
            local.set 4
            br 3 (;@1;)
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
      return
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
  )
  (func $js_sys.externref.next (;10;) (type 6) (result i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.load offset=1048604
        br_if 0 (;@2;)
        i32.const 0
        i32.const -1
        i32.store offset=1048604
        block ;; label = @3
          i32.const 0
          i32.load offset=1048612
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          i32.const 0
          local.get 0
          i32.const -1
          i32.add
          local.tee 0
          i32.store offset=1048612
          i32.const 0
          i32.load offset=1048600
          local.get 0
          i32.const 2
          i32.shl
          i32.add
          i32.load
          local.set 0
          br 2 (;@1;)
        end
        block ;; label = @3
          i32.const 1
          call $js_sys.externref.grow
          local.tee 0
          i32.const -1
          i32.eq
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1048604
          i32.const 1
          i32.add
          local.set 1
          br 2 (;@1;)
        end
        unreachable
      end
      i32.const 43
      call $_ZN4core6result13unwrap_failed17h28bb9ae37aca2287E
      unreachable
    end
    i32.const 0
    local.get 1
    i32.store offset=1048604
    local.get 0
  )
  (func $js_sys.string_decode (;11;) (type 7) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $js_sys.import.string_decode
    call $js_sys.externref.insert
  )
  (func $js_sys.externref.grow (;12;) (type 8) (param i32) (result i32)
    ref.null extern
    local.get 0
    table.grow 0
  )
  (func $js_sys.externref.insert (;13;) (type 9) (param externref) (result i32)
    (local i32)
    call $js_sys.externref.next
    local.tee 1
    local.get 0
    table.set 0
    local.get 1
  )
  (func $js_sys.externref.get (;14;) (type 10) (param i32) (result externref)
    local.get 0
    table.get 0
  )
  (func $js_sys.externref.remove (;15;) (type 4) (param i32)
    local.get 0
    ref.null extern
    table.set 0
  )
  (func $web_sys.console.log (;16;) (type 4) (param i32)
    local.get 0
    call $js_sys.externref.get
    call $web_sys.import.console.log
  )
  (func $web_sys.console.log0 (;17;) (type 2)
    call $web_sys.import.console.log0
  )
  (func $web_sys.console.log2 (;18;) (type 11) (param i32 i32)
    local.get 0
    call $js_sys.externref.get
    local.get 1
    call $js_sys.externref.get
    call $web_sys.import.console.log2
  )
  (data $.rodata (;0;) (i32.const 1048576) "Hello, World!HelloWorld!")
  (data $.data (;1;) (i32.const 1048600) "\04\00\00\00")
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.93.1 (01f6ddf75 2026-02-11)")
    (processed-by "js-bindgen" "0.0.0")
  )
  (@custom "target_features" (after data) "\08+\0bbulk-memory+\0fbulk-memory-opt+\16call-indirect-overlong+\0amultivalue+\0fmutable-globals+\13nontrapping-fptoint+\0freference-types+\08sign-ext")
)
