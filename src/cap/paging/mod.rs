pub mod table;
pub mod entry;

use common::*;
use core::marker::PhantomData;

pub struct AddressCapability<T> {
    addr: VirtualAddress,
    page_table_addr: PhysicalAddress,
    _marker: PhantomData<T>,
}

impl<T> AddressCapability<T> {
    pub unsafe fn new(addr: VirtualAddress, page_table_addr: PhysicalAddress) -> Self {
        AddressCapability::<T> {
            addr: addr,
            page_table_addr: page_table_addr,
            _marker: PhantomData,
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe { ::x86::controlregs::cr3() as PhysicalAddress == self.page_table_addr }
    }

    fn borrow(&self) -> &T {
        assert!(self.is_active());
        unsafe { &*(self.addr as *const _) }
    }

    fn borrow_mut(&mut self) -> &mut T {
        assert!(self.is_active());
        unsafe { &mut *(self.addr as *mut _) }
    }
}

impl<T> Drop for AddressCapability<T> {
    fn drop(&mut self) {
        unimplemented!()
    }
}
