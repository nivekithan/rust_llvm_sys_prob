use llvm_sys::prelude::*;


pub struct LlvmValueRef {
    pub llvm_ref :  LLVMValueRef
}

#[cfg(test)]
mod test {
    #[test]
    fn test_foo() {
        assert_eq!(1, 1);
    }
}