#![allow(missing_docs)]

use super::sys::*;
use dlopen::wrapper::{WrapperApi, Container};
use std::ffi::c_void;

extern crate dlopen;
extern crate dlopen_derive;

#[derive(WrapperApi)]
pub struct JvmLibrary {
    #[dlopen_name = "JNI_CreateJavaVM"]
    jni_create_java_vm: unsafe extern "C" fn(
        java_vm: *mut *mut JavaVM,
        env: *mut *mut c_void,
        args: *mut c_void,
    ) -> jint,
    #[dlopen_name = "JNI_GetCreatedJavaVMs"]
    jni_get_created_java_vms: unsafe extern "C" fn(
        java_vm: *mut *mut JavaVM,
        buf_len: jsize,
        n_vms: *mut jsize,
    ) -> jint,
    #[dlopen_name = "JNI_GetDefaultJavaVMInitArgs"]
    jni_get_default_java_vm_init_args: unsafe extern "C" fn (
        args: *mut c_void,
    ) -> jint
}

pub unsafe fn jni_create_java_vm(lib: Container<JvmLibrary>, java_vm: *mut *mut JavaVM,
                                 env: *mut *mut c_void,
                                 args: *mut c_void) -> jint {
    lib.jni_create_java_vm(java_vm, env, args)

}
pub unsafe fn jni_get_created_java_vms(lib: Container<JvmLibrary>, java_vm: *mut *mut JavaVM,
                                       buf_len: jsize,
                                       n_vms: *mut jsize) -> jint {
    lib.jni_get_created_java_vms(java_vm, buf_len, n_vms)
}

pub unsafe fn jni_get_default_java_vm_init_args(lib: Container<JvmLibrary>, args: *mut c_void ) -> jint {
    lib.jni_get_default_java_vm_init_args(args)
}
