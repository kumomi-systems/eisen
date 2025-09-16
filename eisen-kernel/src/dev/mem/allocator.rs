// Eisen Operating System
// Copyright (C) 2025  Kumomi Systems
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/*
  I have big plans for this.

  Haskell seems like a fun idea to try implement here, it might offer a
  good solution to concurrent memory allocation in a safe way. Of course,
  we must also consider the performance impacts of the allocator, since this
  will be used a LOT. C is also an option, so keep things open. TL;DR, make
  the allocator really good.
*/

// Taken from here:
// https://os.phil-opp.com/allocator-designs/#linked-list-allocator

use core::alloc::GlobalAlloc;

struct AllocatorNode {
  size: usize,
  next: Option<&'static mut Self>
}

impl AllocatorNode {
  const fn new(size: usize) -> Self {
    Self { size, next: None }
  }

  fn start_addr(&self) -> usize {
    self as *const Self as usize
  }

  fn end_addr(&self) -> usize {
    self.start_addr() + self.size
  }
}

pub struct Allocator {
  head: AllocatorNode
}

impl Allocator {
  pub const fn new() -> Self {
    Self {
      head: AllocatorNode::new(0)
    }
  }
}

unsafe impl GlobalAlloc for Allocator {
  unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
    0 as *mut u8
  }

  unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
    0 as *mut u8
  }

  unsafe fn realloc(&self, ptr: *mut u8, layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
    0 as *mut u8
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
    
  }
}