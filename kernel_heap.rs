/// A simple heap that only tracks ownership
pub struct SimpleHeap<
    const size: usize
> {
    data: [u64; size / 8],
    occupied: [u8; size / 8]
}
