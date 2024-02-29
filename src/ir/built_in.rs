use llvm_sys::*;

use crate::ir::function::*;
use crate::ir::llvm_type::*;
use crate::c_string;

#[allow(dead_code)]
pub fn create_printf(module: *mut LLVMModule) -> *mut LLVMValue {
    let mut printf_args_type_list = vec![pointer_type()];
    let printf_type = function_type_var_arg(pointer_type(), &mut printf_args_type_list);

    add_function(module, printf_type, "printf")
}
