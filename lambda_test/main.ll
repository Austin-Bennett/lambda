; ModuleID = 'lambda_program'
source_filename = "lambda_program"

%RayConstants = type { %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color, %Color }
%Color = type { i8, i8, i8, i8 }
%Rect = type { float, float, float, float }
%FVec2 = type { float, float }
%GameObject = type { ptr, %GameObjectVTable }
%GameObjectVTable = type { ptr, ptr, ptr }
%Paddle = type { %Rect }
%Ball = type { %Rect, %FVec2 }

@cstring_literal = private constant [12 x i8] c"Lambda PONG\00"
@cstring_literal.1 = private constant [14 x i8] c"Hello, LAMBDA\00"

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
  %member2 = extractvalue %GameObjectVTable %member, 2
  %self3 = load ptr, ptr %self, align 8
  %refread4 = load %GameObject, ptr %self3, align 8
  %member5 = extractvalue %GameObject %refread4, 0
  call void %member2(ptr %member5)
  br label %drop_and_merge_

drop_and_return_:                                 ; No predecessors!
  br label %return

drop_and_merge_:                                  ; preds = %code_
  br label %return
}

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
  %return_val = load %Ball, ptr %return_var, align 4
  ret %Ball %return_val

code_:                                            ; preds = %entry
  store %Ball { %Rect { float 3.950000e+02, float 2.950000e+02, float 1.000000e+01, float 1.000000e+01 }, %FVec2 { double -1.000000e+01, float 0.000000e+00 } }, ptr %return_var, align 4
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
  %refread = load %Ball, ptr %self1, align 4
  %member = extractvalue %Ball %refread, 0
  %call = call float @Rect_center_x(%Rect %member)
  %fptosi = fptosi float %call to i32
  %self2 = load ptr, ptr %self, align 8
  %refread3 = load %Ball, ptr %self2, align 4
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

define void @Ball_update(ptr %0, ptr %1, ptr %2) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %lpaddle = alloca ptr, align 8
  store ptr %1, ptr %lpaddle, align 8
  %rpaddle = alloca ptr, align 8
  store ptr %2, ptr %rpaddle, align 8
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

define i8 @main() {
entry:
  %return_var = alloca i8, align 1
  %constants = alloca %RayConstants, align 8
  %r = alloca %Rect, align 8
  %mrect = alloca %Rect, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  call void @InitWindow(i32 800, i32 600, ptr @cstring_literal)
  %call = call %RayConstants @RayConstants_new()
  store %RayConstants %call, ptr %constants, align 1
  store %Rect { float 2.000000e+02, float 2.000000e+02, float 1.000000e+02, float 1.000000e+02 }, ptr %r, align 4
  store %Rect { float 0.000000e+00, float 0.000000e+00, float 2.000000e+01, float 2.000000e+01 }, ptr %mrect, align 4
  br label %loop_start

drop_and_return_:                                 ; preds = %loop_merge, %drop_and_return_3
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return

loop_start:                                       ; preds = %drop_and_merge_4, %code_
  %call1 = call i1 @WindowShouldClose()
  %bnot = xor i1 %call1, true
  br i1 %bnot, label %loop, label %loop_merge

loop:                                             ; preds = %loop_start
  br label %code_2

loop_merge:                                       ; preds = %loop_start
  call void @CloseWindow()
  %mrect8 = load %Rect, ptr %mrect, align 4
  %member9 = extractvalue %Rect %mrect8, 1
  %fptosi = fptosi float %member9 to i8
  store i8 %fptosi, ptr %return_var, align 1
  br label %drop_and_return_

code_2:                                           ; preds = %loop
  call void @BeginDrawing()
  %constants5 = load %RayConstants, ptr %constants, align 1
  %member = extractvalue %RayConstants %constants5, 22
  call void @ClearBackground(%Color %member)
  %constants6 = load %RayConstants, ptr %constants, align 1
  %member7 = extractvalue %RayConstants %constants6, 4
  call void @DrawText(ptr @cstring_literal.1, i32 10, i32 10, i32 20, %Color %member7)
  call void @EndDrawing()
  br label %drop_and_merge_4

drop_and_return_3:                                ; No predecessors!
  br label %drop_and_return_

drop_and_merge_4:                                 ; preds = %code_2
  br label %loop_start
}
