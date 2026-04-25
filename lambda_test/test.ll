; ModuleID = 'lambda_program'
source_filename = "lambda_program"

@string_literal = private global <{ i64, ptr }> <{ i64 30, ptr @.str_blob }>
@.str_blob = private constant [30 x i8] c"=== Pair<float32> methods ===\0A"
@string_literal.1 = private global <{ i64, ptr }> <{ i64 42, ptr @.str_blob.2 }>
@.str_blob.2 = private constant [42 x i8] c"=== boolean short-circuit and assigns ===\0A"
@string_literal.3 = private global <{ i64, ptr }> <{ i64 42, ptr @.str_blob.4 }>
@.str_blob.4 = private constant [42 x i8] c"=== weird: many instantiations of max ===\0A"
@string_literal.5 = private global <{ i64, ptr }> <{ i64 48, ptr @.str_blob.6 }>
@.str_blob.6 = private constant [48 x i8] c"=== weird: function results as generic args ===\0A"
@string_literal.7 = private global <{ i64, ptr }> <{ i64 12, ptr @.str_blob.8 }>
@.str_blob.8 = private constant [12 x i8] c"=== gcd ===\0A"
@string_literal.9 = private global <{ i64, ptr }> <{ i64 10, ptr @.str_blob.10 }>
@.str_blob.10 = private constant [10 x i8] c"all done!\0A"
@string_literal.11 = private global <{ i64, ptr }> <{ i64 30, ptr @.str_blob.12 }>
@.str_blob.12 = private constant [30 x i8] c"=== Counter self-mutation ===\0A"
@string_literal.13 = private global <{ i64, ptr }> <{ i64 41, ptr @.str_blob.14 }>
@.str_blob.14 = private constant [41 x i8] c"=== weird: Pair sum through identity ===\0A"
@string_literal.15 = private global <{ i64, ptr }> <{ i64 44, ptr @.str_blob.16 }>
@.str_blob.16 = private constant [44 x i8] c"=== weird: factorial overflow into int8 ===\0A"
@string_literal.17 = private global <{ i64, ptr }> <{ i64 28, ptr @.str_blob.18 }>
@.str_blob.18 = private constant [28 x i8] c"=== Pair<int32> methods ===\0A"
@string_literal.19 = private global <{ i64, ptr }> <{ i64 39, ptr @.str_blob.20 }>
@.str_blob.20 = private constant [39 x i8] c"=== max / min across numeric types ===\0A"
@string_literal.21 = private global <{ i64, ptr }> <{ i64 51, ptr @.str_blob.22 }>
@.str_blob.22 = private constant [51 x i8] c"=== double_identity (generic -> generic chain) ===\0A"
@string_literal.23 = private global <{ i64, ptr }> <{ i64 42, ptr @.str_blob.24 }>
@.str_blob.24 = private constant [42 x i8] c"=== Pair<int64> (third instantiation) ===\0A"
@string_literal.25 = private global <{ i64, ptr }> <{ i64 34, ptr @.str_blob.26 }>
@.str_blob.26 = private constant [34 x i8] c"=== Vec2 operator overloading ===\0A"
@string_literal.27 = private global <{ i64, ptr }> <{ i64 33, ptr @.str_blob.28 }>
@.str_blob.28 = private constant [33 x i8] c"=== factorial / fib / sum_to ===\0A"
@string_literal.29 = private global <{ i64, ptr }> <{ i64 14, ptr @.str_blob.30 }>
@.str_blob.30 = private constant [14 x i8] c"=== casts ===\0A"
@string_literal.31 = private global <{ i64, ptr }> <{ i64 36, ptr @.str_blob.32 }>
@.str_blob.32 = private constant [36 x i8] c"=== compound arithmetic assigns ===\0A"
@string_literal.33 = private global <{ i64, ptr }> <{ i64 33, ptr @.str_blob.34 }>
@.str_blob.34 = private constant [33 x i8] c"=== compound bitwise assigns ===\0A"
@string_literal.35 = private global <{ i64, ptr }> <{ i64 24, ptr @.str_blob.36 }>
@.str_blob.36 = private constant [24 x i8] c"=== unary operators ===\0A"
@string_literal.37 = private global <{ i64, ptr }> <{ i64 42, ptr @.str_blob.38 }>
@.str_blob.38 = private constant [42 x i8] c"=== weird: negative fib edge (n <= 1) ===\0A"
@string_literal.39 = private global <{ i64, ptr }> <{ i64 54, ptr @.str_blob.40 }>
@.str_blob.40 = private constant [54 x i8] c"=== Vec2 chained: result of op used in further op ===\0A"
@string_literal.41 = private global <{ i64, ptr }> <{ i64 30, ptr @.str_blob.42 }>
@.str_blob.42 = private constant [30 x i8] c"=== identity across types ===\0A"
@string_literal.43 = private global <{ i64, ptr }> <{ i64 35, ptr @.str_blob.44 }>
@.str_blob.44 = private constant [35 x i8] c"=== nested: identity(max(...)) ===\0A"

declare void @cprint(<{ i64, ptr }>)

define { float, float } @Vec2__op_add(ptr %0, { float, float } %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %rhs = alloca { float, float }, align 8
  store { float, float } %1, ptr %rhs, align 4
  %return_var = alloca { float, float }, align 8
  %r = alloca { float, float }, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load { float, float }, ptr %return_var, align 4
  ret { float, float } %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { float, float }, ptr %self1, align 4
  %member = extractvalue { float, float } %refread, 0
  %rhs2 = load { float, float }, ptr %rhs, align 4
  %member3 = extractvalue { float, float } %rhs2, 0
  %fadd = fadd float %member, %member3
  %field_ptr = getelementptr inbounds nuw { float, float }, ptr %r, i32 0, i32 0
  store float %fadd, ptr %field_ptr, align 4
  %self4 = load ptr, ptr %self, align 8
  %refread5 = load { float, float }, ptr %self4, align 4
  %member6 = extractvalue { float, float } %refread5, 1
  %rhs7 = load { float, float }, ptr %rhs, align 4
  %member8 = extractvalue { float, float } %rhs7, 1
  %fadd9 = fadd float %member6, %member8
  %field_ptr10 = getelementptr inbounds nuw { float, float }, ptr %r, i32 0, i32 1
  store float %fadd9, ptr %field_ptr10, align 4
  %r11 = load { float, float }, ptr %r, align 4
  store { float, float } %r11, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define { float, float } @Vec2__op_sub(ptr %0, { float, float } %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %rhs = alloca { float, float }, align 8
  store { float, float } %1, ptr %rhs, align 4
  %return_var = alloca { float, float }, align 8
  %r = alloca { float, float }, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load { float, float }, ptr %return_var, align 4
  ret { float, float } %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { float, float }, ptr %self1, align 4
  %member = extractvalue { float, float } %refread, 0
  %rhs2 = load { float, float }, ptr %rhs, align 4
  %member3 = extractvalue { float, float } %rhs2, 0
  %fsub = fsub float %member, %member3
  %field_ptr = getelementptr inbounds nuw { float, float }, ptr %r, i32 0, i32 0
  store float %fsub, ptr %field_ptr, align 4
  %self4 = load ptr, ptr %self, align 8
  %refread5 = load { float, float }, ptr %self4, align 4
  %member6 = extractvalue { float, float } %refread5, 1
  %rhs7 = load { float, float }, ptr %rhs, align 4
  %member8 = extractvalue { float, float } %rhs7, 1
  %fsub9 = fsub float %member6, %member8
  %field_ptr10 = getelementptr inbounds nuw { float, float }, ptr %r, i32 0, i32 1
  store float %fsub9, ptr %field_ptr10, align 4
  %r11 = load { float, float }, ptr %r, align 4
  store { float, float } %r11, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @Vec2__op_cmp(ptr %0, { float, float } %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %rhs = alloca { float, float }, align 8
  store { float, float } %1, ptr %rhs, align 4
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { float, float }, ptr %self1, align 4
  %member = extractvalue { float, float } %refread, 0
  %rhs2 = load { float, float }, ptr %rhs, align 4
  %member3 = extractvalue { float, float } %rhs2, 0
  %flt = fcmp olt float %member, %member3
  br i1 %flt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge32, %drop_and_return_40, %drop_and_return_28, %drop_and_return_16, %drop_and_return_5
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_4

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_6
  %self10 = load ptr, ptr %self, align 8
  %refread11 = load { float, float }, ptr %self10, align 4
  %member12 = extractvalue { float, float } %refread11, 0
  %rhs13 = load { float, float }, ptr %rhs, align 4
  %member14 = extractvalue { float, float } %rhs13, 0
  %fgt = fcmp ogt float %member12, %member14
  br i1 %fgt, label %then7, label %else8

code_4:                                           ; preds = %then
  store i32 -1, ptr %return_var, align 4
  br label %drop_and_return_5

drop_and_return_5:                                ; preds = %code_4
  br label %drop_and_return_

drop_and_merge_6:                                 ; No predecessors!
  br label %if_merge

then7:                                            ; preds = %if_merge
  br label %code_15

else8:                                            ; preds = %if_merge
  br label %if_merge9

if_merge9:                                        ; preds = %else8, %drop_and_merge_17
  %self21 = load ptr, ptr %self, align 8
  %refread22 = load { float, float }, ptr %self21, align 4
  %member23 = extractvalue { float, float } %refread22, 1
  %rhs24 = load { float, float }, ptr %rhs, align 4
  %member25 = extractvalue { float, float } %rhs24, 1
  %flt26 = fcmp olt float %member23, %member25
  br i1 %flt26, label %then18, label %else19

code_15:                                          ; preds = %then7
  store i8 1, ptr %return_var, align 1
  br label %drop_and_return_16

drop_and_return_16:                               ; preds = %code_15
  br label %drop_and_return_

drop_and_merge_17:                                ; No predecessors!
  br label %if_merge9

then18:                                           ; preds = %if_merge9
  br label %code_27

else19:                                           ; preds = %if_merge9
  br label %if_merge20

if_merge20:                                       ; preds = %else19, %drop_and_merge_29
  %self33 = load ptr, ptr %self, align 8
  %refread34 = load { float, float }, ptr %self33, align 4
  %member35 = extractvalue { float, float } %refread34, 1
  %rhs36 = load { float, float }, ptr %rhs, align 4
  %member37 = extractvalue { float, float } %rhs36, 1
  %fgt38 = fcmp ogt float %member35, %member37
  br i1 %fgt38, label %then30, label %else31

code_27:                                          ; preds = %then18
  store i32 -1, ptr %return_var, align 4
  br label %drop_and_return_28

drop_and_return_28:                               ; preds = %code_27
  br label %drop_and_return_

drop_and_merge_29:                                ; No predecessors!
  br label %if_merge20

then30:                                           ; preds = %if_merge20
  br label %code_39

else31:                                           ; preds = %if_merge20
  br label %if_merge32

if_merge32:                                       ; preds = %else31, %drop_and_merge_41
  store i8 0, ptr %return_var, align 1
  br label %drop_and_return_

code_39:                                          ; preds = %then30
  store i8 1, ptr %return_var, align 1
  br label %drop_and_return_40

drop_and_return_40:                               ; preds = %code_39
  br label %drop_and_return_

drop_and_merge_41:                                ; No predecessors!
  br label %if_merge32
}

define void @Counter_tick(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i32 }, ptr %self1, align 4
  %member = extractvalue { i32 } %refread, 0
  %iadd = add i32 %member, 1
  %self2 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { i32 }, ptr %self2, i32 0, i32 0
  store i32 %iadd, ptr %field_ptr, align 4
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define void @Counter_add_n(ptr %0, i32 %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %k = alloca i32, align 4
  store i32 %1, ptr %k, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i32 }, ptr %self1, align 4
  %member = extractvalue { i32 } %refread, 0
  %k2 = load i32, ptr %k, align 4
  %iadd = add i32 %member, %k2
  %self3 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { i32 }, ptr %self3, i32 0, i32 0
  store i32 %iadd, ptr %field_ptr, align 4
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define i32 @Counter_value(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i32 }, ptr %self1, align 4
  %member = extractvalue { i32 } %refread, 0
  store i32 %member, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @Counter_reset(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { i32 }, ptr %self1, i32 0, i32 0
  store i32 0, ptr %field_ptr, align 4
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define i32 @factorial(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, ptr %n, align 4
  %return_var = alloca i32, align 4
  %r = alloca i32, align 4
  %i = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  store i32 1, ptr %r, align 4
  store i32 2, ptr %i, align 4
  br label %loop_start

drop_and_return_:                                 ; preds = %loop_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

loop_start:                                       ; preds = %drop_and_merge_5, %code_
  %i1 = load i32, ptr %i, align 4
  %n2 = load i32, ptr %n, align 4
  %ile = icmp sle i32 %i1, %n2
  br i1 %ile, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_3

loop_merge:                                       ; preds = %loop_start
  %r9 = load i32, ptr %r, align 4
  store i32 %r9, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %loop
  %r6 = load i32, ptr %r, align 4
  %i7 = load i32, ptr %i, align 4
  %imul = mul i32 %r6, %i7
  store i32 %imul, ptr %r, align 4
  %i8 = load i32, ptr %i, align 4
  %iadd = add i32 %i8, 1
  store i32 %iadd, ptr %i, align 4
  br label %drop_and_merge_5

drop_and_return_4:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_5:                                 ; preds = %code_3
  br label %loop_start
}

define i32 @fib(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, ptr %n, align 4
  %return_var = alloca i32, align 4
  %a = alloca i32, align 4
  %b = alloca i32, align 4
  %i = alloca i32, align 4
  %t = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %n1 = load i32, ptr %n, align 4
  %ile = icmp sle i32 %n1, 1
  br i1 %ile, label %then, label %else

drop_and_return_:                                 ; preds = %loop_merge, %drop_and_return_10, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  store i32 0, ptr %a, align 4
  store i32 1, ptr %b, align 4
  store i32 2, ptr %i, align 4
  br label %loop_start

code_2:                                           ; preds = %then
  %n5 = load i32, ptr %n, align 4
  store i32 %n5, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge

loop_start:                                       ; preds = %drop_and_merge_11, %if_merge
  %i6 = load i32, ptr %i, align 4
  %n7 = load i32, ptr %n, align 4
  %ile8 = icmp sle i32 %i6, %n7
  br i1 %ile8, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_9

loop_merge:                                       ; preds = %loop_start
  %b18 = load i32, ptr %b, align 4
  store i32 %b18, ptr %return_var, align 4
  br label %drop_and_return_

code_9:                                           ; preds = %loop
  %a12 = load i32, ptr %a, align 4
  %b13 = load i32, ptr %b, align 4
  %iadd = add i32 %a12, %b13
  store i32 %iadd, ptr %t, align 4
  %b14 = load i32, ptr %b, align 4
  store i32 %b14, ptr %a, align 4
  %t15 = load i32, ptr %t, align 4
  store i32 %t15, ptr %b, align 4
  %i16 = load i32, ptr %i, align 4
  %iadd17 = add i32 %i16, 1
  store i32 %iadd17, ptr %i, align 4
  br label %drop_and_merge_11

drop_and_return_10:                               ; No predecessors!
  br label %drop_and_return_

drop_and_merge_11:                                ; preds = %code_9
  br label %loop_start
}

define i32 @sum_to(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, ptr %n, align 4
  %return_var = alloca i32, align 4
  %total = alloca i32, align 4
  %i = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  store i32 0, ptr %total, align 4
  store i32 1, ptr %i, align 4
  br label %loop_start

drop_and_return_:                                 ; preds = %loop_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

loop_start:                                       ; preds = %drop_and_merge_5, %code_
  %i1 = load i32, ptr %i, align 4
  %n2 = load i32, ptr %n, align 4
  %ile = icmp sle i32 %i1, %n2
  br i1 %ile, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_3

loop_merge:                                       ; preds = %loop_start
  %total9 = load i32, ptr %total, align 4
  store i32 %total9, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %loop
  %ca_load = load i32, ptr %total, align 4
  %i6 = load i32, ptr %i, align 4
  %iadd = add i32 %ca_load, %i6
  store i32 %iadd, ptr %total, align 4
  %ca_load7 = load i32, ptr %i, align 4
  %iadd8 = add i32 %ca_load7, 1
  store i32 %iadd8, ptr %i, align 4
  br label %drop_and_merge_5

drop_and_return_4:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_5:                                 ; preds = %code_3
  br label %loop_start
}

define i32 @abs(i32 %0) {
entry:
  %x = alloca i32, align 4
  store i32 %0, ptr %x, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %x1 = load i32, ptr %x, align 4
  %ilt = icmp slt i32 %x1, 0
  br i1 %ilt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  %x6 = load i32, ptr %x, align 4
  store i32 %x6, ptr %return_var, align 4
  br label %drop_and_return_

code_2:                                           ; preds = %then
  %x5 = load i32, ptr %x, align 4
  %ineg = sub i32 0, %x5
  store i32 %ineg, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i32 @gcd(i32 %0, i32 %1) {
entry:
  %a = alloca i32, align 4
  store i32 %0, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 %1, ptr %b, align 4
  %return_var = alloca i32, align 4
  %t = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  br label %loop_start

drop_and_return_:                                 ; preds = %loop_merge, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

loop_start:                                       ; preds = %drop_and_merge_4, %code_
  %b1 = load i32, ptr %b, align 4
  %ine = icmp ne i32 %b1, 0
  br i1 %ine, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_2

loop_merge:                                       ; preds = %loop_start
  %a11 = load i32, ptr %a, align 4
  %call = call i32 @abs(i32 %a11)
  store i32 %call, ptr %return_var, align 4
  br label %drop_and_return_

code_2:                                           ; preds = %loop
  %b5 = load i32, ptr %b, align 4
  store i32 %b5, ptr %t, align 4
  %a6 = load i32, ptr %a, align 4
  %a7 = load i32, ptr %a, align 4
  %b8 = load i32, ptr %b, align 4
  %idiv = sdiv i32 %a7, %b8
  %b9 = load i32, ptr %b, align 4
  %imul = mul i32 %idiv, %b9
  %isub = sub i32 %a6, %imul
  store i32 %isub, ptr %b, align 4
  %t10 = load i32, ptr %t, align 4
  store i32 %t10, ptr %a, align 4
  br label %drop_and_merge_4

drop_and_return_3:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_4:                                 ; preds = %code_2
  br label %loop_start
}

define i8 @main() {
entry:
  %return_var = alloca i8, align 1
  %i_i32 = alloca i32, align 4
  %i_i64 = alloca i64, align 8
  %i_u32 = alloca i32, align 4
  %i_f32 = alloca float, align 4
  %i_f64 = alloca double, align 8
  %i_bool = alloca i1, align 1
  %di_i32 = alloca i32, align 4
  %di_i64 = alloca i64, align 8
  %di_bool = alloca i1, align 1
  %mx_i32 = alloca i32, align 4
  %mn_i32 = alloca i32, align 4
  %mx_i64 = alloca i64, align 8
  %mx_u8 = alloca i8, align 1
  %mx_f32 = alloca float, align 4
  %mx_f64 = alloca double, align 8
  %mn_f64 = alloca double, align 8
  %p = alloca { i32, i32 }, align 8
  %p_sum = alloca i32, align 4
  %p_first = alloca i32, align 4
  %p_after = alloca i32, align 4
  %fp = alloca { float, float }, align 8
  %fp_sum = alloca float, align 4
  %lp = alloca { i64, i64 }, align 8
  %lp_sum = alloca i64, align 8
  %va = alloca { float, float }, align 8
  %vb = alloca { float, float }, align 8
  %vc = alloca { float, float }, align 8
  %vd = alloca { float, float }, align 8
  %eq_aa = alloca i1, align 1
  %eq_ab = alloca i1, align 1
  %lt_ab = alloca i1, align 1
  %gt_ba = alloca i1, align 1
  %ne_ab = alloca i1, align 1
  %le_aa = alloca i1, align 1
  %vsum = alloca { float, float }, align 8
  %vdiff = alloca { float, float }, align 8
  %ct = alloca { i32 }, align 8
  %cv1 = alloca i32, align 4
  %cv2 = alloca i32, align 4
  %cv3 = alloca i32, align 4
  %f0 = alloca i32, align 4
  %f1 = alloca i32, align 4
  %f5 = alloca i32, align 4
  %fib0 = alloca i32, align 4
  %fib1 = alloca i32, align 4
  %fib10 = alloca i32, align 4
  %s10 = alloca i32, align 4
  %g1 = alloca i32, align 4
  %g2 = alloca i32, align 4
  %g3 = alloca i32, align 4
  %g4 = alloca i32, align 4
  %big = alloca i64, align 8
  %to_i32 = alloca i32, align 4
  %to_i8 = alloca i8, align 1
  %fl = alloca float, align 4
  %to_int = alloca i32, align 4
  %u255 = alloca i8, align 1
  %s_neg1 = alloca i8, align 1
  %orig = alloca i32, align 4
  %as_f = alloca double, align 8
  %back = alloca i32, align 4
  %x = alloca i32, align 4
  %bx = alloca i32, align 4
  %bt = alloca i1, align 1
  %bf = alloca i1, align 1
  %and_r = alloca i1, align 1
  %or_r = alloca i1, align 1
  %pos = alloca i32, align 4
  %neg_val = alloca i32, align 4
  %zero = alloca i32, align 4
  %all_ones = alloca i32, align 4
  %nbool = alloca i1, align 1
  %not_true = alloca i1, align 1
  %nested = alloca i32, align 4
  %weird1 = alloca i32, align 4
  %weird2 = alloca i32, align 4
  %p2 = alloca { i32, i32 }, align 8
  %p2sum = alloca i32, align 4
  %f10 = alloca i32, align 4
  %f10_i8 = alloca i8, align 1
  %fib_neg = alloca i32, align 4
  %r_u8 = alloca i8, align 1
  %r_u16 = alloca i16, align 2
  %r_u32 = alloca i32, align 4
  %r_i8 = alloca i8, align 1
  %r_f32 = alloca float, align 4
  %r_f64 = alloca double, align 8
  %check = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %string_literal = load <{ i64, ptr }>, ptr @string_literal.41, align 1
  call void @cprint(<{ i64, ptr }> %string_literal)
  %call = call i32 @identity_int32(i32 42)
  store i32 %call, ptr %i_i32, align 4
  %call1 = call i64 @identity_int64(i64 9999999999)
  store i64 %call1, ptr %i_i64, align 4
  %call2 = call i32 @identity_uint32(i32 -1)
  store i32 %call2, ptr %i_u32, align 4
  %call3 = call float @identity_float32(float 0x40091EB860000000)
  store float %call3, ptr %i_f32, align 4
  %call4 = call double @identity_float64(double 0x4005BF0A8B04919B)
  store double %call4, ptr %i_f64, align 8
  %call5 = call i1 @identity_bool(i1 true)
  store i1 %call5, ptr %i_bool, align 1
  %string_literal6 = load <{ i64, ptr }>, ptr @string_literal.21, align 1
  call void @cprint(<{ i64, ptr }> %string_literal6)
  %call7 = call i32 @double_identity_int32(i32 99)
  store i32 %call7, ptr %di_i32, align 4
  %call8 = call i64 @double_identity_int64(i64 1234567890)
  store i64 %call8, ptr %di_i64, align 4
  %call9 = call i1 @double_identity_bool(i1 false)
  store i1 %call9, ptr %di_bool, align 1
  %string_literal10 = load <{ i64, ptr }>, ptr @string_literal.19, align 1
  call void @cprint(<{ i64, ptr }> %string_literal10)
  %call11 = call i32 @max_int32(i32 10, i32 20)
  store i32 %call11, ptr %mx_i32, align 4
  %call12 = call i32 @min_int32(i32 10, i32 20)
  store i32 %call12, ptr %mn_i32, align 4
  %call13 = call i64 @max_int64(i64 100, i64 50)
  store i64 %call13, ptr %mx_i64, align 4
  %call14 = call i8 @max_uint8(i8 -56, i8 100)
  store i8 %call14, ptr %mx_u8, align 1
  %call15 = call float @max_float32(float 1.500000e+00, float 2.500000e+00)
  store float %call15, ptr %mx_f32, align 4
  %call16 = call double @max_float64(double 3.140000e+00, double 2.720000e+00)
  store double %call16, ptr %mx_f64, align 8
  %call17 = call double @min_float64(double 3.140000e+00, double 2.720000e+00)
  store double %call17, ptr %mn_f64, align 8
  %string_literal18 = load <{ i64, ptr }>, ptr @string_literal.17, align 1
  call void @cprint(<{ i64, ptr }> %string_literal18)
  %field_ptr = getelementptr inbounds nuw { i32, i32 }, ptr %p, i32 0, i32 0
  store i32 7, ptr %field_ptr, align 4
  %field_ptr19 = getelementptr inbounds nuw { i32, i32 }, ptr %p, i32 0, i32 1
  store i32 13, ptr %field_ptr19, align 4
  %call20 = call i32 @Pair_int32_sum(ptr %p)
  store i32 %call20, ptr %p_sum, align 4
  %call21 = call i32 @Pair_int32_first(ptr %p)
  store i32 %call21, ptr %p_first, align 4
  call void @Pair_int32_swap(ptr %p)
  %call22 = call i32 @Pair_int32_first(ptr %p)
  store i32 %call22, ptr %p_after, align 4
  %string_literal23 = load <{ i64, ptr }>, ptr @string_literal, align 1
  call void @cprint(<{ i64, ptr }> %string_literal23)
  %field_ptr24 = getelementptr inbounds nuw { float, float }, ptr %fp, i32 0, i32 0
  store float 1.500000e+00, ptr %field_ptr24, align 4
  %field_ptr25 = getelementptr inbounds nuw { float, float }, ptr %fp, i32 0, i32 1
  store float 2.500000e+00, ptr %field_ptr25, align 4
  %call26 = call float @Pair_float32_sum(ptr %fp)
  store float %call26, ptr %fp_sum, align 4
  %string_literal27 = load <{ i64, ptr }>, ptr @string_literal.23, align 1
  call void @cprint(<{ i64, ptr }> %string_literal27)
  %field_ptr28 = getelementptr inbounds nuw { i64, i64 }, ptr %lp, i32 0, i32 0
  store i64 1000000000, ptr %field_ptr28, align 4
  %field_ptr29 = getelementptr inbounds nuw { i64, i64 }, ptr %lp, i32 0, i32 1
  store i64 2000000000, ptr %field_ptr29, align 4
  %call30 = call i64 @Pair_int64_sum(ptr %lp)
  store i64 %call30, ptr %lp_sum, align 4
  %string_literal31 = load <{ i64, ptr }>, ptr @string_literal.25, align 1
  call void @cprint(<{ i64, ptr }> %string_literal31)
  %field_ptr32 = getelementptr inbounds nuw { float, float }, ptr %va, i32 0, i32 0
  store float 1.000000e+00, ptr %field_ptr32, align 4
  %field_ptr33 = getelementptr inbounds nuw { float, float }, ptr %va, i32 0, i32 1
  store float 2.000000e+00, ptr %field_ptr33, align 4
  %field_ptr34 = getelementptr inbounds nuw { float, float }, ptr %vb, i32 0, i32 0
  store float 3.000000e+00, ptr %field_ptr34, align 4
  %field_ptr35 = getelementptr inbounds nuw { float, float }, ptr %vb, i32 0, i32 1
  store float 4.000000e+00, ptr %field_ptr35, align 4
  %va36 = load { float, float }, ptr %va, align 4
  %vb37 = load { float, float }, ptr %vb, align 4
  %tmp_self = alloca { float, float }, align 8
  store { float, float } %va36, ptr %tmp_self, align 4
  %user_binop = call { float, float } @Vec2__op_add(ptr %tmp_self, { float, float } %vb37)
  store { float, float } %user_binop, ptr %vc, align 4
  %vb38 = load { float, float }, ptr %vb, align 4
  %va39 = load { float, float }, ptr %va, align 4
  %tmp_self40 = alloca { float, float }, align 8
  store { float, float } %vb38, ptr %tmp_self40, align 4
  %user_binop41 = call { float, float } @Vec2__op_sub(ptr %tmp_self40, { float, float } %va39)
  store { float, float } %user_binop41, ptr %vd, align 4
  %va42 = load { float, float }, ptr %va, align 4
  %va43 = load { float, float }, ptr %va, align 4
  %tmp_self44 = alloca { float, float }, align 8
  store { float, float } %va42, ptr %tmp_self44, align 4
  %user_cmp = call i8 @Vec2__op_cmp(ptr %tmp_self44, { float, float } %va43)
  %ucmp = icmp eq i8 %user_cmp, 0
  store i1 %ucmp, ptr %eq_aa, align 1
  %va45 = load { float, float }, ptr %va, align 4
  %vb46 = load { float, float }, ptr %vb, align 4
  %tmp_self47 = alloca { float, float }, align 8
  store { float, float } %va45, ptr %tmp_self47, align 4
  %user_cmp48 = call i8 @Vec2__op_cmp(ptr %tmp_self47, { float, float } %vb46)
  %ucmp49 = icmp eq i8 %user_cmp48, 0
  store i1 %ucmp49, ptr %eq_ab, align 1
  %va50 = load { float, float }, ptr %va, align 4
  %vb51 = load { float, float }, ptr %vb, align 4
  %tmp_self52 = alloca { float, float }, align 8
  store { float, float } %va50, ptr %tmp_self52, align 4
  %user_cmp53 = call i8 @Vec2__op_cmp(ptr %tmp_self52, { float, float } %vb51)
  %ucmp54 = icmp slt i8 %user_cmp53, 0
  store i1 %ucmp54, ptr %lt_ab, align 1
  %vb55 = load { float, float }, ptr %vb, align 4
  %va56 = load { float, float }, ptr %va, align 4
  %tmp_self57 = alloca { float, float }, align 8
  store { float, float } %vb55, ptr %tmp_self57, align 4
  %user_cmp58 = call i8 @Vec2__op_cmp(ptr %tmp_self57, { float, float } %va56)
  %ucmp59 = icmp sgt i8 %user_cmp58, 0
  store i1 %ucmp59, ptr %gt_ba, align 1
  %va60 = load { float, float }, ptr %va, align 4
  %vb61 = load { float, float }, ptr %vb, align 4
  %tmp_self62 = alloca { float, float }, align 8
  store { float, float } %va60, ptr %tmp_self62, align 4
  %user_cmp63 = call i8 @Vec2__op_cmp(ptr %tmp_self62, { float, float } %vb61)
  %ucmp64 = icmp ne i8 %user_cmp63, 0
  store i1 %ucmp64, ptr %ne_ab, align 1
  %va65 = load { float, float }, ptr %va, align 4
  %va66 = load { float, float }, ptr %va, align 4
  %tmp_self67 = alloca { float, float }, align 8
  store { float, float } %va65, ptr %tmp_self67, align 4
  %user_cmp68 = call i8 @Vec2__op_cmp(ptr %tmp_self67, { float, float } %va66)
  %ucmp69 = icmp sle i8 %user_cmp68, 0
  store i1 %ucmp69, ptr %le_aa, align 1
  %string_literal70 = load <{ i64, ptr }>, ptr @string_literal.39, align 1
  call void @cprint(<{ i64, ptr }> %string_literal70)
  %vc71 = load { float, float }, ptr %vc, align 4
  %vd72 = load { float, float }, ptr %vd, align 4
  %tmp_self73 = alloca { float, float }, align 8
  store { float, float } %vc71, ptr %tmp_self73, align 4
  %user_binop74 = call { float, float } @Vec2__op_add(ptr %tmp_self73, { float, float } %vd72)
  store { float, float } %user_binop74, ptr %vsum, align 4
  %vc75 = load { float, float }, ptr %vc, align 4
  %vd76 = load { float, float }, ptr %vd, align 4
  %tmp_self77 = alloca { float, float }, align 8
  store { float, float } %vc75, ptr %tmp_self77, align 4
  %user_binop78 = call { float, float } @Vec2__op_sub(ptr %tmp_self77, { float, float } %vd76)
  store { float, float } %user_binop78, ptr %vdiff, align 4
  %string_literal79 = load <{ i64, ptr }>, ptr @string_literal.11, align 1
  call void @cprint(<{ i64, ptr }> %string_literal79)
  %field_ptr80 = getelementptr inbounds nuw { i32 }, ptr %ct, i32 0, i32 0
  store i32 0, ptr %field_ptr80, align 4
  call void @Counter_tick(ptr %ct)
  call void @Counter_tick(ptr %ct)
  call void @Counter_tick(ptr %ct)
  %call81 = call i32 @Counter_value(ptr %ct)
  store i32 %call81, ptr %cv1, align 4
  call void @Counter_add_n(ptr %ct, i32 7)
  %call82 = call i32 @Counter_value(ptr %ct)
  store i32 %call82, ptr %cv2, align 4
  call void @Counter_reset(ptr %ct)
  %call83 = call i32 @Counter_value(ptr %ct)
  store i32 %call83, ptr %cv3, align 4
  %string_literal84 = load <{ i64, ptr }>, ptr @string_literal.27, align 1
  call void @cprint(<{ i64, ptr }> %string_literal84)
  %call85 = call i32 @factorial(i32 0)
  store i32 %call85, ptr %f0, align 4
  %call86 = call i32 @factorial(i32 1)
  store i32 %call86, ptr %f1, align 4
  %call87 = call i32 @factorial(i32 5)
  store i32 %call87, ptr %f5, align 4
  %call88 = call i32 @fib(i32 0)
  store i32 %call88, ptr %fib0, align 4
  %call89 = call i32 @fib(i32 1)
  store i32 %call89, ptr %fib1, align 4
  %call90 = call i32 @fib(i32 10)
  store i32 %call90, ptr %fib10, align 4
  %call91 = call i32 @sum_to(i32 10)
  store i32 %call91, ptr %s10, align 4
  %string_literal92 = load <{ i64, ptr }>, ptr @string_literal.7, align 1
  call void @cprint(<{ i64, ptr }> %string_literal92)
  %call93 = call i32 @gcd(i32 48, i32 18)
  store i32 %call93, ptr %g1, align 4
  %call94 = call i32 @gcd(i32 100, i32 75)
  store i32 %call94, ptr %g2, align 4
  %call95 = call i32 @gcd(i32 7, i32 13)
  store i32 %call95, ptr %g3, align 4
  %call96 = call i32 @gcd(i32 42, i32 0)
  store i32 %call96, ptr %g4, align 4
  %string_literal97 = load <{ i64, ptr }>, ptr @string_literal.29, align 1
  call void @cprint(<{ i64, ptr }> %string_literal97)
  store i64 300, ptr %big, align 4
  %big98 = load i64, ptr %big, align 4
  %trunc = trunc i64 %big98 to i32
  store i32 %trunc, ptr %to_i32, align 4
  %big99 = load i64, ptr %big, align 4
  %trunc100 = trunc i64 %big99 to i8
  store i8 %trunc100, ptr %to_i8, align 1
  store float 0x401F9999A0000000, ptr %fl, align 4
  %fl101 = load float, ptr %fl, align 4
  %fptosi = fptosi float %fl101 to i32
  store i32 %fptosi, ptr %to_int, align 4
  store i8 -1, ptr %u255, align 1
  %u255102 = load i8, ptr %u255, align 1
  store i8 %u255102, ptr %s_neg1, align 1
  store i32 42, ptr %orig, align 4
  %orig103 = load i32, ptr %orig, align 4
  %sitofp = sitofp i32 %orig103 to double
  store double %sitofp, ptr %as_f, align 8
  %as_f104 = load double, ptr %as_f, align 8
  %fptosi105 = fptosi double %as_f104 to i32
  store i32 %fptosi105, ptr %back, align 4
  %string_literal106 = load <{ i64, ptr }>, ptr @string_literal.31, align 1
  call void @cprint(<{ i64, ptr }> %string_literal106)
  store i32 10, ptr %x, align 4
  %ca_load = load i32, ptr %x, align 4
  %iadd = add i32 %ca_load, 5
  store i32 %iadd, ptr %x, align 4
  %ca_load107 = load i32, ptr %x, align 4
  %isub = sub i32 %ca_load107, 3
  store i32 %isub, ptr %x, align 4
  %ca_load108 = load i32, ptr %x, align 4
  %imul = mul i32 %ca_load108, 2
  store i32 %imul, ptr %x, align 4
  %ca_load109 = load i32, ptr %x, align 4
  %idiv = sdiv i32 %ca_load109, 4
  store i32 %idiv, ptr %x, align 4
  %string_literal110 = load <{ i64, ptr }>, ptr @string_literal.33, align 1
  call void @cprint(<{ i64, ptr }> %string_literal110)
  store i32 255, ptr %bx, align 4
  %ca_load111 = load i32, ptr %bx, align 4
  %band = and i32 %ca_load111, 15
  store i32 %band, ptr %bx, align 4
  %ca_load112 = load i32, ptr %bx, align 4
  %bor = or i32 %ca_load112, 48
  store i32 %bor, ptr %bx, align 4
  %ca_load113 = load i32, ptr %bx, align 4
  %bxor = xor i32 %ca_load113, 15
  store i32 %bxor, ptr %bx, align 4
  %ca_load114 = load i32, ptr %bx, align 4
  %shl = shl i32 %ca_load114, 2
  store i32 %shl, ptr %bx, align 4
  %ca_load115 = load i32, ptr %bx, align 4
  %ashr = ashr i32 %ca_load115, 1
  store i32 %ashr, ptr %bx, align 4
  %string_literal116 = load <{ i64, ptr }>, ptr @string_literal.1, align 1
  call void @cprint(<{ i64, ptr }> %string_literal116)
  store i1 true, ptr %bt, align 1
  store i1 false, ptr %bf, align 1
  %bt117 = load i1, ptr %bt, align 1
  br i1 %bt117, label %and_rhs, label %and_merge

drop_and_return_:                                 ; preds = %bca_merge125
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

and_rhs:                                          ; preds = %code_
  %bf118 = load i1, ptr %bf, align 1
  br label %and_merge

and_merge:                                        ; preds = %and_rhs, %code_
  %and_result = phi i1 [ false, %code_ ], [ %bf118, %and_rhs ]
  store i1 %and_result, ptr %and_r, align 1
  %bt119 = load i1, ptr %bt, align 1
  br i1 %bt119, label %or_merge, label %or_rhs

or_rhs:                                           ; preds = %and_merge
  %bf120 = load i1, ptr %bf, align 1
  br label %or_merge

or_merge:                                         ; preds = %or_rhs, %and_merge
  %or_result = phi i1 [ true, %and_merge ], [ %bf120, %or_rhs ]
  store i1 %or_result, ptr %or_r, align 1
  %ca_load121 = load i1, ptr %bt, align 1
  br i1 %ca_load121, label %bca_rhs, label %bca_merge

bca_rhs:                                          ; preds = %or_merge
  %bf122 = load i1, ptr %bf, align 1
  br label %bca_merge

bca_merge:                                        ; preds = %bca_rhs, %or_merge
  %bca_result = phi i1 [ false, %or_merge ], [ %bf122, %bca_rhs ]
  store i1 %bca_result, ptr %bt, align 1
  %ca_load123 = load i1, ptr %bt, align 1
  br i1 %ca_load123, label %bca_merge125, label %bca_rhs124

bca_rhs124:                                       ; preds = %bca_merge
  br label %bca_merge125

bca_merge125:                                     ; preds = %bca_rhs124, %bca_merge
  %bca_result126 = phi i1 [ true, %bca_merge ], [ true, %bca_rhs124 ]
  store i1 %bca_result126, ptr %bt, align 1
  %string_literal127 = load <{ i64, ptr }>, ptr @string_literal.35, align 1
  call void @cprint(<{ i64, ptr }> %string_literal127)
  store i32 42, ptr %pos, align 4
  %pos128 = load i32, ptr %pos, align 4
  %ineg = sub i32 0, %pos128
  store i32 %ineg, ptr %neg_val, align 4
  store i32 0, ptr %zero, align 4
  %zero129 = load i32, ptr %zero, align 4
  %bnot = xor i32 %zero129, -1
  store i32 %bnot, ptr %all_ones, align 4
  store i1 true, ptr %nbool, align 1
  %nbool130 = load i1, ptr %nbool, align 1
  %bnot131 = xor i1 %nbool130, true
  store i1 %bnot131, ptr %not_true, align 1
  %string_literal132 = load <{ i64, ptr }>, ptr @string_literal.43, align 1
  call void @cprint(<{ i64, ptr }> %string_literal132)
  %call133 = call i32 @max_int32(i32 5, i32 8)
  %call134 = call i32 @identity_int32(i32 %call133)
  store i32 %call134, ptr %nested, align 4
  %string_literal135 = load <{ i64, ptr }>, ptr @string_literal.5, align 1
  call void @cprint(<{ i64, ptr }> %string_literal135)
  %call136 = call i32 @factorial(i32 3)
  %call137 = call i32 @fib(i32 7)
  %call138 = call i32 @max_int32(i32 %call136, i32 %call137)
  store i32 %call138, ptr %weird1, align 4
  %call139 = call i32 @sum_to(i32 5)
  %call140 = call i32 @factorial(i32 4)
  %call141 = call i32 @min_int32(i32 %call139, i32 %call140)
  store i32 %call141, ptr %weird2, align 4
  %string_literal142 = load <{ i64, ptr }>, ptr @string_literal.13, align 1
  call void @cprint(<{ i64, ptr }> %string_literal142)
  %field_ptr143 = getelementptr inbounds nuw { i32, i32 }, ptr %p2, i32 0, i32 0
  store i32 100, ptr %field_ptr143, align 4
  %field_ptr144 = getelementptr inbounds nuw { i32, i32 }, ptr %p2, i32 0, i32 1
  store i32 100, ptr %field_ptr144, align 4
  %call145 = call i32 @Pair_int32_sum(ptr %p2)
  %call146 = call i32 @identity_int32(i32 %call145)
  store i32 %call146, ptr %p2sum, align 4
  %string_literal147 = load <{ i64, ptr }>, ptr @string_literal.15, align 1
  call void @cprint(<{ i64, ptr }> %string_literal147)
  %call148 = call i32 @factorial(i32 10)
  store i32 %call148, ptr %f10, align 4
  %f10149 = load i32, ptr %f10, align 4
  %trunc150 = trunc i32 %f10149 to i8
  store i8 %trunc150, ptr %f10_i8, align 1
  %string_literal151 = load <{ i64, ptr }>, ptr @string_literal.37, align 1
  call void @cprint(<{ i64, ptr }> %string_literal151)
  %call152 = call i32 @fib(i32 -1)
  store i32 %call152, ptr %fib_neg, align 4
  %string_literal153 = load <{ i64, ptr }>, ptr @string_literal.3, align 1
  call void @cprint(<{ i64, ptr }> %string_literal153)
  %call154 = call i8 @max_uint8(i8 10, i8 20)
  store i8 %call154, ptr %r_u8, align 1
  %call155 = call i16 @max_uint16(i16 100, i16 200)
  store i16 %call155, ptr %r_u16, align 2
  %call156 = call i32 @max_uint32(i32 1000, i32 2000)
  store i32 %call156, ptr %r_u32, align 4
  %call157 = call i8 @max_int8(i8 10, i8 20)
  store i8 %call157, ptr %r_i8, align 1
  %call158 = call float @max_float32(float 1.500000e+00, float 2.500000e+00)
  store float %call158, ptr %r_f32, align 4
  %call159 = call double @max_float64(double 1.500000e+00, double 2.500000e+00)
  store double %call159, ptr %r_f64, align 8
  %string_literal160 = load <{ i64, ptr }>, ptr @string_literal.9, align 1
  call void @cprint(<{ i64, ptr }> %string_literal160)
  %f5161 = load i32, ptr %f5, align 4
  %isub162 = sub i32 %f5161, 120
  %fib10163 = load i32, ptr %fib10, align 4
  %isub164 = sub i32 %fib10163, 55
  %iadd165 = add i32 %isub162, %isub164
  store i32 %iadd165, ptr %check, align 4
  %check166 = load i32, ptr %check, align 4
  %trunc167 = trunc i32 %check166 to i8
  store i8 %trunc167, ptr %return_var, align 1
  br label %drop_and_return_
}

define i32 @identity_int32(i32 %0) {
entry:
  %x = alloca i32, align 4
  store i32 %0, ptr %x, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %x1 = load i32, ptr %x, align 4
  store i32 %x1, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @identity_int64(i64 %0) {
entry:
  %x = alloca i64, align 8
  store i64 %0, ptr %x, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %x1 = load i64, ptr %x, align 4
  store i64 %x1, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @identity_uint32(i32 %0) {
entry:
  %x = alloca i32, align 4
  store i32 %0, ptr %x, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %x1 = load i32, ptr %x, align 4
  store i32 %x1, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define float @identity_float32(float %0) {
entry:
  %x = alloca float, align 4
  store float %0, ptr %x, align 4
  %return_var = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load float, ptr %return_var, align 4
  ret float %return_val

code_:                                            ; preds = %entry
  %x1 = load float, ptr %x, align 4
  store float %x1, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define double @identity_float64(double %0) {
entry:
  %x = alloca double, align 8
  store double %0, ptr %x, align 8
  %return_var = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load double, ptr %return_var, align 8
  ret double %return_val

code_:                                            ; preds = %entry
  %x1 = load double, ptr %x, align 8
  store double %x1, ptr %return_var, align 8
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @identity_bool(i1 %0) {
entry:
  %x = alloca i1, align 1
  store i1 %0, ptr %x, align 1
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %x1 = load i1, ptr %x, align 1
  store i1 %x1, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @double_identity_int32(i32 %0) {
entry:
  %x = alloca i32, align 4
  store i32 %0, ptr %x, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %x1 = load i32, ptr %x, align 4
  %call = call i32 @identity_int32(i32 %x1)
  store i32 %call, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @double_identity_int64(i64 %0) {
entry:
  %x = alloca i64, align 8
  store i64 %0, ptr %x, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %x1 = load i64, ptr %x, align 4
  %call = call i64 @identity_int64(i64 %x1)
  store i64 %call, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @double_identity_bool(i1 %0) {
entry:
  %x = alloca i1, align 1
  store i1 %0, ptr %x, align 1
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %x1 = load i1, ptr %x, align 1
  %call = call i1 @identity_bool(i1 %x1)
  store i1 %call, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @max_int32(i32 %0, i32 %1) {
entry:
  %a = alloca i32, align 4
  store i32 %0, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 %1, ptr %b, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %a1 = load i32, ptr %a, align 4
  %b2 = load i32, ptr %b, align 4
  %igt = icmp sgt i32 %a1, %b2
  br i1 %igt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load i32, ptr %b, align 4
  store i32 %b7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load i32, ptr %a, align 4
  store i32 %a6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i32 @min_int32(i32 %0, i32 %1) {
entry:
  %a = alloca i32, align 4
  store i32 %0, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 %1, ptr %b, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %a1 = load i32, ptr %a, align 4
  %b2 = load i32, ptr %b, align 4
  %ilt = icmp slt i32 %a1, %b2
  br i1 %ilt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load i32, ptr %b, align 4
  store i32 %b7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load i32, ptr %a, align 4
  store i32 %a6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i64 @max_int64(i64 %0, i64 %1) {
entry:
  %a = alloca i64, align 8
  store i64 %0, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 %1, ptr %b, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %a1 = load i64, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %igt = icmp sgt i64 %a1, %b2
  br i1 %igt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load i64, ptr %b, align 4
  store i64 %b7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load i64, ptr %a, align 4
  store i64 %a6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i8 @max_uint8(i8 %0, i8 %1) {
entry:
  %a = alloca i8, align 1
  store i8 %0, ptr %a, align 1
  %b = alloca i8, align 1
  store i8 %1, ptr %b, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %a1 = load i8, ptr %a, align 1
  %b2 = load i8, ptr %b, align 1
  %ugt = icmp ugt i8 %a1, %b2
  br i1 %ugt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load i8, ptr %b, align 1
  store i8 %b7, ptr %return_var, align 1
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load i8, ptr %a, align 1
  store i8 %a6, ptr %return_var, align 1
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define float @max_float32(float %0, float %1) {
entry:
  %a = alloca float, align 4
  store float %0, ptr %a, align 4
  %b = alloca float, align 4
  store float %1, ptr %b, align 4
  %return_var = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load float, ptr %return_var, align 4
  ret float %return_val

code_:                                            ; preds = %entry
  %a1 = load float, ptr %a, align 4
  %b2 = load float, ptr %b, align 4
  %fgt = fcmp ogt float %a1, %b2
  br i1 %fgt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load float, ptr %b, align 4
  store float %b7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load float, ptr %a, align 4
  store float %a6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define double @max_float64(double %0, double %1) {
entry:
  %a = alloca double, align 8
  store double %0, ptr %a, align 8
  %b = alloca double, align 8
  store double %1, ptr %b, align 8
  %return_var = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load double, ptr %return_var, align 8
  ret double %return_val

code_:                                            ; preds = %entry
  %a1 = load double, ptr %a, align 8
  %b2 = load double, ptr %b, align 8
  %fgt = fcmp ogt double %a1, %b2
  br i1 %fgt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load double, ptr %b, align 8
  store double %b7, ptr %return_var, align 8
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load double, ptr %a, align 8
  store double %a6, ptr %return_var, align 8
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define double @min_float64(double %0, double %1) {
entry:
  %a = alloca double, align 8
  store double %0, ptr %a, align 8
  %b = alloca double, align 8
  store double %1, ptr %b, align 8
  %return_var = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load double, ptr %return_var, align 8
  ret double %return_val

code_:                                            ; preds = %entry
  %a1 = load double, ptr %a, align 8
  %b2 = load double, ptr %b, align 8
  %flt = fcmp olt double %a1, %b2
  br i1 %flt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load double, ptr %b, align 8
  store double %b7, ptr %return_var, align 8
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load double, ptr %a, align 8
  store double %a6, ptr %return_var, align 8
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i16 @max_uint16(i16 %0, i16 %1) {
entry:
  %a = alloca i16, align 2
  store i16 %0, ptr %a, align 2
  %b = alloca i16, align 2
  store i16 %1, ptr %b, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %a1 = load i16, ptr %a, align 2
  %b2 = load i16, ptr %b, align 2
  %ugt = icmp ugt i16 %a1, %b2
  br i1 %ugt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load i16, ptr %b, align 2
  store i16 %b7, ptr %return_var, align 2
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load i16, ptr %a, align 2
  store i16 %a6, ptr %return_var, align 2
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i32 @max_uint32(i32 %0, i32 %1) {
entry:
  %a = alloca i32, align 4
  store i32 %0, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 %1, ptr %b, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %a1 = load i32, ptr %a, align 4
  %b2 = load i32, ptr %b, align 4
  %ugt = icmp ugt i32 %a1, %b2
  br i1 %ugt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load i32, ptr %b, align 4
  store i32 %b7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load i32, ptr %a, align 4
  store i32 %a6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i8 @max_int8(i8 %0, i8 %1) {
entry:
  %a = alloca i8, align 1
  store i8 %0, ptr %a, align 1
  %b = alloca i8, align 1
  store i8 %1, ptr %b, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %a1 = load i8, ptr %a, align 1
  %b2 = load i8, ptr %b, align 1
  %igt = icmp sgt i8 %a1, %b2
  br i1 %igt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %b7 = load i8, ptr %b, align 1
  store i8 %b7, ptr %return_var, align 1
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %a6 = load i8, ptr %a, align 1
  store i8 %a6, ptr %return_var, align 1
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i32 @Pair_int32_sum(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i32, i32 }, ptr %self1, align 4
  %member = extractvalue { i32, i32 } %refread, 0
  %self2 = load ptr, ptr %self, align 8
  %refread3 = load { i32, i32 }, ptr %self2, align 4
  %member4 = extractvalue { i32, i32 } %refread3, 1
  %iadd = add i32 %member, %member4
  store i32 %iadd, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @Pair_int32_first(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i32, i32 }, ptr %self1, align 4
  %member = extractvalue { i32, i32 } %refread, 0
  store i32 %member, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @Pair_int32_second(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i32, i32 }, ptr %self1, align 4
  %member = extractvalue { i32, i32 } %refread, 1
  store i32 %member, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @Pair_int32_swap(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %tmp = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i32, i32 }, ptr %self1, align 4
  %member = extractvalue { i32, i32 } %refread, 0
  store i32 %member, ptr %tmp, align 4
  %self2 = load ptr, ptr %self, align 8
  %refread3 = load { i32, i32 }, ptr %self2, align 4
  %member4 = extractvalue { i32, i32 } %refread3, 1
  %self5 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { i32, i32 }, ptr %self5, i32 0, i32 0
  store i32 %member4, ptr %field_ptr, align 4
  %tmp6 = load i32, ptr %tmp, align 4
  %self7 = load ptr, ptr %self, align 8
  %field_ptr8 = getelementptr inbounds nuw { i32, i32 }, ptr %self7, i32 0, i32 1
  store i32 %tmp6, ptr %field_ptr8, align 4
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define float @Pair_float32_sum(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load float, ptr %return_var, align 4
  ret float %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { float, float }, ptr %self1, align 4
  %member = extractvalue { float, float } %refread, 0
  %self2 = load ptr, ptr %self, align 8
  %refread3 = load { float, float }, ptr %self2, align 4
  %member4 = extractvalue { float, float } %refread3, 1
  %fadd = fadd float %member, %member4
  store float %fadd, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define float @Pair_float32_first(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load float, ptr %return_var, align 4
  ret float %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { float, float }, ptr %self1, align 4
  %member = extractvalue { float, float } %refread, 0
  store float %member, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define float @Pair_float32_second(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load float, ptr %return_var, align 4
  ret float %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { float, float }, ptr %self1, align 4
  %member = extractvalue { float, float } %refread, 1
  store float %member, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @Pair_float32_swap(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %tmp = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { float, float }, ptr %self1, align 4
  %member = extractvalue { float, float } %refread, 0
  store float %member, ptr %tmp, align 4
  %self2 = load ptr, ptr %self, align 8
  %refread3 = load { float, float }, ptr %self2, align 4
  %member4 = extractvalue { float, float } %refread3, 1
  %self5 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { float, float }, ptr %self5, i32 0, i32 0
  store float %member4, ptr %field_ptr, align 4
  %tmp6 = load float, ptr %tmp, align 4
  %self7 = load ptr, ptr %self, align 8
  %field_ptr8 = getelementptr inbounds nuw { float, float }, ptr %self7, i32 0, i32 1
  store float %tmp6, ptr %field_ptr8, align 4
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define i64 @Pair_int64_sum(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i64, i64 }, ptr %self1, align 4
  %member = extractvalue { i64, i64 } %refread, 0
  %self2 = load ptr, ptr %self, align 8
  %refread3 = load { i64, i64 }, ptr %self2, align 4
  %member4 = extractvalue { i64, i64 } %refread3, 1
  %iadd = add i64 %member, %member4
  store i64 %iadd, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @Pair_int64_first(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i64, i64 }, ptr %self1, align 4
  %member = extractvalue { i64, i64 } %refread, 0
  store i64 %member, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @Pair_int64_second(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i64, i64 }, ptr %self1, align 4
  %member = extractvalue { i64, i64 } %refread, 1
  store i64 %member, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @Pair_int64_swap(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %tmp = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { i64, i64 }, ptr %self1, align 4
  %member = extractvalue { i64, i64 } %refread, 0
  store i64 %member, ptr %tmp, align 4
  %self2 = load ptr, ptr %self, align 8
  %refread3 = load { i64, i64 }, ptr %self2, align 4
  %member4 = extractvalue { i64, i64 } %refread3, 1
  %self5 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { i64, i64 }, ptr %self5, i32 0, i32 0
  store i64 %member4, ptr %field_ptr, align 4
  %tmp6 = load i64, ptr %tmp, align 4
  %self7 = load ptr, ptr %self, align 8
  %field_ptr8 = getelementptr inbounds nuw { i64, i64 }, ptr %self7, i32 0, i32 1
  store i64 %tmp6, ptr %field_ptr8, align 4
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}
