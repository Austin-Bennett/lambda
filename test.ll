; ModuleID = 'lambda_program'
source_filename = "lambda_program"

declare ptr @malloc(i64)

declare void @free(ptr)

declare ptr @realloc(ptr, i64)

define { ptr, i64, i64 } @IntList_new() {
entry:
  %return_var = alloca { ptr, i64, i64 }, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load { ptr, i64, i64 }, ptr %return_var, align 8
  ret { ptr, i64, i64 } %return_val

code_:                                            ; preds = %entry
  %call = call ptr @malloc(i64 4)
  %sf = insertvalue { ptr, i64, i64 } undef, ptr %call, 0
  %sf1 = insertvalue { ptr, i64, i64 } %sf, i64 1, 1
  %sf2 = insertvalue { ptr, i64, i64 } %sf1, i64 0, 2
  store { ptr, i64, i64 } %sf2, ptr %return_var, align 8
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define { ptr, i64, i64 } @IntList_with_capacity(i64 %0) {
entry:
  %cap = alloca i64, align 8
  store i64 %0, ptr %cap, align 4
  %return_var = alloca { ptr, i64, i64 }, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load { ptr, i64, i64 }, ptr %return_var, align 8
  ret { ptr, i64, i64 } %return_val

code_:                                            ; preds = %entry
  %cap1 = load i64, ptr %cap, align 4
  %umul = mul i64 4, %cap1
  %call = call ptr @malloc(i64 %umul)
  %sf = insertvalue { ptr, i64, i64 } undef, ptr %call, 0
  %cap2 = load i64, ptr %cap, align 4
  %sf3 = insertvalue { ptr, i64, i64 } %sf, i64 %cap2, 1
  %sf4 = insertvalue { ptr, i64, i64 } %sf3, i64 0, 2
  store { ptr, i64, i64 } %sf4, ptr %return_var, align 8
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @IntList_add(ptr %0, i32 %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %e = alloca i32, align 4
  store i32 %1, ptr %e, align 4
  %new_size = alloca i64, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { ptr, i64, i64 }, ptr %self1, align 8
  %member = extractvalue { ptr, i64, i64 } %refread, 2
  %self2 = load ptr, ptr %self, align 8
  %refread3 = load { ptr, i64, i64 }, ptr %self2, align 8
  %member4 = extractvalue { ptr, i64, i64 } %refread3, 1
  %uge = icmp uge i64 %member, %member4
  br i1 %uge, label %then, label %else

drop_and_return_:                                 ; preds = %drop_and_return_6
  br label %return

drop_and_merge_:                                  ; preds = %if_merge
  br label %return

then:                                             ; preds = %code_
  br label %code_5

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_7
  %e20 = load i32, ptr %e, align 4
  %self21 = load ptr, ptr %self, align 8
  %refread22 = load { ptr, i64, i64 }, ptr %self21, align 8
  %member23 = extractvalue { ptr, i64, i64 } %refread22, 0
  %self24 = load ptr, ptr %self, align 8
  %refread25 = load { ptr, i64, i64 }, ptr %self24, align 8
  %member26 = extractvalue { ptr, i64, i64 } %refread25, 2
  %ptr_add = getelementptr i8, ptr %member23, i64 %member26
  store i32 %e20, ptr %ptr_add, align 4
  %self27 = load ptr, ptr %self, align 8
  %refread28 = load { ptr, i64, i64 }, ptr %self27, align 8
  %member29 = extractvalue { ptr, i64, i64 } %refread28, 2
  %uadd30 = add i64 %member29, 1
  %self31 = load ptr, ptr %self, align 8
  %field_ptr32 = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self31, i32 0, i32 2
  store i64 %uadd30, ptr %field_ptr32, align 4
  br label %drop_and_merge_

code_5:                                           ; preds = %then
  %self8 = load ptr, ptr %self, align 8
  %refread9 = load { ptr, i64, i64 }, ptr %self8, align 8
  %member10 = extractvalue { ptr, i64, i64 } %refread9, 2
  %umul = mul i64 %member10, 2
  %uadd = add i64 %umul, 1
  store i64 %uadd, ptr %new_size, align 4
  %self11 = load ptr, ptr %self, align 8
  %refread12 = load { ptr, i64, i64 }, ptr %self11, align 8
  %member13 = extractvalue { ptr, i64, i64 } %refread12, 0
  %new_size14 = load i64, ptr %new_size, align 4
  %umul15 = mul i64 %new_size14, 4
  %call = call ptr @realloc(ptr %member13, i64 %umul15)
  %self16 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self16, i32 0, i32 0
  store ptr %call, ptr %field_ptr, align 8
  %new_size17 = load i64, ptr %new_size, align 4
  %self18 = load ptr, ptr %self, align 8
  %field_ptr19 = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self18, i32 0, i32 1
  store i64 %new_size17, ptr %field_ptr19, align 4
  br label %drop_and_merge_7

drop_and_return_6:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_7:                                 ; preds = %code_5
  br label %if_merge
}

define ptr @IntList_get(ptr %0, i64 %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %index = alloca i64, align 8
  store i64 %1, ptr %index, align 4
  %return_var = alloca ptr, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load ptr, ptr %return_var, align 8
  ret ptr %return_val

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { ptr, i64, i64 }, ptr %self1, align 8
  %member = extractvalue { ptr, i64, i64 } %refread, 0
  %index2 = load i64, ptr %index, align 4
  %ptr_add = getelementptr i8, ptr %member, i64 %index2
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define void @IntList__op_drop(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load { ptr, i64, i64 }, ptr %self1, align 8
  %member = extractvalue { ptr, i64, i64 } %refread, 0
  call void @free(ptr %member)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define i8 @main() {
entry:
  %return_var = alloca i8, align 1
  %list = alloca { ptr, i64, i64 }, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  br label %loop_start

drop_and_return_:                                 ; preds = %drop_and_return_2
  br label %return

drop_and_merge_:                                  ; preds = %loop_merge
  br label %return

loop_start:                                       ; preds = %drop_and_merge_3, %code_
  br i1 true, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_1

loop_merge:                                       ; preds = %loop_start
  br label %drop_and_merge_

code_1:                                           ; preds = %loop
  %call = call { ptr, i64, i64 } @IntList_with_capacity(i64 100)
  store { ptr, i64, i64 } %call, ptr %list, align 8
  br label %drop_and_merge_3

drop_and_return_2:                                ; No predecessors!
  call void @IntList__op_drop(ptr %list)
  br label %drop_and_return_

drop_and_merge_3:                                 ; preds = %code_1
  call void @IntList__op_drop(ptr %list)
  br label %loop_start
}
