; ModuleID = 'lambda_program'
source_filename = "lambda_program"

%Paddle = type { %Rect }
%Rect = type { float, float, float, float }
%Color = type { i8, i8, i8, i8 }
%Ball = type { %Rect, float, float, ptr }
%RayConstants = type { %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color }
%Game = type { { ptr, i64, i64 } }
%GameObjectVTable = type { ptr, ptr, ptr, ptr }
%FVec2 = type { float, float }
%GameObject = type { ptr, %GameObjectVTable }

@cstring_literal = private constant [12 x i8] c"Lambda PONG\00"
@cstring_literal.1 = private constant [14 x i8] c"Hello, LAMBDA\00"

define %Paddle @Paddle_new(float %0, float %1) {
entry:
  %x = alloca float, align 4
  store float %0, ptr %x, align 4
  %y = alloca float, align 4
  store float %1, ptr %y, align 4
  %return_var = alloca %Paddle, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load %Paddle, ptr %return_var, align 4
  ret %Paddle %return_val

code_:                                            ; preds = %entry
  %x1 = load float, ptr %x, align 4
  %sf = insertvalue %Rect undef, float %x1, 0
  %y2 = load float, ptr %y, align 4
  %sf3 = insertvalue %Rect %sf, float %y2, 1
  %sf4 = insertvalue %Rect %sf3, float 2.000000e+01, 2
  %sf5 = insertvalue %Rect %sf4, float 1.000000e+02, 3
  %sf6 = insertvalue %Paddle undef, %Rect %sf5, 0
  store %Paddle %sf6, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @Paddle_draw(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load %Paddle, ptr %self1, align 4
  %member = extractvalue %Paddle %refread, 0
  call void @DrawRectangleRec(%Rect %member, %Color { i8 -1, i8 -1, i8 -1, i8 -1 })
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define %Ball @Ball_new() {
entry:
  %return_var = alloca %Ball, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load %Ball, ptr %return_var, align 8
  ret %Ball %return_val

code_:                                            ; preds = %entry
  store %Ball { %Rect { float 3.950000e+02, float 3.950000e+02, float 1.000000e+01, float 1.000000e+01 }, float -3.000000e+02, float 0.000000e+00, ptr null }, ptr %return_var, align 8
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @Ball_draw(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load %Ball, ptr %self1, align 8
  %member = extractvalue %Ball %refread, 0
  %call = call float @Rect_center_x(%Rect %member)
  %fptosi = fptosi float %call to i32
  %self2 = load ptr, ptr %self, align 8
  %refread3 = load %Ball, ptr %self2, align 8
  %member4 = extractvalue %Ball %refread3, 0
  %call5 = call float @Rect_center_y(%Rect %member4)
  %fptosi6 = fptosi float %call5 to i32
  call void @DrawCirc(i32 %fptosi, i32 %fptosi6, float 5.000000e+00, %Color { i8 -1, i8 -1, i8 -1, i8 -1 })
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define void @Ball_update(ptr %0, float %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %dt = alloca float, align 4
  store float %1, ptr %dt, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw %Ball, ptr %self1, i32 0, i32 0
  %field_ptr2 = getelementptr inbounds nuw %Rect, ptr %field_ptr, i32 0, i32 0
  %ca_load = load float, ptr %field_ptr2, align 4
  %self3 = load ptr, ptr %self, align 8
  %refread = load %Ball, ptr %self3, align 8
  %member = extractvalue %Ball %refread, 1
  %dt4 = load float, ptr %dt, align 4
  %fmul = fmul float %member, %dt4
  %fadd = fadd float %ca_load, %fmul
  store float %fadd, ptr %field_ptr2, align 4
  %self5 = load ptr, ptr %self, align 8
  %field_ptr6 = getelementptr inbounds nuw %Ball, ptr %self5, i32 0, i32 0
  %field_ptr7 = getelementptr inbounds nuw %Rect, ptr %field_ptr6, i32 0, i32 1
  %ca_load8 = load float, ptr %field_ptr7, align 4
  %self9 = load ptr, ptr %self, align 8
  %refread10 = load %Ball, ptr %self9, align 8
  %member11 = extractvalue %Ball %refread10, 2
  %dt12 = load float, ptr %dt, align 4
  %fmul13 = fmul float %member11, %dt12
  %fadd14 = fadd float %ca_load8, %fmul13
  store float %fadd14, ptr %field_ptr7, align 4
  %self15 = load ptr, ptr %self, align 8
  %refread16 = load %Ball, ptr %self15, align 8
  %member17 = extractvalue %Ball %refread16, 0
  %member18 = extractvalue %Rect %member17, 0
  %flt = fcmp olt float %member18, 0.000000e+00
  br i1 %flt, label %then, label %else

drop_and_return_:                                 ; preds = %drop_and_return_32, %drop_and_return_20
  br label %return

drop_and_merge_:                                  ; preds = %if_merge26
  br label %return

then:                                             ; preds = %code_
  br label %code_19

else:                                             ; preds = %code_
  br label %if_merge

if_merge:                                         ; preds = %else, %drop_and_merge_21
  %self27 = load ptr, ptr %self, align 8
  %refread28 = load %Ball, ptr %self27, align 8
  %member29 = extractvalue %Ball %refread28, 0
  %member30 = extractvalue %Rect %member29, 0
  %fgt = fcmp ogt float %member30, 8.000000e+02
  br i1 %fgt, label %then24, label %else25

code_19:                                          ; preds = %then
  %self22 = load ptr, ptr %self, align 8
  %field_ptr23 = getelementptr inbounds nuw %Ball, ptr %self22, i32 0, i32 1
  store float 3.000000e+02, ptr %field_ptr23, align 4
  br label %drop_and_merge_21

drop_and_return_20:                               ; No predecessors!
  br label %drop_and_return_

drop_and_merge_21:                                ; preds = %code_19
  br label %if_merge

then24:                                           ; preds = %if_merge
  br label %code_31

else25:                                           ; preds = %if_merge
  br label %if_merge26

if_merge26:                                       ; preds = %else25, %drop_and_merge_33
  br label %drop_and_merge_

code_31:                                          ; preds = %then24
  %self34 = load ptr, ptr %self, align 8
  %field_ptr35 = getelementptr inbounds nuw %Ball, ptr %self34, i32 0, i32 1
  store float -3.000000e+02, ptr %field_ptr35, align 4
  br label %drop_and_merge_33

drop_and_return_32:                               ; No predecessors!
  br label %drop_and_return_

drop_and_merge_33:                                ; preds = %code_31
  br label %if_merge26
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

define i8 @main() {
entry:
  %return_var = alloca i8, align 1
  %constants = alloca %RayConstants, align 8
  %list = alloca { ptr, i64, i64 }, align 8
  %a = alloca i32, align 4
  %b = alloca i32, align 4
  %game = alloca %Game, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  call void @InitWindow(i32 800, i32 800, ptr @cstring_literal)
  %call = call %RayConstants @RayConstants_new()
  store %RayConstants %call, ptr %constants, align 1
  %call1 = call { ptr, i64, i64 } @List_int32_new()
  store { ptr, i64, i64 } %call1, ptr %list, align 8
  store i32 -1000, ptr %a, align 4
  %a2 = load i32, ptr %a, align 4
  %call3 = call i32 @int32_abs(i32 %a2)
  store i32 %call3, ptr %b, align 4
  %call4 = call %Game @Game_new()
  store %Game %call4, ptr %game, align 8
  %call5 = call %Ball @Ball_new()
  call void @Game_add_obj_Ball(ptr %game, %Ball %call5, %GameObjectVTable { ptr @lambda_main_0, ptr @lambda_main_1, ptr @lambda_main_2, ptr @lambda_main_3 })
  call void @SetTargetFPS(i32 60)
  br label %loop_start

drop_and_return_:                                 ; preds = %loop_merge, %drop_and_return_8
  call void @Game__op_drop(ptr %game)
  call void @List_int32__op_drop(ptr %list)
  br label %return

drop_and_merge_:                                  ; No predecessors!
  call void @Game__op_drop(ptr %game)
  call void @List_int32__op_drop(ptr %list)
  br label %return

loop_start:                                       ; preds = %drop_and_merge_9, %code_
  %call6 = call i1 @WindowShouldClose()
  %bnot = xor i1 %call6, true
  br i1 %bnot, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_7

loop_merge:                                       ; preds = %loop_start
  call void @CloseWindow()
  store i8 0, ptr %return_var, align 1
  br label %drop_and_return_

code_7:                                           ; preds = %loop
  call void @Game_update(ptr %game, float 0x3F9119CE00000000)
  call void @BeginDrawing()
  %constants10 = load %RayConstants, ptr %constants, align 1
  %member = extractvalue %RayConstants %constants10, 22
  call void @ClearBackground(%Color %member)
  call void @Game_draw(ptr %game)
  %constants11 = load %RayConstants, ptr %constants, align 1
  %member12 = extractvalue %RayConstants %constants11, 4
  call void @DrawText(ptr @cstring_literal.1, i32 10, i32 10, i32 20, %Color %member12)
  call void @EndDrawing()
  br label %drop_and_merge_9

drop_and_return_8:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_9:                                 ; preds = %code_7
  br label %loop_start
}

declare ptr @aligned_alloc(i64, i64)

declare void @free(ptr)

declare void @exit(i32)

declare void @printf(ptr)

define %RayConstants @RayConstants_new() {
entry:
  %return_var = alloca %RayConstants, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load %RayConstants, ptr %return_var, align 1
  ret %RayConstants %return_val

code_:                                            ; preds = %entry
  store %RayConstants { %Color { i8 -56, i8 -56, i8 -56, i8 -1 }, %Color { i8 -126, i8 -126, i8 -126, i8 -1 }, %Color { i8 80, i8 80, i8 80, i8 -1 }, %Color { i8 -3, i8 -7, i8 0, i8 -1 }, %Color { i8 -1, i8 -53, i8 0, i8 -1 }, %Color { i8 -1, i8 -95, i8 0, i8 -1 }, %Color { i8 -1, i8 109, i8 -62, i8 -1 }, %Color { i8 -26, i8 41, i8 55, i8 -1 }, %Color { i8 -66, i8 33, i8 55, i8 -1 }, %Color { i8 0, i8 -28, i8 48, i8 -1 }, %Color { i8 0, i8 -98, i8 47, i8 -1 }, %Color { i8 0, i8 117, i8 44, i8 -1 }, %Color { i8 102, i8 -65, i8 -1, i8 -1 }, %Color { i8 0, i8 121, i8 -15, i8 -1 }, %Color { i8 0, i8 82, i8 -84, i8 -1 }, %Color { i8 -56, i8 122, i8 -1, i8 -1 }, %Color { i8 -121, i8 60, i8 -66, i8 -1 }, %Color { i8 112, i8 31, i8 126, i8 -1 }, %Color { i8 -45, i8 -80, i8 -125, i8 -1 }, %Color { i8 127, i8 106, i8 79, i8 -1 }, %Color { i8 76, i8 63, i8 47, i8 -1 }, %Color { i8 -1, i8 -1, i8 -1, i8 -1 }, %Color { i8 0, i8 0, i8 0, i8 -1 }, %Color zeroinitializer, %Color { i8 -1, i8 0, i8 -1, i8 -1 }, %Color { i8 -11, i8 -11, i8 -11, i8 -1 } }, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define i1 @Rect_collides(%Rect %0, %Rect %1) {
entry:
  %self = alloca %Rect, align 8
  store %Rect %0, ptr %self, align 4
  %r = alloca %Rect, align 8
  store %Rect %1, ptr %r, align 4
  %return_var = alloca i1, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i1, ptr %return_var, align 1
  ret i1 %return_val

code_:                                            ; preds = %entry
  %self1 = load %Rect, ptr %self, align 4
  %member = extractvalue %Rect %self1, 0
  %r2 = load %Rect, ptr %r, align 4
  %member3 = extractvalue %Rect %r2, 0
  %r4 = load %Rect, ptr %r, align 4
  %member5 = extractvalue %Rect %r4, 2
  %fadd = fadd float %member3, %member5
  %flt = fcmp olt float %member, %fadd
  br i1 %flt, label %and_rhs, label %and_merge

drop_and_return_:                                 ; preds = %and_merge
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

and_rhs:                                          ; preds = %code_
  %self6 = load %Rect, ptr %self, align 4
  %member7 = extractvalue %Rect %self6, 0
  %self8 = load %Rect, ptr %self, align 4
  %member9 = extractvalue %Rect %self8, 2
  %fadd10 = fadd float %member7, %member9
  %r11 = load %Rect, ptr %r, align 4
  %member12 = extractvalue %Rect %r11, 0
  %fgt = fcmp ogt float %fadd10, %member12
  br i1 %fgt, label %and_rhs13, label %and_merge14

and_merge:                                        ; preds = %and_merge14, %code_
  %and_result34 = phi i1 [ false, %code_ ], [ %and_result33, %and_merge14 ]
  store i1 %and_result34, ptr %return_var, align 1
  br label %drop_and_return_

and_rhs13:                                        ; preds = %and_rhs
  %self15 = load %Rect, ptr %self, align 4
  %member16 = extractvalue %Rect %self15, 1
  %r17 = load %Rect, ptr %r, align 4
  %member18 = extractvalue %Rect %r17, 1
  %r19 = load %Rect, ptr %r, align 4
  %member20 = extractvalue %Rect %r19, 3
  %fadd21 = fadd float %member18, %member20
  %flt22 = fcmp olt float %member16, %fadd21
  br i1 %flt22, label %and_rhs23, label %and_merge24

and_merge14:                                      ; preds = %and_merge24, %and_rhs
  %and_result33 = phi i1 [ false, %and_rhs ], [ %and_result, %and_merge24 ]
  br label %and_merge

and_rhs23:                                        ; preds = %and_rhs13
  %self25 = load %Rect, ptr %self, align 4
  %member26 = extractvalue %Rect %self25, 1
  %self27 = load %Rect, ptr %self, align 4
  %member28 = extractvalue %Rect %self27, 3
  %fadd29 = fadd float %member26, %member28
  %r30 = load %Rect, ptr %r, align 4
  %member31 = extractvalue %Rect %r30, 1
  %fgt32 = fcmp ogt float %fadd29, %member31
  br label %and_merge24

and_merge24:                                      ; preds = %and_rhs23, %and_rhs13
  %and_result = phi i1 [ false, %and_rhs13 ], [ %fgt32, %and_rhs23 ]
  br label %and_merge14
}

define float @Rect_center_x(%Rect %0) {
entry:
  %self = alloca %Rect, align 8
  store %Rect %0, ptr %self, align 4
  %return_var = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load float, ptr %return_var, align 4
  ret float %return_val

code_:                                            ; preds = %entry
  %self1 = load %Rect, ptr %self, align 4
  %member = extractvalue %Rect %self1, 0
  %self2 = load %Rect, ptr %self, align 4
  %member3 = extractvalue %Rect %self2, 2
  %fdiv = fdiv float %member3, 2.000000e+00
  %fadd = fadd float %member, %fdiv
  store float %fadd, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define float @Rect_center_y(%Rect %0) {
entry:
  %self = alloca %Rect, align 8
  store %Rect %0, ptr %self, align 4
  %return_var = alloca float, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load float, ptr %return_var, align 4
  ret float %return_val

code_:                                            ; preds = %entry
  %self1 = load %Rect, ptr %self, align 4
  %member = extractvalue %Rect %self1, 1
  %self2 = load %Rect, ptr %self, align 4
  %member3 = extractvalue %Rect %self2, 3
  %fdiv = fdiv float %member3, 2.000000e+00
  %fadd = fadd float %member, %fdiv
  store float %fadd, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

declare void @InitWindow(i32, i32, ptr)

declare void @CloseWindow()

declare i1 @WindowShouldClose()

declare void @SetTargetFPS(i32)

declare void @BeginDrawing()

declare void @EndDrawing()

declare void @DrawText(ptr, i32, i32, i32, %Color)

declare void @ClearBackground(%Color)

declare i32 @GetMouseX()

declare i32 @GetMouseY()

declare void @DrawCircle(i32, i32, float, i32)

define void @DrawCirc(i32 %0, i32 %1, float %2, %Color %3) {
entry:
  %cx = alloca i32, align 4
  store i32 %0, ptr %cx, align 4
  %cy = alloca i32, align 4
  store i32 %1, ptr %cy, align 4
  %rad = alloca float, align 4
  store float %2, ptr %rad, align 4
  %c = alloca %Color, align 8
  store %Color %3, ptr %c, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %cx1 = load i32, ptr %cx, align 4
  %cy2 = load i32, ptr %cy, align 4
  %rad3 = load float, ptr %rad, align 4
  %c4 = load %Color, ptr %c, align 1
  %call = call i32 @cToInt(%Color %c4)
  call void @DrawCircle(i32 %cx1, i32 %cy2, float %rad3, i32 %call)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define %FVec2 @GetMousePosition() {
entry:
  %return_var = alloca %FVec2, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load %FVec2, ptr %return_var, align 4
  ret %FVec2 %return_val

code_:                                            ; preds = %entry
  %call = call i32 @GetMouseX()
  %sitofp = sitofp i32 %call to float
  %sf = insertvalue %FVec2 undef, float %sitofp, 0
  %call1 = call i32 @GetMouseY()
  %sitofp2 = sitofp i32 %call1 to float
  %sf3 = insertvalue %FVec2 %sf, float %sitofp2, 1
  store %FVec2 %sf3, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

declare void @DrawRectangle(i32, i32, i32, i32, i32)

define i32 @cToInt(%Color %0) {
entry:
  %c = alloca %Color, align 8
  store %Color %0, ptr %c, align 1
  %return_var = alloca i32, align 4
  %r = alloca i32, align 4
  %g = alloca i32, align 4
  %b = alloca i32, align 4
  %a = alloca i32, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i32, ptr %return_var, align 4
  ret i32 %return_val

code_:                                            ; preds = %entry
  %c1 = load %Color, ptr %c, align 1
  %member = extractvalue %Color %c1, 0
  %zext = zext i8 %member to i32
  store i32 %zext, ptr %r, align 4
  %c2 = load %Color, ptr %c, align 1
  %member3 = extractvalue %Color %c2, 1
  %zext4 = zext i8 %member3 to i32
  store i32 %zext4, ptr %g, align 4
  %c5 = load %Color, ptr %c, align 1
  %member6 = extractvalue %Color %c5, 2
  %zext7 = zext i8 %member6 to i32
  store i32 %zext7, ptr %b, align 4
  %c8 = load %Color, ptr %c, align 1
  %member9 = extractvalue %Color %c8, 3
  %zext10 = zext i8 %member9 to i32
  store i32 %zext10, ptr %a, align 4
  %r11 = load i32, ptr %r, align 4
  %shl = shl i32 %r11, 24
  %g12 = load i32, ptr %g, align 4
  %shl13 = shl i32 %g12, 16
  %b14 = load i32, ptr %b, align 4
  %shl15 = shl i32 %b14, 8
  %a16 = load i32, ptr %a, align 4
  %bor = or i32 %shl15, %a16
  %bor17 = or i32 %shl13, %bor
  %bor18 = or i32 %shl, %bor17
  store i32 %bor18, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @DrawRectangleRec(%Rect %0, %Color %1) {
entry:
  %r = alloca %Rect, align 8
  store %Rect %0, ptr %r, align 4
  %c = alloca %Color, align 8
  store %Color %1, ptr %c, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %r1 = load %Rect, ptr %r, align 4
  %member = extractvalue %Rect %r1, 0
  %fptosi = fptosi float %member to i32
  %r2 = load %Rect, ptr %r, align 4
  %member3 = extractvalue %Rect %r2, 1
  %fptosi4 = fptosi float %member3 to i32
  %r5 = load %Rect, ptr %r, align 4
  %member6 = extractvalue %Rect %r5, 2
  %fptosi7 = fptosi float %member6 to i32
  %r8 = load %Rect, ptr %r, align 4
  %member9 = extractvalue %Rect %r8, 3
  %fptosi10 = fptosi float %member9 to i32
  %c11 = load %Color, ptr %c, align 1
  %call = call i32 @cToInt(%Color %c11)
  call void @DrawRectangle(i32 %fptosi, i32 %fptosi4, i32 %fptosi7, i32 %fptosi10, i32 %call)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define void @GameObject__op_drop(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %refread = load %GameObject, ptr %self1, align 8
  %member = extractvalue %GameObject %refread, 1
  %member2 = extractvalue %GameObjectVTable %member, 3
  %self3 = load ptr, ptr %self, align 8
  %refread4 = load %GameObject, ptr %self3, align 8
  %member5 = extractvalue %GameObject %refread4, 0
  call void %member2(ptr %member5)
  %self6 = load ptr, ptr %self, align 8
  %refread7 = load %GameObject, ptr %self6, align 8
  %member8 = extractvalue %GameObject %refread7, 0
  call void @free(ptr %member8)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define %Game @Game_new() {
entry:
  %return_var = alloca %Game, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load %Game, ptr %return_var, align 8
  ret %Game %return_val

code_:                                            ; preds = %entry
  %call = call { ptr, i64, i64 } @List_GameObject_new()
  %sf = insertvalue %Game undef, { ptr, i64, i64 } %call, 0
  store %Game %sf, ptr %return_var, align 8
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @Game_update(ptr %0, float %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %dt = alloca float, align 4
  store float %1, ptr %dt, align 4
  %i = alloca i64, align 8
  %obj = alloca ptr, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  store i64 0, ptr %i, align 4
  br label %loop_start

drop_and_return_:                                 ; preds = %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; preds = %loop_merge
  br label %return

loop_start:                                       ; preds = %drop_and_merge_5, %code_
  %i1 = load i64, ptr %i, align 4
  %self2 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw %Game, ptr %self2, i32 0, i32 0
  %call = call i64 @List_GameObject_len(ptr %field_ptr)
  %ult = icmp ult i64 %i1, %call
  br i1 %ult, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_3

loop_merge:                                       ; preds = %loop_start
  br label %drop_and_merge_

code_3:                                           ; preds = %loop
  %self6 = load ptr, ptr %self, align 8
  %field_ptr7 = getelementptr inbounds nuw %Game, ptr %self6, i32 0, i32 0
  %i8 = load i64, ptr %i, align 4
  %call9 = call ptr @List_GameObject_get(ptr %field_ptr7, i64 %i8)
  store ptr %call9, ptr %obj, align 8
  %obj10 = load ptr, ptr %obj, align 8
  %refread = load %GameObject, ptr %obj10, align 8
  %member = extractvalue %GameObject %refread, 1
  %member11 = extractvalue %GameObjectVTable %member, 1
  %obj12 = load ptr, ptr %obj, align 8
  %refread13 = load %GameObject, ptr %obj12, align 8
  %member14 = extractvalue %GameObject %refread13, 0
  %dt15 = load float, ptr %dt, align 4
  call void %member11(ptr %member14, float %dt15)
  %ca_load = load i64, ptr %i, align 4
  %uadd = add i64 %ca_load, 1
  store i64 %uadd, ptr %i, align 4
  br label %drop_and_merge_5

drop_and_return_4:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_5:                                 ; preds = %code_3
  br label %loop_start
}

define void @Game_draw(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %i = alloca i64, align 8
  %obj = alloca ptr, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  store i64 0, ptr %i, align 4
  br label %loop_start

drop_and_return_:                                 ; preds = %drop_and_return_4
  br label %return

drop_and_merge_:                                  ; preds = %loop_merge
  br label %return

loop_start:                                       ; preds = %drop_and_merge_5, %code_
  %i1 = load i64, ptr %i, align 4
  %self2 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw %Game, ptr %self2, i32 0, i32 0
  %call = call i64 @List_GameObject_len(ptr %field_ptr)
  %ult = icmp ult i64 %i1, %call
  br i1 %ult, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_3

loop_merge:                                       ; preds = %loop_start
  br label %drop_and_merge_

code_3:                                           ; preds = %loop
  %self6 = load ptr, ptr %self, align 8
  %field_ptr7 = getelementptr inbounds nuw %Game, ptr %self6, i32 0, i32 0
  %i8 = load i64, ptr %i, align 4
  %call9 = call ptr @List_GameObject_get(ptr %field_ptr7, i64 %i8)
  store ptr %call9, ptr %obj, align 8
  %obj10 = load ptr, ptr %obj, align 8
  %refread = load %GameObject, ptr %obj10, align 8
  %member = extractvalue %GameObject %refread, 1
  %member11 = extractvalue %GameObjectVTable %member, 2
  %obj12 = load ptr, ptr %obj, align 8
  %refread13 = load %GameObject, ptr %obj12, align 8
  %member14 = extractvalue %GameObject %refread13, 0
  call void %member11(ptr %member14)
  %ca_load = load i64, ptr %i, align 4
  %uadd = add i64 %ca_load, 1
  store i64 %uadd, ptr %i, align 4
  br label %drop_and_merge_5

drop_and_return_4:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_5:                                 ; preds = %code_3
  br label %loop_start
}

define void @Game__op_drop(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %self1 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw %Game, ptr %self1, i32 0, i32 0
  call void @List_GameObject__op_drop(ptr %field_ptr)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define void @lambda_main_0(ptr %0, ptr %1) {
entry:
  %this = alloca ptr, align 8
  store ptr %0, ptr %this, align 8
  %game = alloca ptr, align 8
  store ptr %1, ptr %game, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %game1 = load ptr, ptr %game, align 8
  %this2 = load ptr, ptr %this, align 8
  %field_ptr = getelementptr inbounds nuw %Ball, ptr %this2, i32 0, i32 3
  store ptr %game1, ptr %field_ptr, align 8
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define void @lambda_main_1(ptr %0, float %1) {
entry:
  %this = alloca ptr, align 8
  store ptr %0, ptr %this, align 8
  %dt = alloca float, align 4
  store float %1, ptr %dt, align 4
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %this1 = load ptr, ptr %this, align 8
  %dt2 = load float, ptr %dt, align 4
  call void @Ball_update(ptr %this1, float %dt2)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define void @lambda_main_2(ptr %0) {
entry:
  %this = alloca ptr, align 8
  store ptr %0, ptr %this, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %this1 = load ptr, ptr %this, align 8
  call void @Ball_draw(ptr %this1)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define void @lambda_main_3(ptr %0) {
entry:
  %this = alloca ptr, align 8
  store ptr %0, ptr %this, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define void @Game_add_obj_Ball(ptr %0, %Ball %1, %GameObjectVTable %2) {
entry:
  %this = alloca ptr, align 8
  store ptr %0, ptr %this, align 8
  %o = alloca %Ball, align 8
  store %Ball %1, ptr %o, align 8
  %vtable = alloca %GameObjectVTable, align 8
  store %GameObjectVTable %2, ptr %vtable, align 8
  %ptr = alloca ptr, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  ret void

code_:                                            ; preds = %entry
  %call = call ptr @alloc_Ball(i64 1)
  store ptr %call, ptr %ptr, align 8
  %o1 = load %Ball, ptr %o, align 8
  %ptr2 = load ptr, ptr %ptr, align 8
  store %Ball %o1, ptr %ptr2, align 8
  %this3 = load ptr, ptr %this, align 8
  %field_ptr = getelementptr inbounds nuw %Game, ptr %this3, i32 0, i32 0
  %ptr4 = load ptr, ptr %ptr, align 8
  %sf = insertvalue %GameObject undef, ptr %ptr4, 0
  %vtable5 = load %GameObjectVTable, ptr %vtable, align 8
  %sf6 = insertvalue %GameObject %sf, %GameObjectVTable %vtable5, 1
  call void @List_GameObject_add(ptr %field_ptr, %GameObject %sf6)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

define { ptr, i64, i64 } @List_GameObject_new() {
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

define i64 @List_GameObject_len(ptr %0) {
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
  %refread = load { ptr, i64, i64 }, ptr %self1, align 8
  %member = extractvalue { ptr, i64, i64 } %refread, 2
  store i64 %member, ptr %return_var, align 4
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @List_GameObject_add(ptr %0, %GameObject %1) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %val = alloca %GameObject, align 8
  store %GameObject %1, ptr %val, align 8
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
  %val55 = load %GameObject, ptr %val, align 8
  %self56 = load ptr, ptr %self, align 8
  %refread57 = load { ptr, i64, i64 }, ptr %self56, align 8
  %member58 = extractvalue { ptr, i64, i64 } %refread57, 0
  %self59 = load ptr, ptr %self, align 8
  %refread60 = load { ptr, i64, i64 }, ptr %self59, align 8
  %member61 = extractvalue { ptr, i64, i64 } %refread60, 2
  %ptr_add = getelementptr i8, ptr %member58, i64 %member61
  store %GameObject %val55, ptr %ptr_add, align 8
  %self62 = load ptr, ptr %self, align 8
  %field_ptr63 = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self62, i32 0, i32 2
  %ca_load64 = load i64, ptr %field_ptr63, align 4
  %uadd65 = add i64 %ca_load64, 1
  store i64 %uadd65, ptr %field_ptr63, align 4
  br label %drop_and_merge_

code_5:                                           ; preds = %then
  %self8 = load ptr, ptr %self, align 8
  %refread9 = load { ptr, i64, i64 }, ptr %self8, align 8
  %member10 = extractvalue { ptr, i64, i64 } %refread9, 2
  %umul = mul i64 %member10, 2
  %uadd = add i64 %umul, 1
  store i64 %uadd, ptr %new_size, align 4
  %new_size11 = load i64, ptr %new_size, align 4
  %call = call ptr @alloc_GameObject(i64 %new_size11)
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
  %new_size50 = load i64, ptr %new_size, align 4
  %self51 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self51, i32 0, i32 1
  store i64 %new_size50, ptr %field_ptr, align 4
  %new_alloc52 = load ptr, ptr %new_alloc, align 8
  %self53 = load ptr, ptr %self, align 8
  %field_ptr54 = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self53, i32 0, i32 0
  store ptr %new_alloc52, ptr %field_ptr54, align 8
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
  %self47 = load ptr, ptr %self, align 8
  %refread48 = load { ptr, i64, i64 }, ptr %self47, align 8
  %member49 = extractvalue { ptr, i64, i64 } %refread48, 0
  call void @dealloc_GameObject(ptr %member49)
  br label %drop_and_merge_22

code_35:                                          ; preds = %loop
  %ray_slice38 = load <{ i64, ptr }>, ptr %ray_slice, align 1
  %i39 = load i64, ptr %i, align 4
  %sl_ptr40 = extractvalue <{ i64, ptr }> %ray_slice38, 1
  %sl_elem_ptr = getelementptr %GameObject, ptr %sl_ptr40, i64 %i39
  %refread41 = load %GameObject, ptr %sl_elem_ptr, align 8
  %new_alloc_slice42 = load <{ i64, ptr }>, ptr %new_alloc_slice, align 1
  %i43 = load i64, ptr %i, align 4
  %sl_ptr44 = extractvalue <{ i64, ptr }> %new_alloc_slice42, 1
  %sl_elem_ptr45 = getelementptr %GameObject, ptr %sl_ptr44, i64 %i43
  store %GameObject %refread41, ptr %sl_elem_ptr45, align 8
  %ca_load = load i64, ptr %i, align 4
  %uadd46 = add i64 %ca_load, 1
  store i64 %uadd46, ptr %i, align 4
  br label %drop_and_merge_37

drop_and_return_36:                               ; No predecessors!
  br label %drop_and_return_21

drop_and_merge_37:                                ; preds = %code_35
  br label %loop_start
}

define ptr @List_GameObject_get(ptr %0, i64 %1) {
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

define void @List_GameObject__op_drop(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %sl = alloca <{ i64, ptr }>, align 8
  %i = alloca i64, align 8
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
  %member7 = extractvalue { ptr, i64, i64 } %refread6, 2
  %self8 = load ptr, ptr %self, align 8
  %refread9 = load { ptr, i64, i64 }, ptr %self8, align 8
  %member10 = extractvalue { ptr, i64, i64 } %refread9, 0
  %sl_len = insertvalue <{ i64, ptr }> undef, i64 %member7, 0
  %sl_ptr = insertvalue <{ i64, ptr }> %sl_len, ptr %member10, 1
  store <{ i64, ptr }> %sl_ptr, ptr %sl, align 1
  store i64 0, ptr %i, align 4
  br label %loop_start

drop_and_return_3:                                ; preds = %drop_and_return_16
  br label %drop_and_return_

drop_and_merge_4:                                 ; preds = %loop_merge
  br label %if_merge

loop_start:                                       ; preds = %drop_and_merge_17, %code_2
  %i11 = load i64, ptr %i, align 4
  %self12 = load ptr, ptr %self, align 8
  %refread13 = load { ptr, i64, i64 }, ptr %self12, align 8
  %member14 = extractvalue { ptr, i64, i64 } %refread13, 2
  %ult = icmp ult i64 %i11, %member14
  br i1 %ult, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_15

loop_merge:                                       ; preds = %loop_start
  %self21 = load ptr, ptr %self, align 8
  %refread22 = load { ptr, i64, i64 }, ptr %self21, align 8
  %member23 = extractvalue { ptr, i64, i64 } %refread22, 0
  call void @dealloc_GameObject(ptr %member23)
  br label %drop_and_merge_4

code_15:                                          ; preds = %loop
  %sl18 = load <{ i64, ptr }>, ptr %sl, align 1
  %i19 = load i64, ptr %i, align 4
  %sl_ptr20 = extractvalue <{ i64, ptr }> %sl18, 1
  %sl_elem_ptr = getelementptr %GameObject, ptr %sl_ptr20, i64 %i19
  call void @GameObject__op_drop(ptr %sl_elem_ptr)
  %ca_load = load i64, ptr %i, align 4
  %uadd = add i64 %ca_load, 1
  store i64 %uadd, ptr %i, align 4
  br label %drop_and_merge_17

drop_and_return_16:                               ; No predecessors!
  br label %drop_and_return_3

drop_and_merge_17:                                ; preds = %code_15
  br label %loop_start
}

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

define i64 @List_int32_len(ptr %0) {
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
  %refread = load { ptr, i64, i64 }, ptr %self1, align 8
  %member = extractvalue { ptr, i64, i64 } %refread, 2
  store i64 %member, ptr %return_var, align 4
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
  %val55 = load i32, ptr %val, align 4
  %self56 = load ptr, ptr %self, align 8
  %refread57 = load { ptr, i64, i64 }, ptr %self56, align 8
  %member58 = extractvalue { ptr, i64, i64 } %refread57, 0
  %self59 = load ptr, ptr %self, align 8
  %refread60 = load { ptr, i64, i64 }, ptr %self59, align 8
  %member61 = extractvalue { ptr, i64, i64 } %refread60, 2
  %ptr_add = getelementptr i8, ptr %member58, i64 %member61
  store i32 %val55, ptr %ptr_add, align 4
  %self62 = load ptr, ptr %self, align 8
  %field_ptr63 = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self62, i32 0, i32 2
  %ca_load64 = load i64, ptr %field_ptr63, align 4
  %uadd65 = add i64 %ca_load64, 1
  store i64 %uadd65, ptr %field_ptr63, align 4
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
  %new_size50 = load i64, ptr %new_size, align 4
  %self51 = load ptr, ptr %self, align 8
  %field_ptr = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self51, i32 0, i32 1
  store i64 %new_size50, ptr %field_ptr, align 4
  %new_alloc52 = load ptr, ptr %new_alloc, align 8
  %self53 = load ptr, ptr %self, align 8
  %field_ptr54 = getelementptr inbounds nuw { ptr, i64, i64 }, ptr %self53, i32 0, i32 0
  store ptr %new_alloc52, ptr %field_ptr54, align 8
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
  %self47 = load ptr, ptr %self, align 8
  %refread48 = load { ptr, i64, i64 }, ptr %self47, align 8
  %member49 = extractvalue { ptr, i64, i64 } %refread48, 0
  call void @dealloc_int32(ptr %member49)
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
  %ca_load = load i64, ptr %i, align 4
  %uadd46 = add i64 %ca_load, 1
  store i64 %uadd46, ptr %i, align 4
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
  %sl = alloca <{ i64, ptr }>, align 8
  %i = alloca i64, align 8
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
  %member7 = extractvalue { ptr, i64, i64 } %refread6, 2
  %self8 = load ptr, ptr %self, align 8
  %refread9 = load { ptr, i64, i64 }, ptr %self8, align 8
  %member10 = extractvalue { ptr, i64, i64 } %refread9, 0
  %sl_len = insertvalue <{ i64, ptr }> undef, i64 %member7, 0
  %sl_ptr = insertvalue <{ i64, ptr }> %sl_len, ptr %member10, 1
  store <{ i64, ptr }> %sl_ptr, ptr %sl, align 1
  store i64 0, ptr %i, align 4
  br label %loop_start

drop_and_return_3:                                ; preds = %drop_and_return_16
  br label %drop_and_return_

drop_and_merge_4:                                 ; preds = %loop_merge
  br label %if_merge

loop_start:                                       ; preds = %drop_and_merge_17, %code_2
  %i11 = load i64, ptr %i, align 4
  %self12 = load ptr, ptr %self, align 8
  %refread13 = load { ptr, i64, i64 }, ptr %self12, align 8
  %member14 = extractvalue { ptr, i64, i64 } %refread13, 2
  %ult = icmp ult i64 %i11, %member14
  br i1 %ult, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_15

loop_merge:                                       ; preds = %loop_start
  %self21 = load ptr, ptr %self, align 8
  %refread22 = load { ptr, i64, i64 }, ptr %self21, align 8
  %member23 = extractvalue { ptr, i64, i64 } %refread22, 0
  call void @dealloc_int32(ptr %member23)
  br label %drop_and_merge_4

code_15:                                          ; preds = %loop
  %sl18 = load <{ i64, ptr }>, ptr %sl, align 1
  %i19 = load i64, ptr %i, align 4
  %sl_ptr20 = extractvalue <{ i64, ptr }> %sl18, 1
  %sl_elem_ptr = getelementptr i32, ptr %sl_ptr20, i64 %i19
  %ca_load = load i64, ptr %i, align 4
  %uadd = add i64 %ca_load, 1
  store i64 %uadd, ptr %i, align 4
  br label %drop_and_merge_17

drop_and_return_16:                               ; No predecessors!
  br label %drop_and_return_3

drop_and_merge_17:                                ; preds = %code_15
  br label %loop_start
}

define ptr @alloc_Ball(i64 %0) {
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
  %umul = mul i64 32, %count1
  %call = call ptr @aligned_alloc(i64 8, i64 %umul)
  store ptr %call, ptr %ptr, align 8
  %ptr2 = load ptr, ptr %ptr, align 8
  store ptr %ptr2, ptr %return_var, align 8
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define ptr @alloc_GameObject(i64 %0) {
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
  %umul = mul i64 40, %count1
  %call = call ptr @aligned_alloc(i64 8, i64 %umul)
  store ptr %call, ptr %ptr, align 8
  %ptr2 = load ptr, ptr %ptr, align 8
  store ptr %ptr2, ptr %return_var, align 8
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define void @dealloc_GameObject(ptr %0) {
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
