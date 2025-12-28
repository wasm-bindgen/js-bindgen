(module $basic-b5a1f30c82cbea71.wasm
  (type (;0;) (func (result externref)))
  (type (;1;) (func (param externref)))
  (type (;2;) (func))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (result i32)))
  (type (;5;) (func (param i32) (result i32)))
  (type (;6;) (func (param externref) (result i32)))
  (type (;7;) (func (param i32) (result externref)))
  (type (;8;) (func (param i32)))
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
    (local i32 i32)
    call $js_sys.is_nan
    local.tee 0
    call $web_sys.console.log
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1048580
        br_if 1 (;@1;)
        i32.const 0
        i32.const -1
        i32.store offset=1048580
        block ;; label = @3
          i32.const 0
          i32.load offset=1048588
          local.tee 1
          i32.const 0
          i32.load offset=1048584
          i32.ne
          br_if 0 (;@3;)
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17had1084cbf211c5eeE
        end
        i32.const 0
        local.get 1
        i32.const 1
        i32.add
        i32.store offset=1048588
        i32.const 0
        i32.load offset=1048576
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
      return
    end
    call $_ZN4core4cell22panic_already_borrowed17heeb094e5c4b1bc01E
    unreachable
  )
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17had1084cbf211c5eeE (;3;) (type 2)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 8
    i32.add
    i32.const 0
    i32.load offset=1048584
    call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14grow_amortized17hbdf5fdba18540de2E
    block ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 1
      i32.const -2147483647
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      i32.load offset=12
      call $_ZN5alloc7raw_vec12handle_error17h801d426cf510b77bE
      unreachable
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN4core4cell22panic_already_borrowed17heeb094e5c4b1bc01E (;4;) (type 2)
    call $_ZN4core4cell22panic_already_borrowed8do_panic7runtime17h5120f5d632e3deefE
    unreachable
  )
  (func $_RNvCsiGVaDesi5rv_7___rustc25___rdl_alloc_error_handler (;5;) (type 3) (param i32 i32)
    call $_ZN4core9panicking18panic_nounwind_fmt17h13ae3b8cc1e6e417E
    unreachable
  )
  (func $_ZN4core9panicking18panic_nounwind_fmt17h13ae3b8cc1e6e417E (;6;) (type 2)
    unreachable
  )
  (func $_ZN5alloc7raw_vec12handle_error17h801d426cf510b77bE (;7;) (type 3) (param i32 i32)
    block ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      call $_RNvCsiGVaDesi5rv_7___rustc26___rust_alloc_error_handler
      unreachable
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17hf37eaeedcf19c4ccE
    unreachable
  )
  (func $_RNvCsiGVaDesi5rv_7___rustc26___rust_alloc_error_handler (;8;) (type 2)
    i32.const 0
    i32.const 0
    call $_RNvCsiGVaDesi5rv_7___rustc25___rdl_alloc_error_handler
    unreachable
  )
  (func $_ZN5alloc7raw_vec17capacity_overflow17hf37eaeedcf19c4ccE (;9;) (type 2)
    call $_ZN4core9panicking18panic_nounwind_fmt17h13ae3b8cc1e6e417E
    unreachable
  )
  (func $_ZN4core4cell22panic_already_borrowed8do_panic7runtime17h5120f5d632e3deefE (;10;) (type 2)
    call $_ZN4core9panicking18panic_nounwind_fmt17h13ae3b8cc1e6e417E
    unreachable
  )
  (func $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14grow_amortized17hbdf5fdba18540de2E (;11;) (type 3) (param i32 i32)
    (local i32 i32)
    block ;; label = @1
      local.get 1
      i32.const 1
      i32.add
      local.tee 1
      i32.const 0
      i32.load offset=1048584
      local.tee 2
      i32.const 1
      i32.shl
      local.tee 3
      local.get 1
      local.get 3
      i32.gt_u
      select
      local.tee 1
      i32.const 1073741823
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      local.get 1
      i32.const 4
      i32.gt_u
      select
      i32.const 2
      i32.shl
      i32.const 2147483644
      i32.gt_u
      br_if 0 (;@1;)
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      loop ;; label = @2
        br 0 (;@2;)
      end
    end
    local.get 0
    i32.const 0
    i32.store
  )
  (func $js_sys.externref.next (;12;) (type 4) (result i32)
    (local i32 i32)
    block ;; label = @1
      i32.const 0
      i32.load offset=1048580
      br_if 0 (;@1;)
      i32.const 0
      i32.const -1
      i32.store offset=1048580
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load offset=1048588
          local.tee 0
          br_if 0 (;@3;)
          block ;; label = @4
            i32.const 1
            call $js_sys.externref.grow
            local.tee 0
            i32.const -1
            i32.eq
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1048580
            i32.const 1
            i32.add
            local.set 1
            br 2 (;@2;)
          end
          call $_ZN4core9panicking18panic_nounwind_fmt17h13ae3b8cc1e6e417E
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
      return
    end
    call $_ZN4core4cell22panic_already_borrowed17heeb094e5c4b1bc01E
    unreachable
  )
  (func $js_sys.externref.grow (;13;) (type 5) (param i32) (result i32)
    ref.null extern
    local.get 0
    table.grow 0
  )
  (func $js_sys.externref.insert (;14;) (type 6) (param externref) (result i32)
    (local i32)
    call $js_sys.externref.next
    local.tee 1
    local.get 0
    table.set 0
    local.get 1
  )
  (func $js_sys.externref.get (;15;) (type 7) (param i32) (result externref)
    local.get 0
    table.get 0
  )
  (func $js_sys.externref.remove (;16;) (type 8) (param i32)
    local.get 0
    ref.null extern
    table.set 0
  )
  (func $js_sys.is_nan (;17;) (type 4) (result i32)
    call $js_sys.import.is_nan
    call $js_sys.externref.insert
  )
  (func $web_sys.console.log (;18;) (type 8) (param i32)
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
