use llvm_sys::core::*;
use llvm_sys::LLVMLinkage::*;
use llvm_sys::*;

use crate::ir::creator::*;
use crate::ir::llvm_type::*;
use crate::ir::scope::*;
use crate::c_string;
use crate::string_from_raw;

#[allow(dead_code)]
pub fn const_int(llvm_type: *mut LLVMType, value: u64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, 0) }
}

#[allow(dead_code)]
pub fn const_neg(value: *mut LLVMValue) -> *mut LLVMValue {
    unsafe { LLVMConstNeg(value) }
}

#[allow(dead_code)]
pub fn const_int_signed(llvm_type: *mut LLVMType, value: u64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, 0) }
}

#[allow(dead_code)]
pub fn const_array(
    lc: &mut LLVMCreator,
    llvm_type: *mut LLVMType,
    mut value: Vec<*mut LLVMValue>,
) -> *mut LLVMValue {
    let llvm_array = unsafe { LLVMConstArray(llvm_type, value.as_mut_ptr(), value.len() as u32) };
    let global_array_val = add_global(lc.module, type_of(llvm_array), "");
    set_linkage(global_array_val, LLVMPrivateLinkage);
    set_initializer(global_array_val, llvm_array);
    set_global_constant(global_array_val);
    set_unnamed_address(global_array_val);

    return global_array_val;
}
