use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::ir::creator::*;
use crate::ir::operate::*;
use crate::c_string;

#[allow(unused_imports)]
use crate::ir::block::*;

#[allow(unused_imports)]
use crate::ir::condition::*;

#[allow(unused_imports)]
use crate::ir::const_value::*;

#[allow(unused_imports)]
use crate::ir::test_util::*;

#[allow(unused_imports)]
use crate::ir::llvm_type::*;


#[allow(dead_code)]
pub fn add_function(
    target_module: *mut LLVMModule,
    function_type: *mut LLVMType,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMAddFunction(target_module, c_string!(name).as_ptr(), function_type) }
}

#[allow(dead_code)]
pub fn call_function(
    builder: *mut LLVMBuilder,
    func_type: *mut LLVMType,
    function: *mut LLVMValue,
    mut args: Vec<*mut LLVMValue>,
    name: &str,
) -> *mut LLVMValue {
    unsafe {
        LLVMBuildCall2(
            builder,
            func_type,
            function,
            args.as_mut_ptr(),
            args.len() as u32,
            c_string!(name).as_ptr(),
        )
    }
}

#[allow(dead_code)]
pub fn get_param(target_func: *mut LLVMValue, arg_index: u32) -> *mut LLVMValue {
    unsafe { LLVMGetParam(target_func, arg_index) }
}

#[allow(dead_code)]
pub fn create_function(
    lc: &mut LLVMCreator,
    fn_type: *mut LLVMType,
) -> (*mut LLVMValue, *mut LLVMBasicBlock) {
    let function = add_function(lc.module, fn_type, "");
    let block = append_basic_block(function, "entry");
    build_position_at_end(lc.builder, block);
    (function, block)
}

#[allow(dead_code)]
pub fn get_named_function(module: *mut LLVMModule, name: &str) -> *mut LLVMValue {
    unsafe { LLVMGetNamedFunction(module, c_string!(name).as_ptr()) }
}
/*
#[test]
fn call_printf() {
    let mut lc = LLVMCreator::new("test_module");
    let main = setup_main(&mut lc);
    let printf = lc.built_ins["printf"];
    //let printf_args = vec![codegen_string(&mut lc, "hello world\n\r", "")];
    let printf_args = vec![];

    unsafe{
        /*let ptr_type = LLVMTypeOf(printf);
        let val_type = LLVMGetElementType(ptr_type);
        call_function(lc.builder, int32_type(), printf, printf_args, "");

        build_ret(lc.builder, const_int(int32_type(), 2));

        execute_test_ir_function(lc.module, main);*/
    }
    let test_fn_type = function_type(int32_type(), &mut [pointer_type()]);

    call_function(lc.builder, test_fn_type, printf, printf_args, "");

    build_ret(lc.builder, const_int(int32_type(), 2));

    execute_test_ir_function(lc.module, main);
}
*/
#[test]
fn call_int_func() {
    let mut lc = LLVMCreator::new("test_module");
    let test_fn_type = function_type(int32_type(), &mut [int32_type()]);
    let (test_func, _) = create_function(&mut lc, test_fn_type);
    build_ret(lc.builder, get_param(test_func, 0));

    let main = setup_main(&mut lc);
    let test_func_args = vec![const_int(int32_type(), 10)];
    let called = call_function(lc.builder, test_fn_type, test_func, test_func_args, "");
    build_ret(lc.builder, called);

    let for_assert = execute_test_ir_function(lc.module, main);
    let expected = 10;
    assert!(
        for_assert == expected,
        "test failed \r\nexpected: {}\r\nactual:{}",
        expected,
        for_assert
    );
}
