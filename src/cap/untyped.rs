use common::*;
use core::ops::Drop;

use super::{MemoryBlockCapability};

/// Untyped memory and page table are memory management tricks, those are not
/// actually accessible in the virtual memory.

pub struct UntypedMemoryCapability {
    block_start_addr: PhysicalAddress,
    block_size: usize,
}

impl MemoryBlockCapability for UntypedMemoryCapability {
    fn block_start_addr(&self) -> PhysicalAddress {
        self.block_start_addr
    }

    fn block_size(&self) -> usize {
        self.block_size
    }
}

impl Drop for UntypedMemoryCapability {
    fn drop(&mut self) {
        if self.block_size() == 0 { return }

        unimplemented!();
    }
}

impl UntypedMemoryCapability {
    pub fn from_untyped_middle(cap: UntypedMemoryCapability, block_start_addr: usize, block_size: usize)
                              -> (UntypedMemoryCapability, Option<UntypedMemoryCapability>, Option<UntypedMemoryCapability>) {
        assert!(block_start_addr >= cap.block_start_addr(),
                "Requested block start address must be after the original capability.");
        assert!(block_start_addr + block_size <= cap.block_end_addr(),
                "Requested block end address must be before the original capability.");
        assert!(block_size > 0,
                "Block size must be greater than 0.");

        let u1_start_addr = cap.block_start_addr();
        let u1_size = block_start_addr - cap.block_start_addr();
        let u2_start_addr = block_start_addr;
        let u2_size = block_size;
        let u3_start_addr = u2_start_addr + u2_size;
        let u3_size = cap.block_end_addr() - u3_start_addr + 1;

        cap.block_start_addr = u2_start_addr;
        cap.block_size = u2_size;

        if u1_size > 0 && u3_size > 0 {
            (cap,
             Some(UntypedMemoryCapability { block_start_addr: u1_start_addr, block_size: u1_size }),
             Some(UntypedMemoryCapability { block_start_addr: u3_start_addr, block_size: u3_size }))
        } else if u1_size > 0 {
            (cap,
             Some(UntypedMemoryCapability { block_start_addr: u1_start_addr, block_size: u1_size }),
             None)
        } else if u3_size > 0 {
            (cap,
             Some(UntypedMemoryCapability { block_start_addr: u3_start_addr, block_size: u3_size}),
             None)
        } else {
            (cap,
             None,
             None)
        }
    }

    pub fn from_untyped(cap: UntypedMemoryCapability, block_size: usize)
                        -> (UntypedMemoryCapability, Option<UntypedMemoryCapability>) {
        let tuple = UntypedMemoryCapability::from_untyped_middle(cap, cap.block_start_addr(), block_size);
        assert!(tuple.2 == None, "According to logic, the third item of the tuple should be none.");

        (tuple.0, tuple.1)
    }

    pub unsafe fn reset(&self, block_start_addr: PhysicalAddress, block_size: usize) {
        self.block_start_addr = block_start_addr;
        self.block_size = block_size;
    }
}
