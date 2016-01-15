extern crate child;

#[derive(Clone,Copy)]
#[repr(u32)]
pub enum LocalEnum {
  A = 0,
  B = 1
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LocalWrapper {
  A = LocalEnum::A as isize,
  B = LocalEnum::B as isize
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ChildWrapper {
  A = child::OtherCrateEnum::A as isize,
  B = child::OtherCrateEnum::B as isize
}
