; ModuleID = 'lambda_program'
source_filename = "lambda_program"

define { i32 } @Counter_new() {
entry:
  %return_var = alloca { i32 }, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load { i32 }, ptr %return_var, align 4
  ret { i32 } %return_val

code_:                                            ; preds = %entry
  store { i32 } zeroinitializer, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
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
  %field_ptr = getelementptr inbounds nuw { i32 }, ptr %self1, i32 0, i32 0
  %ca_load = load i32, ptr %field_ptr, align 4
  %iadd = add i32 %ca_load, 1
  store i32 %iadd, ptr %field_ptr, align 4
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define i32 @plusone(i32 %0) {
entry:
  %a = alloca i32, align 4
  store i32 %0, ptr %a, align 4
  %return_var = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %a1 = load i32, ptr %a, align 4
  %iadd = add i32 %a1, 1
  store i32 %iadd, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i8 @main() {
entry:
  %return_var = alloca i8, align 1
  %my_list = alloca { ptr, i64, i64 }, align 8
  %my_list2 = alloca { ptr, i64, i64 }, align 8
  %my_func = alloca ptr, align 8
  %my_generic_func = alloca ptr, align 8
  %my_static_method_func = alloca ptr, align 8
  %my_method_func = alloca ptr, align 8
  %my_generic_method = alloca ptr, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  %call = call { ptr, i64, i64 } @List_int32_new()
  store { ptr, i64, i64 } %call, ptr %my_list, align 8
  %call1 = call { ptr, i64, i64 } @List_int32_new()
  call void @List_int32__op_drop(ptr %my_list)
  store { ptr, i64, i64 } %call1, ptr %my_list2, align 8
  store i8 0, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

declare ptr @aligned_alloc(i64, i64)

declare void @free(ptr)

declare void @exit(i32)

define { ptr, i64, i64 } @List_int32_new() {
entry:
  %return_var = alloca { ptr, i64, i64 }, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load { ptr, i64, i64 }, ptr %return_var, align 8
  ret { ptr, i64, i64 } %return_val

code_:                                            ; preds = %entry
  store { ptr, i64, i64 } zeroinitializer, ptr %return_var, align 8
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @List_int32_add(ptr %0, i32 %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %val = alloca i32, align 4
  store i32 %1, ptr %val, align 4
  %new_size = alloca i64, align 8
  %new_alloc = alloca ptr, align 8
  %new_alloc_slice = alloca <{ i64, ptr }>, align 8
  %ray_slice = alloca <{ i64, ptr }>, align 8
  %i = alloca i64, align 8
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
  %ueq = icmp eq i64 %member, %member4
  br i1 %ueq, label %then, label %else

drop_and_return_:                                 ; preds = %drop_and_return_6
  br label %return

drop_and_merge_:                                  ; preds = %if_merge
  br label %return

then:                                             ; preds = %code_
  br label %code_5

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_7
  %val54 = load i32, ptr %val, align 4
  %self55 = load ptr, ptr %self, align 8
  %refread56 = load { ptr, i64, i64 }, ptr %self55, align 8
  %member57 = extractvalue { ptr, i64, i64 } %refread56, 0
  %self58 = load ptr, ptr %self, align 8
  %refread59 = load { ptr, i64, i64 }, ptr %self58, align 8
  %member60 = extractvalue { ptr, i64, i64 } %refread59, 2
  %ptr_add = getelementptr i8, ptr %member57, i64 %member60
  store i32 %val54, ptr %ptr_add, align 4
  %self61 = load ptr, ptr %self, align 8
  %field_ptr62 = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self61, i32 0, i32 2
  %ca_load = load i64, ptr %field_ptr62, align 4
  %uadd63 = add i64 %ca_load, 1
  store i64 %uadd63, ptr %field_ptr62, align 4
  br label %drop_and_merge_

code_5:                                           ; preds = %then
  %self8 = load ptr, ptr %self, align 8
  %refread9 = load { ptr, i64, i64 }, ptr %self8, align 8
  %member10 = extractvalue { ptr, i64, i64 } %refread9, 2
  %umul = mul i64 %member10, 2
  %uadd = add i64 %umul, 1
  store i64 %uadd, ptr %new_size, align 4
  %new_size11 = load i64, ptr %new_size, align 4
  %call = call ptr @alloc_int32(i64 %new_size11)
  store ptr %call, ptr %new_alloc, align 8
  %new_size12 = load i64, ptr %new_size, align 4
  %new_alloc13 = load ptr, ptr %new_alloc, align 8
  %sl_len = insertvalue <{ i64, ptr }> undef, i64 %new_size12, 0
  %sl_ptr = insertvalue <{ i64, ptr }> %sl_len, ptr %new_alloc13, 1
  store <{ i64, ptr }> %sl_ptr, ptr %new_alloc_slice, align 1
  %self17 = load ptr, ptr %self, align 8
  %refread18 = load { ptr, i64, i64 }, ptr %self17, align 8
  %member19 = extractvalue { ptr, i64, i64 } %refread18, 0
  %pi = ptrtoint ptr %member19 to i64
  %pne = icmp ne i64 %pi, 0
  br i1 %pne, label %then14, label %else15

drop_and_return_6:                                ; preds = %drop_and_return_21
  br label %drop_and_return_

drop_and_merge_7:                                 ; preds = %if_merge16
  br label %if_merge

then14:                                           ; preds = %code_5
  br label %code_20

else15:                                           ; preds = %code_5
  br label %if_merge16

if_merge16:                                       ; preds = %else15, %drop_and_merge_22
  %new_size49 = load i64, ptr %new_size, align 4
  %self50 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self50, i32 0, i32 1
  store i64 %new_size49, ptr %field_ptr, align 4
  %new_alloc51 = load ptr, ptr %new_alloc, align 8
  %self52 = load ptr, ptr %self, align 8
  %field_ptr53 = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self52, i32 0, i32 0
  store ptr %new_alloc51, ptr %field_ptr53, align 8
  br label %drop_and_merge_7

code_20:                                          ; preds = %then14
  %self23 = load ptr, ptr %self, align 8
  %refread24 = load { ptr, i64, i64 }, ptr %self23, align 8
  %member25 = extractvalue { ptr, i64, i64 } %refread24, 2
  %self26 = load ptr, ptr %self, align 8
  %refread27 = load { ptr, i64, i64 }, ptr %self26, align 8
  %member28 = extractvalue { ptr, i64, i64 } %refread27, 0
  %sl_len29 = insertvalue <{ i64, ptr }> undef, i64 %member25, 0
  %sl_ptr30 = insertvalue <{ i64, ptr }> %sl_len29, ptr %member28, 1
  store <{ i64, ptr }> %sl_ptr30, ptr %ray_slice, align 1
  store i64 0, ptr %i, align 4
  br label %loop_start

drop_and_return_21:                               ; preds = %drop_and_return_36
  br label %drop_and_return_6

drop_and_merge_22:                                ; preds = %loop_merge
  br label %if_merge16

loop_start:                                       ; preds = %drop_and_merge_37, %code_20
  %i31 = load i64, ptr %i, align 4
  %self32 = load ptr, ptr %self, align 8
  %refread33 = load { ptr, i64, i64 }, ptr %self32, align 8
  %member34 = extractvalue { ptr, i64, i64 } %refread33, 2
  %ult = icmp ult i64 %i31, %member34
  br i1 %ult, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_35

loop_merge:                                       ; preds = %loop_start
  %self46 = load ptr, ptr %self, align 8
  %refread47 = load { ptr, i64, i64 }, ptr %self46, align 8
  %member48 = extractvalue { ptr, i64, i64 } %refread47, 0
  call void @dealloc_int32(ptr %member48)
  br label %drop_and_merge_22

code_35:                                          ; preds = %loop
  %ray_slice38 = load <{ i64, ptr }>, ptr %ray_slice, align 1
  %i39 = load i64, ptr %i, align 4
  %sl_ptr40 = extractvalue <{ i64, ptr }> %ray_slice38, 1
  %sl_elem_ptr = getelementptr i32, ptr %sl_ptr40, i64 %i39
  %refread41 = load i32, ptr %sl_elem_ptr, align 4
  %new_alloc_slice42 = load <{ i64, ptr }>, ptr %new_alloc_slice, align 1
  %i43 = load i64, ptr %i, align 4
  %sl_ptr44 = extractvalue <{ i64, ptr }> %new_alloc_slice42, 1
  %sl_elem_ptr45 = getelementptr i32, ptr %sl_ptr44, i64 %i43
  store i32 %refread41, ptr %sl_elem_ptr45, align 4
  br label %drop_and_merge_37

drop_and_return_36:                               ; No predecessors!
  br label %drop_and_return_21

drop_and_merge_37:                                ; preds = %code_35
  br label %loop_start
}

define ptr @List_int32_get(ptr %0, i64 %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %ind = alloca i64, align 8
  store i64 %1, ptr %ind, align 4
  %return_var = alloca ptr, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load ptr, ptr %return_var, align 8
  ret ptr %return_val

code_:                                            ; preds = %entry
  %ind1 = load i64, ptr %ind, align 4
  %self2 = load ptr, ptr %self, align 8
  %refread = load { ptr, i64, i64 }, ptr %self2, align 8
  %member = extractvalue { ptr, i64, i64 } %refread, 2
  %uge = icmp uge i64 %ind1, %member
  br i1 %uge, label %then, label %else

drop_and_return_:                                 ; preds = %if_merge, %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

then:                                             ; preds = %code_
  br label %code_3

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_5
  %self6 = load ptr, ptr %self, align 8
  %refread7 = load { ptr, i64, i64 }, ptr %self6, align 8
  %member8 = extractvalue { ptr, i64, i64 } %refread7, 0
  %ind9 = load i64, ptr %ind, align 4
  %ptr_add = getelementptr i8, ptr %member8, i64 %ind9
  store ptr %ptr_add, ptr %return_var, align 8
  br label %drop_and_return_

code_3:                                           ; preds = %then
  call void @exit(i32 3)
  br label %drop_and_merge_5

drop_and_return_4:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_5:                                 ; preds = %code_3
  br label %if_merge
}

define void @List_int32__op_drop(ptr %0) {
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
  %pi = ptrtoint ptr %member to i64
  %pne = icmp ne i64 %pi, 0
  br i1 %pne, label %then, label %else

drop_and_return_:                                 ; preds = %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; preds = %if_merge
  br label %return

then:                                             ; preds = %code_
  br label %code_2

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_4
  br label %drop_and_merge_

code_2:                                           ; preds = %then
  %self5 = load ptr, ptr %self, align 8
  %refread6 = load { ptr, i64, i64 }, ptr %self5, align 8
  %member7 = extractvalue { ptr, i64, i64 } %refread6, 0
  call void @dealloc_int32(ptr %member7)
  br label %drop_and_merge_4

drop_and_return_3:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_4:                                 ; preds = %code_2
  br label %if_merge
}

define ptr @alloc_int32(i64 %0) {
entry:
  %count = alloca i64, align 8
  store i64 %0, ptr %count, align 4
  %return_var = alloca ptr, align 8
  %ptr = alloca ptr, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load ptr, ptr %return_var, align 8
  ret ptr %return_val

code_:                                            ; preds = %entry
  %count1 = load i64, ptr %count, align 4
  %umul = mul i64 4, %count1
  %call = call ptr @aligned_alloc(i64 4, i64 %umul)
  store ptr %call, ptr %ptr, align 8
  %ptr2 = load ptr, ptr %ptr, align 8
  store ptr %ptr2, ptr %return_var, align 8
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @dealloc_int32(ptr %0) {
entry:
  %ptr = alloca ptr, align 8
  store ptr %0, ptr %ptr, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %ptr1 = load ptr, ptr %ptr, align 8
  call void @free(ptr %ptr1)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}
