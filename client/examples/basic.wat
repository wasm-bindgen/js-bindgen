(module $basic-b5a1f30c82cbea71.wasm
  (type (;0;) (func (result externref)))
  (type (;1;) (func (param externref)))
  (type (;2;) (func))
  (type (;3;) (func (result i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param externref) (result i32)))
  (type (;6;) (func (param i32) (result externref)))
  (type (;7;) (func (param i32)))
  (import "js_sys" "externref.table" (table (;0;) 0 externref))
  (import "js_sys" "is_nan" (func $js_sys.import.is_nan (;0;) (type 0)))
  (import "web_sys" "console.log" (func $web_sys.import.console.log (;1;) (type 1)))
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048592)
  (global (;2;) i32 i32.const 1048592)
  (export "memory" (memory 0))
  (export "foo" (func $foo))
  (export "js_sys.externref.next" (func $js_sys.externref.next))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (func $foo (;2;) (type 2)
    (local i32 i32 i32)
    call $js_sys.is_nan
    local.tee 0
    call $web_sys.console.log
    block ;; label = @1
      local.get 0
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load offset=1048580
          br_if 0 (;@3;)
          i32.const 0
          i32.const -1
          i32.store offset=1048580
          i32.const 0
          i32.load offset=1048584
          i32.const 0
          i32.load offset=1048588
          local.tee 1
          i32.ne
          br_if 1 (;@2;)
          local.get 1
          i32.const -536870912
          i32.add
          i32.const -536870911
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 1
          i32.shl
          local.tee 0
          i32.const 4
          local.get 0
          i32.const 4
          i32.gt_u
          select
          i32.const 2
          i32.shl
          i32.const 2147483644
          i32.gt_u
          br_if 0 (;@3;)
          loop ;; label = @4
            br 0 (;@4;)
          end
        end
        unreachable
      end
      i32.const 0
      i32.load offset=1048576
      local.set 2
      i32.const 0
      local.get 1
      i32.const 1
      i32.add
      i32.store offset=1048588
      local.get 2
      local.get 1
      i32.const 2
      i32.shl
      i32.add
      local.get 0
      i32.store
      local.get 0
      call $js_sys.externref.remove
      i32.const 0
      i32.const 0
      i32.load offset=1048580
      i32.const 1
      i32.add
      i32.store offset=1048580
    end
  )
  (func $js_sys.externref.next (;3;) (type 3) (result i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load offset=1048580
          br_if 0 (;@3;)
          i32.const 0
          i32.const -1
          i32.store offset=1048580
          i32.const 0
          i32.load offset=1048588
          local.tee 0
          br_if 1 (;@2;)
          i32.const 1
          call $js_sys.externref.grow
          local.tee 0
          i32.const -1
          i32.eq
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1048580
          i32.const 1
          i32.add
          local.set 1
          br 2 (;@1;)
        end
        unreachable
      end
      i32.const 0
      local.set 1
      i32.const 0
      local.get 0
      i32.const -1
      i32.add
      local.tee 0
      i32.store offset=1048588
      i32.const 0
      i32.load offset=1048576
      local.get 0
      i32.const 2
      i32.shl
      i32.add
      i32.load
      local.set 0
    end
    i32.const 0
    local.get 1
    i32.store offset=1048580
    local.get 0
  )
  (func $js_sys.externref.grow (;4;) (type 4) (param i32) (result i32)
    ref.null extern
    local.get 0
    table.grow 0
  )
  (func $js_sys.externref.insert (;5;) (type 5) (param externref) (result i32)
    (local i32)
    call $js_sys.externref.next
    local.tee 1
    local.get 0
    table.set 0
    local.get 1
  )
  (func $js_sys.externref.get (;6;) (type 6) (param i32) (result externref)
    local.get 0
    table.get 0
  )
  (func $js_sys.externref.remove (;7;) (type 7) (param i32)
    local.get 0
    ref.null extern
    table.set 0
  )
  (func $js_sys.is_nan (;8;) (type 3) (result i32)
    call $js_sys.import.is_nan
    call $js_sys.externref.insert
  )
  (func $web_sys.console.log (;9;) (type 7) (param i32)
    local.get 0
    call $js_sys.externref.get
    call $web_sys.import.console.log
  )
  (data $.data (;0;) (i32.const 1048576) "\04\00\00\00")
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.92.0 (ded5c06cf 2025-12-08)")
  )
  (@custom "target_features" (after data) "\08+\0bbulk-memory+\0fbulk-memory-opt+\16call-indirect-overlong+\0amultivalue+\0fmutable-globals+\13nontrapping-fptoint+\0freference-types+\08sign-ext")
)
