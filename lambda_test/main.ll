; ModuleID = 'lambda_program'
source_filename = "lambda_program"

@string_literal = private global <{ i64, ptr }> <{ i64 14, ptr @.str_blob }>
@.str_blob = private constant [14 x i8] c"Hello, World!\0A"

define i8 @int8_abs(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %ilt = icmp slt i8 %self1, 0
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
  %self6 = load i8, ptr %self, align 1
  store i8 %self6, ptr %return_var, align 1
  br label %drop_and_return_

code_2:                                           ; preds = %then
  %self5 = load i8, ptr %self, align 1
  %ineg = sub i8 0, %self5
  store i8 %ineg, ptr %return_var, align 1
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i8 @int8_min(i8 %0, i8 %1) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %other = alloca i8, align 1
  store i8 %1, ptr %other, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %other2 = load i8, ptr %other, align 1
  %ilt = icmp slt i8 %self1, %other2
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
  %other7 = load i8, ptr %other, align 1
  store i8 %other7, ptr %return_var, align 1
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i8, ptr %self, align 1
  store i8 %self6, ptr %return_var, align 1
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i8 @int8_max(i8 %0, i8 %1) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %other = alloca i8, align 1
  store i8 %1, ptr %other, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %other2 = load i8, ptr %other, align 1
  %igt = icmp sgt i8 %self1, %other2
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
  %other7 = load i8, ptr %other, align 1
  store i8 %other7, ptr %return_var, align 1
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i8, ptr %self, align 1
  store i8 %self6, ptr %return_var, align 1
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i8 @int8_clamp(i8 %0, i8 %1, i8 %2) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %min = alloca i8, align 1
  store i8 %1, ptr %min, align 1
  %max = alloca i8, align 1
  store i8 %2, ptr %max, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %max2 = load i8, ptr %max, align 1
  %call = call i8 @int8_min(i8 %self1, i8 %max2)
  %min3 = load i8, ptr %min, align 1
  %call4 = call i8 @int8_max(i8 %call, i8 %min3)
  store i8 %call4, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int8_even(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %band = and i8 %self1, 1
  %ieq = icmp eq i8 %band, 0
  store i1 %ieq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int8_odd(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %band = and i8 %self1, 1
  %igt = icmp sgt i8 %band, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @int8_signum(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %ilt = icmp slt i8 %self1, 0
  br i1 %ilt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge7, %drop_and_return_10, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  %self8 = load i8, ptr %self, align 1
  %igt = icmp sgt i8 %self8, 0
  br i1 %igt, label %then5, label %else6

code_2:                                           ; preds = %then
  store i32 -1, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge

then5:                                            ; preds = %if_merge
  br label %code_9

else6:                                            ; preds = %if_merge
  br label %if_merge7

if_merge7:                                        ; preds = %else6, %drop_and_merge_11
  store i8 0, ptr %return_var, align 1
  br label %drop_and_return_

code_9:                                           ; preds = %then5
  store i8 1, ptr %return_var, align 1
  br label %drop_and_return_10

drop_and_return_10:                               ; preds = %code_9
  br label %drop_and_return_

drop_and_merge_11:                                ; No predecessors!
  br label %if_merge7
}

define i1 @int8_positive(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %igt = icmp sgt i8 %self1, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int8_negative(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %ilt = icmp slt i8 %self1, 0
  store i1 %ilt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @int8_sqrt(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %sitofp = sitofp i8 %self1 to float
  %intr = call float @llvm.sqrt.f32(float %sitofp)
  %fptosi = fptosi float %intr to i8
  store i8 %fptosi, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @int8_pow(i8 %0, i32 %1) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %sitofp = sitofp i8 %self1 to float
  %pow2 = load i32, ptr %pow, align 4
  %uitofp = uitofp i32 %pow2 to float
  %intr = call float @llvm.pow.f32(float %sitofp, float %uitofp)
  %fptosi = fptosi float %intr to i8
  store i8 %fptosi, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @int8_ln(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %sitofp = sitofp i8 %self1 to float
  %intr = call float @llvm.log.f32(float %sitofp)
  %fptosi = fptosi float %intr to i8
  store i8 %fptosi, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @int8_log2(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  %lns = alloca float, align 4
  %ln2 = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %sitofp = sitofp i8 %self1 to float
  %intr = call float @llvm.log.f32(float %sitofp)
  store float %intr, ptr %lns, align 4
  %intr2 = call float @llvm.log.f32(float 2.000000e+00)
  store float %intr2, ptr %ln2, align 4
  %lns3 = load float, ptr %lns, align 4
  %ln24 = load float, ptr %ln2, align 4
  %fdiv = fdiv float %lns3, %ln24
  %fptosi = fptosi float %fdiv to i8
  store i8 %fptosi, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @int8_log10(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  %lns = alloca float, align 4
  %ln10 = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %sitofp = sitofp i8 %self1 to float
  %intr = call float @llvm.log.f32(float %sitofp)
  store float %intr, ptr %lns, align 4
  %intr2 = call float @llvm.log.f32(float 1.000000e+01)
  store float %intr2, ptr %ln10, align 4
  %lns3 = load float, ptr %lns, align 4
  %ln104 = load float, ptr %ln10, align 4
  %fdiv = fdiv float %lns3, %ln104
  %fptosi = fptosi float %fdiv to i8
  store i8 %fptosi, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @int8_log(i8 %0, i8 %1) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %base = alloca i8, align 1
  store i8 %1, ptr %base, align 1
  %return_var = alloca i8, align 1
  %lns = alloca float, align 4
  %lnb = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %sitofp = sitofp i8 %self1 to float
  %intr = call float @llvm.log.f32(float %sitofp)
  store float %intr, ptr %lns, align 4
  %base2 = load i8, ptr %base, align 1
  %sitofp3 = sitofp i8 %base2 to float
  %intr4 = call float @llvm.log.f32(float %sitofp3)
  store float %intr4, ptr %lnb, align 4
  %lns5 = load float, ptr %lns, align 4
  %lnb6 = load float, ptr %lnb, align 4
  %fdiv = fdiv float %lns5, %lnb6
  %fptosi = fptosi float %fdiv to i8
  store i8 %fptosi, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @int16_abs(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %ilt = icmp slt i16 %self1, 0
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
  %self6 = load i16, ptr %self, align 2
  store i16 %self6, ptr %return_var, align 2
  br label %drop_and_return_

code_2:                                           ; preds = %then
  %self5 = load i16, ptr %self, align 2
  %ineg = sub i16 0, %self5
  store i16 %ineg, ptr %return_var, align 2
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i16 @int16_min(i16 %0, i16 %1) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %other = alloca i16, align 2
  store i16 %1, ptr %other, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %other2 = load i16, ptr %other, align 2
  %ilt = icmp slt i16 %self1, %other2
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
  %other7 = load i16, ptr %other, align 2
  store i16 %other7, ptr %return_var, align 2
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i16, ptr %self, align 2
  store i16 %self6, ptr %return_var, align 2
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i16 @int16_max(i16 %0, i16 %1) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %other = alloca i16, align 2
  store i16 %1, ptr %other, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %other2 = load i16, ptr %other, align 2
  %igt = icmp sgt i16 %self1, %other2
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
  %other7 = load i16, ptr %other, align 2
  store i16 %other7, ptr %return_var, align 2
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i16, ptr %self, align 2
  store i16 %self6, ptr %return_var, align 2
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i16 @int16_clamp(i16 %0, i16 %1, i16 %2) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %min = alloca i16, align 2
  store i16 %1, ptr %min, align 2
  %max = alloca i16, align 2
  store i16 %2, ptr %max, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %max2 = load i16, ptr %max, align 2
  %call = call i16 @int16_min(i16 %self1, i16 %max2)
  %min3 = load i16, ptr %min, align 2
  %call4 = call i16 @int16_max(i16 %call, i16 %min3)
  store i16 %call4, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int16_even(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %band = and i16 %self1, 1
  %ieq = icmp eq i16 %band, 0
  store i1 %ieq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int16_odd(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %band = and i16 %self1, 1
  %igt = icmp sgt i16 %band, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @int16_signum(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %ilt = icmp slt i16 %self1, 0
  br i1 %ilt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge7, %drop_and_return_10, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  %self8 = load i16, ptr %self, align 2
  %igt = icmp sgt i16 %self8, 0
  br i1 %igt, label %then5, label %else6

code_2:                                           ; preds = %then
  store i32 -1, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge

then5:                                            ; preds = %if_merge
  br label %code_9

else6:                                            ; preds = %if_merge
  br label %if_merge7

if_merge7:                                        ; preds = %else6, %drop_and_merge_11
  store i16 0, ptr %return_var, align 2
  br label %drop_and_return_

code_9:                                           ; preds = %then5
  store i16 1, ptr %return_var, align 2
  br label %drop_and_return_10

drop_and_return_10:                               ; preds = %code_9
  br label %drop_and_return_

drop_and_merge_11:                                ; No predecessors!
  br label %if_merge7
}

define i1 @int16_positive(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %igt = icmp sgt i16 %self1, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int16_negative(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %ilt = icmp slt i16 %self1, 0
  store i1 %ilt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @int16_sqrt(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %sitofp = sitofp i16 %self1 to float
  %intr = call float @llvm.sqrt.f32(float %sitofp)
  %fptosi = fptosi float %intr to i16
  store i16 %fptosi, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @int16_pow(i16 %0, i32 %1) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %sitofp = sitofp i16 %self1 to float
  %pow2 = load i32, ptr %pow, align 4
  %uitofp = uitofp i32 %pow2 to float
  %intr = call float @llvm.pow.f32(float %sitofp, float %uitofp)
  %fptosi = fptosi float %intr to i16
  store i16 %fptosi, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @int16_ln(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %sitofp = sitofp i16 %self1 to float
  %intr = call float @llvm.log.f32(float %sitofp)
  %fptosi = fptosi float %intr to i16
  store i16 %fptosi, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @int16_log2(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  %lns = alloca float, align 4
  %ln2 = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %sitofp = sitofp i16 %self1 to float
  %intr = call float @llvm.log.f32(float %sitofp)
  store float %intr, ptr %lns, align 4
  %intr2 = call float @llvm.log.f32(float 2.000000e+00)
  store float %intr2, ptr %ln2, align 4
  %lns3 = load float, ptr %lns, align 4
  %ln24 = load float, ptr %ln2, align 4
  %fdiv = fdiv float %lns3, %ln24
  %fptosi = fptosi float %fdiv to i16
  store i16 %fptosi, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @int16_log10(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  %lns = alloca float, align 4
  %ln10 = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %sitofp = sitofp i16 %self1 to float
  %intr = call float @llvm.log.f32(float %sitofp)
  store float %intr, ptr %lns, align 4
  %intr2 = call float @llvm.log.f32(float 1.000000e+01)
  store float %intr2, ptr %ln10, align 4
  %lns3 = load float, ptr %lns, align 4
  %ln104 = load float, ptr %ln10, align 4
  %fdiv = fdiv float %lns3, %ln104
  %fptosi = fptosi float %fdiv to i16
  store i16 %fptosi, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @int16_log(i16 %0, i16 %1) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %base = alloca i16, align 2
  store i16 %1, ptr %base, align 2
  %return_var = alloca i16, align 2
  %lns = alloca float, align 4
  %lnb = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %sitofp = sitofp i16 %self1 to float
  %intr = call float @llvm.log.f32(float %sitofp)
  store float %intr, ptr %lns, align 4
  %base2 = load i16, ptr %base, align 2
  %sitofp3 = sitofp i16 %base2 to float
  %intr4 = call float @llvm.log.f32(float %sitofp3)
  store float %intr4, ptr %lnb, align 4
  %lns5 = load float, ptr %lns, align 4
  %lnb6 = load float, ptr %lnb, align 4
  %fdiv = fdiv float %lns5, %lnb6
  %fptosi = fptosi float %fdiv to i16
  store i16 %fptosi, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @int32_abs(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %ilt = icmp slt i32 %self1, 0
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
  %self6 = load i32, ptr %self, align 4
  store i32 %self6, ptr %return_var, align 4
  br label %drop_and_return_

code_2:                                           ; preds = %then
  %self5 = load i32, ptr %self, align 4
  %ineg = sub i32 0, %self5
  store i32 %ineg, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i32 @int32_min(i32 %0, i32 %1) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %other = alloca i32, align 4
  store i32 %1, ptr %other, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %other2 = load i32, ptr %other, align 4
  %ilt = icmp slt i32 %self1, %other2
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
  %other7 = load i32, ptr %other, align 4
  store i32 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i32, ptr %self, align 4
  store i32 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i32 @int32_max(i32 %0, i32 %1) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %other = alloca i32, align 4
  store i32 %1, ptr %other, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %other2 = load i32, ptr %other, align 4
  %igt = icmp sgt i32 %self1, %other2
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
  %other7 = load i32, ptr %other, align 4
  store i32 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i32, ptr %self, align 4
  store i32 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i32 @int32_clamp(i32 %0, i32 %1, i32 %2) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %min = alloca i32, align 4
  store i32 %1, ptr %min, align 4
  %max = alloca i32, align 4
  store i32 %2, ptr %max, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %max2 = load i32, ptr %max, align 4
  %call = call i32 @int32_min(i32 %self1, i32 %max2)
  %min3 = load i32, ptr %min, align 4
  %call4 = call i32 @int32_max(i32 %call, i32 %min3)
  store i32 %call4, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int32_even(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %band = and i32 %self1, 1
  %ieq = icmp eq i32 %band, 0
  store i1 %ieq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int32_odd(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %band = and i32 %self1, 1
  %igt = icmp sgt i32 %band, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @int32_signum(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %ilt = icmp slt i32 %self1, 0
  br i1 %ilt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge7, %drop_and_return_10, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  %self8 = load i32, ptr %self, align 4
  %igt = icmp sgt i32 %self8, 0
  br i1 %igt, label %then5, label %else6

code_2:                                           ; preds = %then
  store i32 -1, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge

then5:                                            ; preds = %if_merge
  br label %code_9

else6:                                            ; preds = %if_merge
  br label %if_merge7

if_merge7:                                        ; preds = %else6, %drop_and_merge_11
  store i32 0, ptr %return_var, align 4
  br label %drop_and_return_

code_9:                                           ; preds = %then5
  store i32 1, ptr %return_var, align 4
  br label %drop_and_return_10

drop_and_return_10:                               ; preds = %code_9
  br label %drop_and_return_

drop_and_merge_11:                                ; No predecessors!
  br label %if_merge7
}

define i1 @int32_positive(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %igt = icmp sgt i32 %self1, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int32_negative(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %ilt = icmp slt i32 %self1, 0
  store i1 %ilt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @int32_sqrt(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %sitofp = sitofp i32 %self1 to double
  %intr = call double @llvm.sqrt.f64(double %sitofp)
  %fptosi = fptosi double %intr to i32
  store i32 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @int32_pow(i32 %0, i32 %1) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %sitofp = sitofp i32 %self1 to double
  %pow2 = load i32, ptr %pow, align 4
  %uitofp = uitofp i32 %pow2 to double
  %intr = call double @llvm.pow.f64(double %sitofp, double %uitofp)
  %fptosi = fptosi double %intr to i32
  store i32 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @int32_ln(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %sitofp = sitofp i32 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  %fptosi = fptosi double %intr to i32
  store i32 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @int32_log2(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  %lns = alloca double, align 8
  %ln2 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %sitofp = sitofp i32 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 2.000000e+00)
  store double %intr2, ptr %ln2, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln24 = load double, ptr %ln2, align 8
  %fdiv = fdiv double %lns3, %ln24
  %fptosi = fptosi double %fdiv to i32
  store i32 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @int32_log10(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  %lns = alloca double, align 8
  %ln10 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %sitofp = sitofp i32 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 1.000000e+01)
  store double %intr2, ptr %ln10, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln104 = load double, ptr %ln10, align 8
  %fdiv = fdiv double %lns3, %ln104
  %fptosi = fptosi double %fdiv to i32
  store i32 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @int32_log(i32 %0, i32 %1) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %base = alloca i32, align 4
  store i32 %1, ptr %base, align 4
  %return_var = alloca i32, align 4
  %lns = alloca double, align 8
  %lnb = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %sitofp = sitofp i32 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  store double %intr, ptr %lns, align 8
  %base2 = load i32, ptr %base, align 4
  %sitofp3 = sitofp i32 %base2 to double
  %intr4 = call double @llvm.log.f64(double %sitofp3)
  store double %intr4, ptr %lnb, align 8
  %lns5 = load double, ptr %lns, align 8
  %lnb6 = load double, ptr %lnb, align 8
  %fdiv = fdiv double %lns5, %lnb6
  %fptosi = fptosi double %fdiv to i32
  store i32 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @int64_abs(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ilt = icmp slt i64 %self1, 0
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
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_

code_2:                                           ; preds = %then
  %self5 = load i64, ptr %self, align 4
  %ineg = sub i64 0, %self5
  store i64 %ineg, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i64 @int64_min(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %other = alloca i64, align 8
  store i64 %1, ptr %other, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %other2 = load i64, ptr %other, align 4
  %ilt = icmp slt i64 %self1, %other2
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
  %other7 = load i64, ptr %other, align 4
  store i64 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i64 @int64_max(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %other = alloca i64, align 8
  store i64 %1, ptr %other, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %other2 = load i64, ptr %other, align 4
  %igt = icmp sgt i64 %self1, %other2
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
  %other7 = load i64, ptr %other, align 4
  store i64 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i64 @int64_clamp(i64 %0, i64 %1, i64 %2) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %min = alloca i64, align 8
  store i64 %1, ptr %min, align 4
  %max = alloca i64, align 8
  store i64 %2, ptr %max, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %max2 = load i64, ptr %max, align 4
  %call = call i64 @int64_min(i64 %self1, i64 %max2)
  %min3 = load i64, ptr %min, align 4
  %call4 = call i64 @int64_max(i64 %call, i64 %min3)
  store i64 %call4, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int64_even(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %band = and i64 %self1, 1
  %ieq = icmp eq i64 %band, 0
  store i1 %ieq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int64_odd(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %band = and i64 %self1, 1
  %igt = icmp sgt i64 %band, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @int64_signum(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ilt = icmp slt i64 %self1, 0
  br i1 %ilt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge7, %drop_and_return_10, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  %self8 = load i64, ptr %self, align 4
  %igt = icmp sgt i64 %self8, 0
  br i1 %igt, label %then5, label %else6

code_2:                                           ; preds = %then
  store i32 -1, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge

then5:                                            ; preds = %if_merge
  br label %code_9

else6:                                            ; preds = %if_merge
  br label %if_merge7

if_merge7:                                        ; preds = %else6, %drop_and_merge_11
  store i64 0, ptr %return_var, align 4
  br label %drop_and_return_

code_9:                                           ; preds = %then5
  store i64 1, ptr %return_var, align 4
  br label %drop_and_return_10

drop_and_return_10:                               ; preds = %code_9
  br label %drop_and_return_

drop_and_merge_11:                                ; No predecessors!
  br label %if_merge7
}

define i1 @int64_positive(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %igt = icmp sgt i64 %self1, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @int64_negative(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ilt = icmp slt i64 %self1, 0
  store i1 %ilt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @int64_sqrt(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.sqrt.f64(double %sitofp)
  %fptosi = fptosi double %intr to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @int64_pow(i64 %0, i32 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %pow2 = load i32, ptr %pow, align 4
  %uitofp = uitofp i32 %pow2 to double
  %intr = call double @llvm.pow.f64(double %sitofp, double %uitofp)
  %fptosi = fptosi double %intr to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @int64_ln(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  %fptosi = fptosi double %intr to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @int64_log2(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %ln2 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 2.000000e+00)
  store double %intr2, ptr %ln2, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln24 = load double, ptr %ln2, align 8
  %fdiv = fdiv double %lns3, %ln24
  %fptosi = fptosi double %fdiv to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @int64_log10(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %ln10 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 1.000000e+01)
  store double %intr2, ptr %ln10, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln104 = load double, ptr %ln10, align 8
  %fdiv = fdiv double %lns3, %ln104
  %fptosi = fptosi double %fdiv to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @int64_log(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %base = alloca i64, align 8
  store i64 %1, ptr %base, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %lnb = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  store double %intr, ptr %lns, align 8
  %base2 = load i64, ptr %base, align 4
  %sitofp3 = sitofp i64 %base2 to double
  %intr4 = call double @llvm.log.f64(double %sitofp3)
  store double %intr4, ptr %lnb, align 8
  %lns5 = load double, ptr %lns, align 8
  %lnb6 = load double, ptr %lnb, align 8
  %fdiv = fdiv double %lns5, %lnb6
  %fptosi = fptosi double %fdiv to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @uint8_abs(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  store i8 %self1, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @uint8_min(i8 %0, i8 %1) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %other = alloca i8, align 1
  store i8 %1, ptr %other, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %other2 = load i8, ptr %other, align 1
  %ult = icmp ult i8 %self1, %other2
  br i1 %ult, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %other7 = load i8, ptr %other, align 1
  store i8 %other7, ptr %return_var, align 1
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i8, ptr %self, align 1
  store i8 %self6, ptr %return_var, align 1
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i8 @uint8_max(i8 %0, i8 %1) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %other = alloca i8, align 1
  store i8 %1, ptr %other, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %other2 = load i8, ptr %other, align 1
  %ugt = icmp ugt i8 %self1, %other2
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
  %other7 = load i8, ptr %other, align 1
  store i8 %other7, ptr %return_var, align 1
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i8, ptr %self, align 1
  store i8 %self6, ptr %return_var, align 1
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i8 @uint8_clamp(i8 %0, i8 %1, i8 %2) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %min = alloca i8, align 1
  store i8 %1, ptr %min, align 1
  %max = alloca i8, align 1
  store i8 %2, ptr %max, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %max2 = load i8, ptr %max, align 1
  %call = call i8 @uint8_min(i8 %self1, i8 %max2)
  %min3 = load i8, ptr %min, align 1
  %call4 = call i8 @uint8_max(i8 %call, i8 %min3)
  store i8 %call4, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @uint8_even(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %band = and i8 %self1, 1
  %ueq = icmp eq i8 %band, 0
  store i1 %ueq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @uint8_odd(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %band = and i8 %self1, 1
  %ugt = icmp ugt i8 %band, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @uint8_signum(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %ugt = icmp ugt i8 %self1, 0
  br i1 %ugt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  store i8 0, ptr %return_var, align 1
  br label %drop_and_return_

code_2:                                           ; preds = %then
  store i8 1, ptr %return_var, align 1
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i1 @uint8_positive(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %ugt = icmp ugt i8 %self1, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @uint8_sqrt(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %uitofp = uitofp i8 %self1 to float
  %intr = call float @llvm.sqrt.f32(float %uitofp)
  %fptoui = fptoui float %intr to i8
  store i8 %fptoui, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @uint8_pow(i8 %0, i32 %1) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %uitofp = uitofp i8 %self1 to float
  %pow2 = load i32, ptr %pow, align 4
  %uitofp3 = uitofp i32 %pow2 to float
  %intr = call float @llvm.pow.f32(float %uitofp, float %uitofp3)
  %fptoui = fptoui float %intr to i8
  store i8 %fptoui, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @uint8_ln(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %uitofp = uitofp i8 %self1 to float
  %intr = call float @llvm.log.f32(float %uitofp)
  %fptoui = fptoui float %intr to i8
  store i8 %fptoui, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @uint8_log2(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  %lns = alloca float, align 4
  %ln2 = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %uitofp = uitofp i8 %self1 to float
  %intr = call float @llvm.log.f32(float %uitofp)
  store float %intr, ptr %lns, align 4
  %intr2 = call float @llvm.log.f32(float 2.000000e+00)
  store float %intr2, ptr %ln2, align 4
  %lns3 = load float, ptr %lns, align 4
  %ln24 = load float, ptr %ln2, align 4
  %fdiv = fdiv float %lns3, %ln24
  %fptoui = fptoui float %fdiv to i8
  store i8 %fptoui, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @uint8_log10(i8 %0) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %return_var = alloca i8, align 1
  %lns = alloca float, align 4
  %ln10 = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %uitofp = uitofp i8 %self1 to float
  %intr = call float @llvm.log.f32(float %uitofp)
  store float %intr, ptr %lns, align 4
  %intr2 = call float @llvm.log.f32(float 1.000000e+01)
  store float %intr2, ptr %ln10, align 4
  %lns3 = load float, ptr %lns, align 4
  %ln104 = load float, ptr %ln10, align 4
  %fdiv = fdiv float %lns3, %ln104
  %fptoui = fptoui float %fdiv to i8
  store i8 %fptoui, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @uint8_log(i8 %0, i8 %1) {
entry:
  %self = alloca i8, align 1
  store i8 %0, ptr %self, align 1
  %base = alloca i8, align 1
  store i8 %1, ptr %base, align 1
  %return_var = alloca i8, align 1
  %lns = alloca float, align 4
  %lnb = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %self1 = load i8, ptr %self, align 1
  %uitofp = uitofp i8 %self1 to float
  %intr = call float @llvm.log.f32(float %uitofp)
  store float %intr, ptr %lns, align 4
  %base2 = load i8, ptr %base, align 1
  %uitofp3 = uitofp i8 %base2 to float
  %intr4 = call float @llvm.log.f32(float %uitofp3)
  store float %intr4, ptr %lnb, align 4
  %lns5 = load float, ptr %lns, align 4
  %lnb6 = load float, ptr %lnb, align 4
  %fdiv = fdiv float %lns5, %lnb6
  %fptoui = fptoui float %fdiv to i8
  store i8 %fptoui, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @uint16_abs(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  store i16 %self1, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @uint16_min(i16 %0, i16 %1) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %other = alloca i16, align 2
  store i16 %1, ptr %other, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %other2 = load i16, ptr %other, align 2
  %ult = icmp ult i16 %self1, %other2
  br i1 %ult, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %other7 = load i16, ptr %other, align 2
  store i16 %other7, ptr %return_var, align 2
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i16, ptr %self, align 2
  store i16 %self6, ptr %return_var, align 2
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i16 @uint16_max(i16 %0, i16 %1) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %other = alloca i16, align 2
  store i16 %1, ptr %other, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %other2 = load i16, ptr %other, align 2
  %ugt = icmp ugt i16 %self1, %other2
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
  %other7 = load i16, ptr %other, align 2
  store i16 %other7, ptr %return_var, align 2
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i16, ptr %self, align 2
  store i16 %self6, ptr %return_var, align 2
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i16 @uint16_clamp(i16 %0, i16 %1, i16 %2) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %min = alloca i16, align 2
  store i16 %1, ptr %min, align 2
  %max = alloca i16, align 2
  store i16 %2, ptr %max, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %max2 = load i16, ptr %max, align 2
  %call = call i16 @uint16_min(i16 %self1, i16 %max2)
  %min3 = load i16, ptr %min, align 2
  %call4 = call i16 @uint16_max(i16 %call, i16 %min3)
  store i16 %call4, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @uint16_even(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %band = and i16 %self1, 1
  %ueq = icmp eq i16 %band, 0
  store i1 %ueq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @uint16_odd(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %band = and i16 %self1, 1
  %ugt = icmp ugt i16 %band, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @uint16_signum(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %ugt = icmp ugt i16 %self1, 0
  br i1 %ugt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  store i16 0, ptr %return_var, align 2
  br label %drop_and_return_

code_2:                                           ; preds = %then
  store i16 1, ptr %return_var, align 2
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i1 @uint16_positive(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %ugt = icmp ugt i16 %self1, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @uint16_sqrt(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %uitofp = uitofp i16 %self1 to float
  %intr = call float @llvm.sqrt.f32(float %uitofp)
  %fptoui = fptoui float %intr to i16
  store i16 %fptoui, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @uint16_pow(i16 %0, i32 %1) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %uitofp = uitofp i16 %self1 to float
  %pow2 = load i32, ptr %pow, align 4
  %uitofp3 = uitofp i32 %pow2 to float
  %intr = call float @llvm.pow.f32(float %uitofp, float %uitofp3)
  %fptoui = fptoui float %intr to i16
  store i16 %fptoui, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @uint16_ln(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %uitofp = uitofp i16 %self1 to float
  %intr = call float @llvm.log.f32(float %uitofp)
  %fptoui = fptoui float %intr to i16
  store i16 %fptoui, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @uint16_log2(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  %lns = alloca float, align 4
  %ln2 = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %uitofp = uitofp i16 %self1 to float
  %intr = call float @llvm.log.f32(float %uitofp)
  store float %intr, ptr %lns, align 4
  %intr2 = call float @llvm.log.f32(float 2.000000e+00)
  store float %intr2, ptr %ln2, align 4
  %lns3 = load float, ptr %lns, align 4
  %ln24 = load float, ptr %ln2, align 4
  %fdiv = fdiv float %lns3, %ln24
  %fptoui = fptoui float %fdiv to i16
  store i16 %fptoui, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @uint16_log10(i16 %0) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %return_var = alloca i16, align 2
  %lns = alloca float, align 4
  %ln10 = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %uitofp = uitofp i16 %self1 to float
  %intr = call float @llvm.log.f32(float %uitofp)
  store float %intr, ptr %lns, align 4
  %intr2 = call float @llvm.log.f32(float 1.000000e+01)
  store float %intr2, ptr %ln10, align 4
  %lns3 = load float, ptr %lns, align 4
  %ln104 = load float, ptr %ln10, align 4
  %fdiv = fdiv float %lns3, %ln104
  %fptoui = fptoui float %fdiv to i16
  store i16 %fptoui, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i16 @uint16_log(i16 %0, i16 %1) {
entry:
  %self = alloca i16, align 2
  store i16 %0, ptr %self, align 2
  %base = alloca i16, align 2
  store i16 %1, ptr %base, align 2
  %return_var = alloca i16, align 2
  %lns = alloca float, align 4
  %lnb = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i16, ptr %return_var, align 2
  ret i16 %return_val

code_:                                            ; preds = %entry
  %self1 = load i16, ptr %self, align 2
  %uitofp = uitofp i16 %self1 to float
  %intr = call float @llvm.log.f32(float %uitofp)
  store float %intr, ptr %lns, align 4
  %base2 = load i16, ptr %base, align 2
  %uitofp3 = uitofp i16 %base2 to float
  %intr4 = call float @llvm.log.f32(float %uitofp3)
  store float %intr4, ptr %lnb, align 4
  %lns5 = load float, ptr %lns, align 4
  %lnb6 = load float, ptr %lnb, align 4
  %fdiv = fdiv float %lns5, %lnb6
  %fptoui = fptoui float %fdiv to i16
  store i16 %fptoui, ptr %return_var, align 2
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @uint32_abs(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  store i32 %self1, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @uint32_min(i32 %0, i32 %1) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %other = alloca i32, align 4
  store i32 %1, ptr %other, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %other2 = load i32, ptr %other, align 4
  %ult = icmp ult i32 %self1, %other2
  br i1 %ult, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %other7 = load i32, ptr %other, align 4
  store i32 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i32, ptr %self, align 4
  store i32 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i32 @uint32_max(i32 %0, i32 %1) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %other = alloca i32, align 4
  store i32 %1, ptr %other, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %other2 = load i32, ptr %other, align 4
  %ugt = icmp ugt i32 %self1, %other2
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
  %other7 = load i32, ptr %other, align 4
  store i32 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i32, ptr %self, align 4
  store i32 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i32 @uint32_clamp(i32 %0, i32 %1, i32 %2) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %min = alloca i32, align 4
  store i32 %1, ptr %min, align 4
  %max = alloca i32, align 4
  store i32 %2, ptr %max, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %max2 = load i32, ptr %max, align 4
  %call = call i32 @uint32_min(i32 %self1, i32 %max2)
  %min3 = load i32, ptr %min, align 4
  %call4 = call i32 @uint32_max(i32 %call, i32 %min3)
  store i32 %call4, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @uint32_even(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %band = and i32 %self1, 1
  %ueq = icmp eq i32 %band, 0
  store i1 %ueq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @uint32_odd(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %band = and i32 %self1, 1
  %ugt = icmp ugt i32 %band, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @uint32_signum(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %ugt = icmp ugt i32 %self1, 0
  br i1 %ugt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  store i32 0, ptr %return_var, align 4
  br label %drop_and_return_

code_2:                                           ; preds = %then
  store i32 1, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i1 @uint32_positive(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %ugt = icmp ugt i32 %self1, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @uint32_sqrt(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %uitofp = uitofp i32 %self1 to double
  %intr = call double @llvm.sqrt.f64(double %uitofp)
  %fptoui = fptoui double %intr to i32
  store i32 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @uint32_pow(i32 %0, i32 %1) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %uitofp = uitofp i32 %self1 to double
  %pow2 = load i32, ptr %pow, align 4
  %uitofp3 = uitofp i32 %pow2 to double
  %intr = call double @llvm.pow.f64(double %uitofp, double %uitofp3)
  %fptoui = fptoui double %intr to i32
  store i32 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @uint32_ln(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %uitofp = uitofp i32 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  %fptoui = fptoui double %intr to i32
  store i32 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @uint32_log2(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  %lns = alloca double, align 8
  %ln2 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %uitofp = uitofp i32 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 2.000000e+00)
  store double %intr2, ptr %ln2, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln24 = load double, ptr %ln2, align 8
  %fdiv = fdiv double %lns3, %ln24
  %fptoui = fptoui double %fdiv to i32
  store i32 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @uint32_log10(i32 %0) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %return_var = alloca i32, align 4
  %lns = alloca double, align 8
  %ln10 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %uitofp = uitofp i32 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 1.000000e+01)
  store double %intr2, ptr %ln10, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln104 = load double, ptr %ln10, align 8
  %fdiv = fdiv double %lns3, %ln104
  %fptoui = fptoui double %fdiv to i32
  store i32 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i32 @uint32_log(i32 %0, i32 %1) {
entry:
  %self = alloca i32, align 4
  store i32 %0, ptr %self, align 4
  %base = alloca i32, align 4
  store i32 %1, ptr %base, align 4
  %return_var = alloca i32, align 4
  %lns = alloca double, align 8
  %lnb = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %self1 = load i32, ptr %self, align 4
  %uitofp = uitofp i32 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  store double %intr, ptr %lns, align 8
  %base2 = load i32, ptr %base, align 4
  %uitofp3 = uitofp i32 %base2 to double
  %intr4 = call double @llvm.log.f64(double %uitofp3)
  store double %intr4, ptr %lnb, align 8
  %lns5 = load double, ptr %lns, align 8
  %lnb6 = load double, ptr %lnb, align 8
  %fdiv = fdiv double %lns5, %lnb6
  %fptoui = fptoui double %fdiv to i32
  store i32 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @uint64_abs(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  store i64 %self1, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @uint64_min(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %other = alloca i64, align 8
  store i64 %1, ptr %other, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %other2 = load i64, ptr %other, align 4
  %ult = icmp ult i64 %self1, %other2
  br i1 %ult, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %other7 = load i64, ptr %other, align 4
  store i64 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i64 @uint64_max(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %other = alloca i64, align 8
  store i64 %1, ptr %other, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %other2 = load i64, ptr %other, align 4
  %ugt = icmp ugt i64 %self1, %other2
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
  %other7 = load i64, ptr %other, align 4
  store i64 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i64 @uint64_clamp(i64 %0, i64 %1, i64 %2) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %min = alloca i64, align 8
  store i64 %1, ptr %min, align 4
  %max = alloca i64, align 8
  store i64 %2, ptr %max, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %max2 = load i64, ptr %max, align 4
  %call = call i64 @uint64_min(i64 %self1, i64 %max2)
  %min3 = load i64, ptr %min, align 4
  %call4 = call i64 @uint64_max(i64 %call, i64 %min3)
  store i64 %call4, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @uint64_even(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %band = and i64 %self1, 1
  %ueq = icmp eq i64 %band, 0
  store i1 %ueq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @uint64_odd(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %band = and i64 %self1, 1
  %ugt = icmp ugt i64 %band, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @uint64_signum(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ugt = icmp ugt i64 %self1, 0
  br i1 %ugt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  store i64 0, ptr %return_var, align 4
  br label %drop_and_return_

code_2:                                           ; preds = %then
  store i64 1, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i1 @uint64_positive(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ugt = icmp ugt i64 %self1, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @uint64_sqrt(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.sqrt.f64(double %uitofp)
  %fptoui = fptoui double %intr to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @uint64_pow(i64 %0, i32 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %pow2 = load i32, ptr %pow, align 4
  %uitofp3 = uitofp i32 %pow2 to double
  %intr = call double @llvm.pow.f64(double %uitofp, double %uitofp3)
  %fptoui = fptoui double %intr to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @uint64_ln(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  %fptoui = fptoui double %intr to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @uint64_log2(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %ln2 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 2.000000e+00)
  store double %intr2, ptr %ln2, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln24 = load double, ptr %ln2, align 8
  %fdiv = fdiv double %lns3, %ln24
  %fptoui = fptoui double %fdiv to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @uint64_log10(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %ln10 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 1.000000e+01)
  store double %intr2, ptr %ln10, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln104 = load double, ptr %ln10, align 8
  %fdiv = fdiv double %lns3, %ln104
  %fptoui = fptoui double %fdiv to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @uint64_log(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %base = alloca i64, align 8
  store i64 %1, ptr %base, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %lnb = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  store double %intr, ptr %lns, align 8
  %base2 = load i64, ptr %base, align 4
  %uitofp3 = uitofp i64 %base2 to double
  %intr4 = call double @llvm.log.f64(double %uitofp3)
  store double %intr4, ptr %lnb, align 8
  %lns5 = load double, ptr %lns, align 8
  %lnb6 = load double, ptr %lnb, align 8
  %fdiv = fdiv double %lns5, %lnb6
  %fptoui = fptoui double %fdiv to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @isize_abs(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ilt = icmp slt i64 %self1, 0
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
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_

code_2:                                           ; preds = %then
  %self5 = load i64, ptr %self, align 4
  %ineg = sub i64 0, %self5
  store i64 %ineg, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i64 @isize_min(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %other = alloca i64, align 8
  store i64 %1, ptr %other, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %other2 = load i64, ptr %other, align 4
  %ilt = icmp slt i64 %self1, %other2
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
  %other7 = load i64, ptr %other, align 4
  store i64 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i64 @isize_max(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %other = alloca i64, align 8
  store i64 %1, ptr %other, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %other2 = load i64, ptr %other, align 4
  %igt = icmp sgt i64 %self1, %other2
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
  %other7 = load i64, ptr %other, align 4
  store i64 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i64 @isize_clamp(i64 %0, i64 %1, i64 %2) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %min = alloca i64, align 8
  store i64 %1, ptr %min, align 4
  %max = alloca i64, align 8
  store i64 %2, ptr %max, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %max2 = load i64, ptr %max, align 4
  %call = call i64 @isize_min(i64 %self1, i64 %max2)
  %min3 = load i64, ptr %min, align 4
  %call4 = call i64 @isize_max(i64 %call, i64 %min3)
  store i64 %call4, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @isize_even(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %band = and i64 %self1, 1
  %ieq = icmp eq i64 %band, 0
  store i1 %ieq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @isize_odd(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %band = and i64 %self1, 1
  %igt = icmp sgt i64 %band, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @isize_signum(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ilt = icmp slt i64 %self1, 0
  br i1 %ilt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge7, %drop_and_return_10, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  %self8 = load i64, ptr %self, align 4
  %igt = icmp sgt i64 %self8, 0
  br i1 %igt, label %then5, label %else6

code_2:                                           ; preds = %then
  store i32 -1, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge

then5:                                            ; preds = %if_merge
  br label %code_9

else6:                                            ; preds = %if_merge
  br label %if_merge7

if_merge7:                                        ; preds = %else6, %drop_and_merge_11
  store i64 0, ptr %return_var, align 4
  br label %drop_and_return_

code_9:                                           ; preds = %then5
  store i64 1, ptr %return_var, align 4
  br label %drop_and_return_10

drop_and_return_10:                               ; preds = %code_9
  br label %drop_and_return_

drop_and_merge_11:                                ; No predecessors!
  br label %if_merge7
}

define i1 @isize_positive(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %igt = icmp sgt i64 %self1, 0
  store i1 %igt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @isize_negative(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ilt = icmp slt i64 %self1, 0
  store i1 %ilt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @isize_sqrt(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.sqrt.f64(double %sitofp)
  %fptosi = fptosi double %intr to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @isize_pow(i64 %0, i32 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %pow2 = load i32, ptr %pow, align 4
  %uitofp = uitofp i32 %pow2 to double
  %intr = call double @llvm.pow.f64(double %sitofp, double %uitofp)
  %fptosi = fptosi double %intr to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @isize_ln(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  %fptosi = fptosi double %intr to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @isize_log2(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %ln2 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 2.000000e+00)
  store double %intr2, ptr %ln2, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln24 = load double, ptr %ln2, align 8
  %fdiv = fdiv double %lns3, %ln24
  %fptosi = fptosi double %fdiv to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @isize_log10(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %ln10 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 1.000000e+01)
  store double %intr2, ptr %ln10, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln104 = load double, ptr %ln10, align 8
  %fdiv = fdiv double %lns3, %ln104
  %fptosi = fptosi double %fdiv to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @isize_log(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %base = alloca i64, align 8
  store i64 %1, ptr %base, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %lnb = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %sitofp = sitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %sitofp)
  store double %intr, ptr %lns, align 8
  %base2 = load i64, ptr %base, align 4
  %sitofp3 = sitofp i64 %base2 to double
  %intr4 = call double @llvm.log.f64(double %sitofp3)
  store double %intr4, ptr %lnb, align 8
  %lns5 = load double, ptr %lns, align 8
  %lnb6 = load double, ptr %lnb, align 8
  %fdiv = fdiv double %lns5, %lnb6
  %fptosi = fptosi double %fdiv to i64
  store i64 %fptosi, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @usize_abs(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  store i64 %self1, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @usize_min(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %other = alloca i64, align 8
  store i64 %1, ptr %other, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %other2 = load i64, ptr %other, align 4
  %ult = icmp ult i64 %self1, %other2
  br i1 %ult, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %other7 = load i64, ptr %other, align 4
  store i64 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i64 @usize_max(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %other = alloca i64, align 8
  store i64 %1, ptr %other, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %other2 = load i64, ptr %other, align 4
  %ugt = icmp ugt i64 %self1, %other2
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
  %other7 = load i64, ptr %other, align 4
  store i64 %other7, ptr %return_var, align 4
  br label %drop_and_return_

code_3:                                           ; preds = %then
  %self6 = load i64, ptr %self, align 4
  store i64 %self6, ptr %return_var, align 4
  br label %drop_and_return_4

drop_and_return_4:                                ; preds = %code_3
  br label %drop_and_return_

drop_and_merge_5:                                 ; No predecessors!
  br label %if_merge
}

define i64 @usize_clamp(i64 %0, i64 %1, i64 %2) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %min = alloca i64, align 8
  store i64 %1, ptr %min, align 4
  %max = alloca i64, align 8
  store i64 %2, ptr %max, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %max2 = load i64, ptr %max, align 4
  %call = call i64 @usize_min(i64 %self1, i64 %max2)
  %min3 = load i64, ptr %min, align 4
  %call4 = call i64 @usize_max(i64 %call, i64 %min3)
  store i64 %call4, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @usize_even(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %band = and i64 %self1, 1
  %ueq = icmp eq i64 %band, 0
  store i1 %ueq, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @usize_odd(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %band = and i64 %self1, 1
  %ugt = icmp ugt i64 %band, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @usize_signum(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ugt = icmp ugt i64 %self1, 0
  br i1 %ugt, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  store i64 0, ptr %return_var, align 4
  br label %drop_and_return_

code_2:                                           ; preds = %then
  store i64 1, ptr %return_var, align 4
  br label %drop_and_return_3

drop_and_return_3:                                ; preds = %code_2
  br label %drop_and_return_

drop_and_merge_4:                                 ; No predecessors!
  br label %if_merge
}

define i1 @usize_positive(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %ugt = icmp ugt i64 %self1, 0
  store i1 %ugt, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @usize_sqrt(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.sqrt.f64(double %uitofp)
  %fptoui = fptoui double %intr to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @usize_pow(i64 %0, i32 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %pow = alloca i32, align 4
  store i32 %1, ptr %pow, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %pow2 = load i32, ptr %pow, align 4
  %uitofp3 = uitofp i32 %pow2 to double
  %intr = call double @llvm.pow.f64(double %uitofp, double %uitofp3)
  %fptoui = fptoui double %intr to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @usize_ln(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  %fptoui = fptoui double %intr to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @usize_log2(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %ln2 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 2.000000e+00)
  store double %intr2, ptr %ln2, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln24 = load double, ptr %ln2, align 8
  %fdiv = fdiv double %lns3, %ln24
  %fptoui = fptoui double %fdiv to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @usize_log10(i64 %0) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %ln10 = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  store double %intr, ptr %lns, align 8
  %intr2 = call double @llvm.log.f64(double 1.000000e+01)
  store double %intr2, ptr %ln10, align 8
  %lns3 = load double, ptr %lns, align 8
  %ln104 = load double, ptr %ln10, align 8
  %fdiv = fdiv double %lns3, %ln104
  %fptoui = fptoui double %fdiv to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i64 @usize_log(i64 %0, i64 %1) {
entry:
  %self = alloca i64, align 8
  store i64 %0, ptr %self, align 4
  %base = alloca i64, align 8
  store i64 %1, ptr %base, align 4
  %return_var = alloca i64, align 8
  %lns = alloca double, align 8
  %lnb = alloca double, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i64, ptr %return_var, align 4
  ret i64 %return_val

code_:                                            ; preds = %entry
  %self1 = load i64, ptr %self, align 4
  %uitofp = uitofp i64 %self1 to double
  %intr = call double @llvm.log.f64(double %uitofp)
  store double %intr, ptr %lns, align 8
  %base2 = load i64, ptr %base, align 4
  %uitofp3 = uitofp i64 %base2 to double
  %intr4 = call double @llvm.log.f64(double %uitofp3)
  store double %intr4, ptr %lnb, align 8
  %lns5 = load double, ptr %lns, align 8
  %lnb6 = load double, ptr %lnb, align 8
  %fdiv = fdiv double %lns5, %lnb6
  %fptoui = fptoui double %fdiv to i64
  store i64 %fptoui, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

declare void @cprint(<{ i64, ptr }>)

declare ptr @aligned_alloc(i64, i64)

declare void @free(ptr)

define i8 @main() {
entry:
  %return_var = alloca i8, align 1
  %i = alloca i32, align 4
  %i2 = alloca i32, align 4
  %v = alloca ptr, align 8
  %ray = alloca <{ i64, ptr }>, align 8
  %i5 = alloca i32, align 4
  %res = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  store i32 35, ptr %i, align 4
  %i1 = load i32, ptr %i, align 4
  %call = call i32 @int32_signum(i32 %i1)
  store i32 %call, ptr %i2, align 4
  %call3 = call ptr @aligned_alloc(i64 4, i64 40)
  store ptr %call3, ptr %v, align 8
  %v4 = load ptr, ptr %v, align 8
  %sl_ptr = insertvalue <{ i64, ptr }> <{ i64 10, ptr undef }>, ptr %v4, 1
  store <{ i64, ptr }> %sl_ptr, ptr %ray, align 1
  store i32 0, ptr %i5, align 4
  br label %loop_start

drop_and_return_:                                 ; preds = %loop_merge, %drop_and_return_8
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

loop_start:                                       ; preds = %drop_and_merge_9, %code_
  %i6 = load i32, ptr %i5, align 4
  %ilt = icmp slt i32 %i6, 10
  br i1 %ilt, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_7

loop_merge:                                       ; preds = %loop_start
  %string_literal = load <{ i64, ptr }>, ptr @string_literal, align 1
  call void @cprint(<{ i64, ptr }> %string_literal)
  %ray14 = load <{ i64, ptr }>, ptr %ray, align 1
  %sl_ptr15 = extractvalue <{ i64, ptr }> %ray14, 1
  %sl_elem_ptr16 = getelementptr i32, ptr %sl_ptr15, i64 4
  %refread = load i32, ptr %sl_elem_ptr16, align 4
  store i32 %refread, ptr %res, align 4
  %v17 = load ptr, ptr %v, align 8
  call void @free(ptr %v17)
  %res18 = load i32, ptr %res, align 4
  %trunc = trunc i32 %res18 to i8
  store i8 %trunc, ptr %return_var, align 1
  br label %drop_and_return_

code_7:                                           ; preds = %loop
  %i10 = load i32, ptr %i5, align 4
  %ray11 = load <{ i64, ptr }>, ptr %ray, align 1
  %i12 = load i32, ptr %i5, align 4
  %sext = sext i32 %i12 to i64
  %sl_ptr13 = extractvalue <{ i64, ptr }> %ray11, 1
  %sl_elem_ptr = getelementptr i32, ptr %sl_ptr13, i64 %sext
  store i32 %i10, ptr %sl_elem_ptr, align 4
  %ca_load = load i32, ptr %i5, align 4
  %iadd = add i32 %ca_load, 1
  store i32 %iadd, ptr %i5, align 4
  br label %drop_and_merge_9

drop_and_return_8:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_9:                                 ; preds = %code_7
  br label %loop_start
}

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.sqrt.f32(float) #0

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.pow.f32(float, float) #0

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.log.f32(float) #0

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.sqrt.f64(double) #0

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.pow.f64(double, double) #0

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.log.f64(double) #0

attributes #0 = { nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none) }
