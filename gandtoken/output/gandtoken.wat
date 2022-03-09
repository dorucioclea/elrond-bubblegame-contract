(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i64) (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32) (result i32)))
  (type (;8;) (func))
  (type (;9;) (func (param i32 i32 i32 i32)))
  (import "env" "signalError" (func (;0;) (type 0)))
  (import "env" "mBufferNew" (func (;1;) (type 1)))
  (import "env" "managedCaller" (func (;2;) (type 2)))
  (import "env" "bigIntNew" (func (;3;) (type 3)))
  (import "env" "bigIntGetUnsignedArgument" (func (;4;) (type 0)))
  (import "env" "mBufferAppendBytes" (func (;5;) (type 4)))
  (import "env" "managedSignalError" (func (;6;) (type 2)))
  (import "env" "getNumArguments" (func (;7;) (type 1)))
  (import "env" "mBufferNewFromBytes" (func (;8;) (type 5)))
  (import "env" "bigIntAdd" (func (;9;) (type 6)))
  (import "env" "mBufferFromBigIntUnsigned" (func (;10;) (type 5)))
  (import "env" "mBufferAppend" (func (;11;) (type 5)))
  (import "env" "mBufferStorageLoad" (func (;12;) (type 5)))
  (import "env" "mBufferToBigIntUnsigned" (func (;13;) (type 5)))
  (import "env" "mBufferStorageStore" (func (;14;) (type 5)))
  (import "env" "bigIntFinishUnsigned" (func (;15;) (type 2)))
  (import "env" "bigIntCmp" (func (;16;) (type 5)))
  (import "env" "bigIntSub" (func (;17;) (type 6)))
  (import "env" "bigIntSign" (func (;18;) (type 7)))
  (import "env" "managedWriteLog" (func (;19;) (type 0)))
  (import "env" "checkNoPayment" (func (;20;) (type 8)))
  (import "env" "mBufferGetArgument" (func (;21;) (type 5)))
  (import "env" "mBufferGetLength" (func (;22;) (type 7)))
  (func (;23;) (type 2) (param i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call 0
    unreachable)
  (func (;24;) (type 2) (param i32)
    block  ;; label = @1
      local.get 0
      i32.load
      br_if 0 (;@1;)
      return
    end
    local.get 0
    call 23
    unreachable)
  (func (;25;) (type 1) (result i32)
    (local i32)
    call 1
    local.tee 0
    call 2
    local.get 0)
  (func (;26;) (type 7) (param i32) (result i32)
    (local i32)
    local.get 0
    i64.const 0
    call 3
    local.tee 1
    call 4
    local.get 1)
  (func (;27;) (type 0) (param i32 i32)
    (local i32)
    i32.const 1048576
    i32.const 23
    call 28
    local.tee 2
    local.get 0
    local.get 1
    call 5
    drop
    local.get 2
    i32.const 1048599
    i32.const 3
    call 5
    drop
    local.get 2
    i32.const 1048627
    i32.const 16
    call 5
    drop
    local.get 2
    call 6
    unreachable)
  (func (;28;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 8)
  (func (;29;) (type 2) (param i32)
    block  ;; label = @1
      call 7
      local.get 0
      i32.ne
      br_if 0 (;@1;)
      return
    end
    i32.const 1048602
    i32.const 25
    call 0
    unreachable)
  (func (;30;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.const 24
    i32.shl
    local.get 1
    i32.const 8
    i32.shl
    i32.const 16711680
    i32.and
    i32.or
    local.get 1
    i32.const 8
    i32.shr_u
    i32.const 65280
    i32.and
    local.get 1
    i32.const 24
    i32.shr_u
    i32.or
    i32.or
    i32.store offset=12
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    i32.const 4
    call 5
    drop
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;31;) (type 0) (param i32 i32)
    local.get 0
    local.get 0
    local.get 1
    call 9)
  (func (;32;) (type 7) (param i32) (result i32)
    (local i32)
    call 1
    local.tee 1
    local.get 0
    call 10
    drop
    local.get 1)
  (func (;33;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 11
    drop)
  (func (;34;) (type 7) (param i32) (result i32)
    (local i32)
    local.get 0
    call 1
    local.tee 1
    call 12
    drop
    local.get 1
    i64.const 0
    call 3
    local.tee 0
    call 13
    drop
    local.get 0)
  (func (;35;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 32
    call 14
    drop)
  (func (;36;) (type 7) (param i32) (result i32)
    call 1
    drop
    local.get 0
    call 32)
  (func (;37;) (type 0) (param i32 i32)
    (local i32)
    call 1
    drop
    call 1
    local.tee 2
    local.get 1
    call 11
    drop
    local.get 0
    local.get 2
    call 30)
  (func (;38;) (type 5) (param i32 i32) (result i32)
    (local i32)
    call 1
    local.tee 2
    local.get 0
    local.get 1
    call 8
    call 30
    local.get 2)
  (func (;39;) (type 2) (param i32)
    local.get 0
    call 34
    call 15)
  (func (;40;) (type 7) (param i32) (result i32)
    (local i32)
    i32.const 1048643
    i32.const 9
    call 8
    local.tee 1
    local.get 0
    i32.load
    call 33
    local.get 1)
  (func (;41;) (type 1) (result i32)
    i32.const 1048652
    i32.const 11
    call 8)
  (func (;42;) (type 5) (param i32 i32) (result i32)
    (local i32)
    i32.const 1048678
    i32.const 9
    call 8
    local.tee 2
    local.get 0
    i32.load
    call 33
    local.get 2
    local.get 1
    i32.load
    call 33
    local.get 2)
  (func (;43;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 16
    i32.const 1
    i32.lt_s)
  (func (;44;) (type 9) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 2
    i32.store offset=12
    local.get 4
    local.get 1
    i32.store offset=8
    i32.const 1048712
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        local.get 4
        i32.const 8
        i32.add
        call 40
        local.tee 6
        call 34
        local.tee 7
        call 43
        local.tee 8
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        local.get 7
        local.get 3
        call 17
        i32.const 0
        local.set 5
        local.get 7
        call 18
        i32.const -1
        i32.le_s
        br_if 1 (;@1;)
      end
      local.get 6
      local.get 7
      call 35
      block  ;; label = @2
        local.get 8
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.const 12
        i32.add
        call 40
        local.tee 7
        call 34
        local.tee 6
        local.get 3
        call 31
        local.get 7
        local.get 6
        call 35
        i32.const 1048670
        i32.const 8
        call 38
        local.tee 7
        local.get 1
        call 37
        local.get 7
        local.get 2
        call 37
        local.get 7
        local.get 3
        call 36
        call 19
      end
      local.get 0
      i32.const 18
      i32.store offset=4
      local.get 0
      local.get 5
      i32.store
      local.get 4
      i32.const 16
      i32.add
      global.set 0
      return
    end
    i32.const 1048752
    i32.const 48
    call 0
    unreachable)
  (func (;45;) (type 8)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 20
    i32.const 1
    call 29
    i32.const 0
    call 26
    local.set 1
    local.get 0
    call 25
    i32.store offset=12
    call 41
    local.get 1
    call 35
    local.get 0
    i32.const 12
    i32.add
    call 40
    local.tee 2
    call 34
    local.tee 3
    local.get 1
    call 31
    local.get 2
    local.get 3
    call 35
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;46;) (type 8)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 20
    i32.const 2
    call 29
    i32.const 0
    call 1
    local.tee 1
    call 21
    drop
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        call 22
        i32.const 32
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.store offset=8
        i32.const 1
        call 1
        local.tee 1
        call 21
        drop
        local.get 1
        call 22
        i32.const 32
        i32.ne
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 0
        i32.const 8
        i32.add
        local.get 0
        i32.const 12
        i32.add
        call 42
        call 39
        local.get 0
        i32.const 16
        i32.add
        global.set 0
        return
      end
      i32.const 1048696
      i32.const 5
      call 27
      unreachable
    end
    i32.const 1048687
    i32.const 7
    call 27
    unreachable)
  (func (;47;) (type 8)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 20
    i32.const 2
    call 29
    i32.const 0
    call 1
    local.tee 1
    call 21
    drop
    block  ;; label = @1
      local.get 1
      call 22
      i32.const 32
      i32.eq
      br_if 0 (;@1;)
      i32.const 1048687
      i32.const 7
      call 27
      unreachable
    end
    i32.const 1
    call 26
    local.set 2
    local.get 0
    local.get 1
    i32.store offset=8
    local.get 0
    call 25
    local.tee 3
    i32.store offset=12
    local.get 0
    i32.const 12
    i32.add
    local.get 0
    i32.const 8
    i32.add
    call 42
    local.get 2
    call 35
    i32.const 1048663
    i32.const 7
    call 38
    local.tee 4
    local.get 3
    call 37
    local.get 4
    local.get 1
    call 37
    local.get 4
    local.get 2
    call 36
    call 19
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    call 24
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;48;) (type 8)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 20
    i32.const 1
    call 29
    i32.const 0
    call 1
    local.tee 1
    call 21
    drop
    block  ;; label = @1
      local.get 1
      call 22
      i32.const 32
      i32.eq
      br_if 0 (;@1;)
      i32.const 1048701
      i32.const 7
      call 27
      unreachable
    end
    local.get 0
    local.get 1
    i32.store offset=12
    local.get 0
    i32.const 12
    i32.add
    call 40
    call 39
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;49;) (type 8)
    call 20
    i32.const 0
    call 29
    call 41
    call 39)
  (func (;50;) (type 8)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 20
    i32.const 2
    call 29
    i32.const 0
    call 1
    local.tee 1
    call 21
    drop
    block  ;; label = @1
      local.get 1
      call 22
      i32.const 32
      i32.eq
      br_if 0 (;@1;)
      i32.const 1048694
      i32.const 2
      call 27
      unreachable
    end
    i32.const 1
    call 26
    local.set 2
    local.get 0
    call 25
    local.get 1
    local.get 2
    call 44
    local.get 0
    i32.const 18
    i32.store offset=12
    local.get 0
    local.get 0
    i32.load
    i32.store offset=8
    local.get 0
    i32.const 8
    i32.add
    call 24
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;51;) (type 8)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    call 20
    i32.const 3
    call 29
    i32.const 0
    call 1
    local.tee 1
    call 21
    drop
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        call 22
        i32.const 32
        i32.ne
        br_if 0 (;@2;)
        i32.const 1
        call 1
        local.tee 2
        call 21
        drop
        local.get 2
        call 22
        i32.const 32
        i32.ne
        br_if 1 (;@1;)
        i32.const 2
        call 26
        local.set 3
        local.get 0
        local.get 1
        i32.store offset=28
        local.get 0
        call 25
        i32.store offset=16
        i32.const 1048730
        local.set 4
        block  ;; label = @3
          local.get 3
          local.get 0
          i32.const 28
          i32.add
          local.get 0
          i32.const 16
          i32.add
          call 42
          local.tee 5
          call 34
          local.tee 6
          call 43
          local.tee 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          local.get 3
          call 31
          i32.const 0
          local.set 4
        end
        local.get 5
        local.get 6
        call 35
        block  ;; label = @3
          block  ;; label = @4
            local.get 7
            br_if 0 (;@4;)
            i32.const 22
            local.set 1
            br 1 (;@3;)
          end
          local.get 0
          i32.const 8
          i32.add
          local.get 1
          local.get 2
          local.get 3
          call 44
          local.get 0
          i32.load offset=12
          local.set 1
          local.get 0
          i32.load offset=8
          local.set 4
        end
        local.get 0
        local.get 4
        i32.store offset=16
        local.get 0
        local.get 1
        i32.store offset=20
        local.get 0
        i32.const 16
        i32.add
        call 24
        local.get 0
        i32.const 32
        i32.add
        global.set 0
        return
      end
      i32.const 1048708
      i32.const 4
      call 27
      unreachable
    end
    i32.const 1048694
    i32.const 2
    call 27
    unreachable)
  (func (;52;) (type 8))
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048800))
  (global (;2;) i32 (i32.const 1048800))
  (export "memory" (memory 0))
  (export "init" (func 45))
  (export "allowance" (func 46))
  (export "approve" (func 47))
  (export "balanceOf" (func 48))
  (export "totalSupply" (func 49))
  (export "transfer" (func 50))
  (export "transferFrom" (func 51))
  (export "callBack" (func 52))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data (;0;) (i32.const 1048576) "argument decode error (): wrong number of argumentsbad array lengthbalanceOftotalSupplyapprovetransferallowancespendertoowneraccountfrominsufficient fundsInsufficient allowancecannot subtract because result would be negative"))
