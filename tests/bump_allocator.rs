//! Pure-memory tests: never initialize hardware or invoke the stress suite.
use hw_ultra::sys::BumpAllocator;
use std::sync::{Arc, Barrier};

#[test]
fn concurrent_reservations_and_boundaries() {
    let allocator = BumpAllocator::new();
    allocator.reset(); // This test binary is the only owner of the shared heap.
    let start = Arc::new(Barrier::new(8));
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let start = Arc::clone(&start);
            std::thread::spawn(move || {
                let allocator = BumpAllocator::new();
                start.wait();
                (0..128)
                    .map(|_| allocator.alloc(16, 16) as usize)
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let mut addresses: Vec<_> = workers
        .into_iter()
        .flat_map(|w| w.join().unwrap())
        .collect();
    assert!(addresses.iter().all(|p| *p != 0 && p % 16 == 0));
    addresses.sort_unstable();
    assert!(addresses.windows(2).all(|pair| pair[1] - pair[0] >= 16));
    assert!(allocator.alloc(8, 0).is_null());
    assert!(allocator.alloc(8, 3).is_null());
    assert!(allocator.alloc(usize::MAX, 8).is_null());
    allocator.reset(); // All worker threads joined; no pointers are used.
    let mut count = 0;
    while !allocator.alloc(1024, 1).is_null() {
        count += 1;
        assert!(count <= 51);
    }
    assert_eq!(count, 51); // floor(64 KiB * 80% / 1 KiB).
    allocator.reset();
    assert!(!allocator.alloc(8, 8).is_null());
}
