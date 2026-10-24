use crate::process::{
	Pid
};

/// The number of bytes in a single register on the
/// target architechure.
const REGISTER_BYTES: usize = core::mem::size_of::<usize>();

/// The number of bytes in a RAM page
#[cfg(not(any(feature = "mid-memory", feature = "high-memory", feature = "desktop-memory")))]
pub const PAGE_SIZE: usize = 512;
#[cfg(all(feature = "mid-memory", not(feature = "high-memory"), not(feature = "desktop-memory")))]
pub const PAGE_SIZE: usize = 1024;
#[cfg(all(feature = "high-memory", not(feature = "desktop-memory")))]
pub const PAGE_SIZE: usize = 2048;
#[cfg(feature = "desktop-memory")]
pub const PAGE_SIZE: usize = 4096;

// Make sure `BreakInfo` is always as big as 2 RAM pages
#[cfg(not(feature = "desktop-memory"))]
pub const MAX_PAGES: usize = (PAGE_SIZE / REGISTER_BYTES) * 2 - 1;
// Keep the `BreakInfo` the same size now including a
// pointer to the next one so desktop programs can
// use a basically unlimited amount of RAM. (Obviously
// still limited by the system's total amount of RAM)
#[cfg(feature = "desktop-memory")]
pub const MAX_PAGES: usize = 510;

/// Information about the memory a process
/// currently has allocated to it.
#[cfg(not(feature = "desktop-memory"))]
pub struct BreakInfo {
	pub size: usize,
	pub pages: [usize; MAX_PAGES]
}
#[cfg(feature = "desktop-memory")]
pub struct BreakInfo {
	/// The number of pages left:
	/// If this is the first break list
	/// entry then it is the total.
	pub size_left: usize,
	pub pages: [usize; MAX_PAGES],
	pub next: usize
}

/// An error from the allocator.
pub enum AllocatorError {
	NotEnoughMemory,
	MaxBreak,
	NotEnoughSlots,
	DoesNotExist
}

/// All allocators must implement this.
pub trait Allocator {
	/// Allocates a new page to a process and adds it to the program break. Not thread safe.
	unsafe fn allocate_page(&mut self, process: Pid) -> Result<usize, AllocatorError>;
	/// Get the current number of free memory pages. Not thread safe.
	unsafe fn free_pages(&mut self) -> usize;
	/// Deallocate a page. Not thread safe.
	unsafe fn deallocate_page(&mut self, page: usize) -> Result<(), AllocatorError>;
	/// Get the `BreakInfo` instance for a process. Not thread safe.
	unsafe fn get_pages(&mut self, process: Pid) -> Result<*BreakInfo, AllocatorError>;
	/// Spawn a new process. Not thread safe.
	unsafe fn add_process(&mut self, pid: Pid) -> Result<(), AllocatorError>;
	/// Remove a process. Must also be called on the
	/// `tynix::process::ExecutionManager` instance. Not thread safe.
	unsafe fn remove_process(&mut self, pid: Pid) -> Result<(), AllocatorError>;
	/// Get the page ID based on the page's index in a process's program break. Not thread safe.
	unsafe fn get_page_id(&mut self, process: Pid, number: usize) -> Result<usize, AllocatorError>;
	/// Get a pointer to a page. Not thread safe.
	unsafe fn get_page(&mut self, id: usize) -> Result<*mut [u8; PAGE_SIZE], AllocatorError>;
	/// Set the program break for a process.
	unsafe fn brk(process: Pid, value: usize) -> Result<(), AllocatorError> {
		let info: *BreakInfo = unsafe { Self::get_pages(process) };
		#[cfg(not(feature = "desktop-memory"))]
		let current_break: usize = unsafe { (*info).size };
		#[cfg(feature = "desktop-memory")]
		let current_break: usize = unsafe { (*info).size_left };
		if value > current_break {
			if Self::free_pages() < (value - current_break) {
			    return Err(AllocatorError::NotEnoughMemory);
			}
			#[cfg(not(feature = "desktop-memory"))]
			if value > MAX_PAGES {
			    return Err(AllocatorError::NotEnoughSlots);
			}
			for i in 0..(value - current_break) {
			    Self::allocate_page(process);
			}
			return Ok(());
		} else if value < current_break {
		    for i in 0..(current_break - value) {
		        Self::deallocate_page(Self::get_page(process, current_break - 1 - i));
		    }
		    return Ok(());
		}
	}
}
