// compile-flags: --crate-type=lib -Zmerge-functions=disabled -C opt-level=2 -C overflow-checks=false

// CHECK-LABEL: @a(
#[no_mangle]
pub fn a(exp: u32) -> u64 {
    // CHECK: %[[R:.+]] = icmp ugt i32 %exp, 64
    // CHECK: %[[R:.+]] = zext i32 %exp to i64
    // CHECK: %[[R:.+]] = shl nuw i64 [[R:.+]], %[[R:.+]]
    // CHECK: ret i64 %[[R:.+]]
    2u64.pow(exp)
}

// CHECK-LABEL: @b(
#[no_mangle]
pub fn b(exp: u32) -> i64 {
    // CHECK: %[[R:.+]] = icmp ugt i32 %exp, 64
    // CHECK: %[[R:.+]] = zext i32 %exp to i64
    // CHECK: %[[R:.+]] = shl nuw i64 [[R:.+]], %[[R:.+]]
    // CHECK: ret i64 %[[R:.+]]
    2i64.pow(exp)
}

// CHECK-LABEL: @c(
#[no_mangle]
pub fn c(exp: u32) -> u32 {
    // CHECK: %[[R:.+]] = icmp ugt i32 %exp, 16
    // CHECK: %[[R:.+]] = shl nuw nsw i32 %exp, 1
    // CHECK: %[[R:.+]] = shl nuw i32 1, %[[R:.+]]
    // CHECK: %[[R:.+]] = select i1 %[[R:.+]], i32 0, i32 %[[R:.+]]
    // CHECK: ret i32 %[[R:.+]]
    4u32.pow(exp)
}

// CHECK-LABEL: @d(
#[no_mangle]
pub fn d(exp: u32) -> u32 {
    // CHECK: %[[R:.+]] = icmp ugt i32 %exp, 6
    // CHECK: %[[R:.+]] = mul nuw nsw i32 %exp, 5
    // CHECK: %[[R:.+]] = shl nuw nsw i32 1, %[[R:.+]]
    // CHECK: %[[R:.+]] = select i1 [[R:.+]], i32 0, i32 %[[R:.+]]
    // CHECK: ret i32 %[[R:.+]]
    32u32.pow(exp)
}

// CHECK-LABEL: @e(
#[no_mangle]
pub fn e(exp: u32) -> i32 {
    // CHECK: %[[R:.+]] = icmp ugt i32 %exp, 6
    // CHECK: %[[R:.+]] = mul nuw nsw i32 %exp, 5
    // CHECK: %[[R:.+]] = shl nuw nsw i32 1, %[[R:.+]]
    // CHECK: %[[R:.+]] = select i1 [[R:.+]], i32 0, i32 %[[R:.+]]
    // CHECK: ret i32 %[[R:.+]]
    32i32.pow(exp)
}
// note: d and e are expected to yield the same IR
