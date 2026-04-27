; ModuleID = 'lambda_program'
source_filename = "lambda_program"

define i8 @main() {
entry:
  %return_var = alloca i8, align 1
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load i8, ptr %return_var, align 1
  ret i8 %return_val

code_:                                            ; preds = %entry
  store i8 0, ptr %return_var, align 1
  br label %drop_and_return_

drop_and_return_:                                 ; preds = %code_
  br label %return

drop_and_merge_:                                  ; No predecessors!
  br label %return
}

define { { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 } } @RayConstants_new() {
entry:
  %return_var = alloca { { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 } }, align 8
  br label %code_

return:                                           ; preds = %drop_and_merge_, %drop_and_return_
  %return_val = load { { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 } }, ptr %return_var, align 1
  ret { { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 } } %return_val

code_:                                            ; preds = %entry
  store { { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 }, { i8, i8, i8, i8 } } { { i8, i8, i8, i8 } { i8 -56, i8 -56, i8 -56, i8 -1 }, { i8, i8, i8, i8 } { i8 -126, i8 -126, i8 -126, i8 -1 }, { i8, i8, i8, i8 } { i8 80, i8 80, i8 80, i8 -1 }, { i8, i8, i8, i8 } { i8 -3, i8 -7, i8 0, i8 -1 }, { i8, i8, i8, i8 } { i8 -1, i8 -53, i8 0, i8 -1 }, { i8, i8, i8, i8 } { i8 -1, i8 -95, i8 0, i8 -1 }, { i8, i8, i8, i8 } { i8 -1, i8 109, i8 -62, i8 -1 }, { i8, i8, i8, i8 } { i8 -26, i8 41, i8 55, i8 -1 }, { i8, i8, i8, i8 } { i8 -66, i8 33, i8 55, i8 -1 }, { i8, i8, i8, i8 } { i8 0, i8 -28, i8 48, i8 -1 }, { i8, i8, i8, i8 } { i8 0, i8 -98, i8 47, i8 -1 }, { i8, i8, i8, i8 } { i8 0, i8 117, i8 44, i8 -1 }, { i8, i8, i8, i8 } { i8 102, i8 -65, i8 -1, i8 -1 }, { i8, i8, i8, i8 } { i8 0, i8 121, i8 -15, i8 -1 }, { i8, i8, i8, i8 } { i8 0, i8 82, i8 -84, i8 -1 }, { i8, i8, i8, i8 } { i8 -56, i8 122, i8 -1, i8 -1 }, { i8, i8, i8, i8 } { i8 -121, i8 60, i8 -66, i8 -1 }, { i8, i8, i8, i8 } { i8 112, i8 31, i8 126, i8 -1 }, { i8, i8, i8, i8 } { i8 -45, i8 -80, i8 -125, i8 -1 }, { i8, i8, i8, i8 } { i8 127, i8 106, i8 79, i8 -1 }, { i8, i8, i8, i8 } { i8 76, i8 63, i8 47, i8 -1 }, { i8, i8, i8, i8 } { i8 -1, i8 -1, i8 -1, i8 -1 }, { i8, i8, i8, i8 } { i8 0, i8 0, i8 0, i8 -1 }, { i8, i8, i8, i8 } zeroinitializer, { i8, i8, i8, i8 } { i8 -1, i8 0, i8 -1, i8 -1 }, { i8, i8, i8, i8 } { i8 -11, i8 -11, i8 -11, i8 -1 } }, ptr %return_var, align 1
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

declare void @DrawText(ptr, i32, i32, i32, { i8, i8, i8, i8 })

declare void @ClearBackground({ i8, i8, i8, i8 })

declare ptr @aligned_alloc(i64, i64)

declare void @free(ptr)

declare void @exit(i32)
