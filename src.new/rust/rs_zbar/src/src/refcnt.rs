#[cfg(any(windows, target_os="macos"))]
use std::sync::atomic::{Ordering, AtomicI32};
#[cfg(not(any(windows, target_os = "macos")))]
use std::sync::{Mutex, Arc};

pub type refcnt_t = i32;

#[cfg(target_os = "windows")]
pub fn _zbar_refcnt(cnt: refcnt_t, mut delta: i32){
    let mut rc = -1;
    let arc = AtomicI32::new(cnt);
    if delta > 0{
        delta -= 1;
        while delta > 0{
            let _ = arc.fetch_add(1, Ordering::Relaxed);
            rc = arc.load(Ordering::Relaxed);
            delta -= 1;
        }
    } else if delta < 0{
        delta += 1;
        while delta < 0{
            let _ = arc.fetch_sub(1, Ordering::Relaxed);
            rc = arc.load(Ordering::Relaxed);
            delta += 1;
        }
    }
    assert!(rc >= 0);
    return rc;
}

#[cfg(target_os = "macos")]
pub fn _zbar_refcnt(cnt: refcnt_t, delta: i32) -> i32{
    let arc = AtomicI32::new(cnt);
    let _ = arc.fetch_add(delta, Ordering::Relaxed);
    let rc = arc.load(Ordering::Relaxed);
    assert!(rc >= 0);
    rc
}

#[cfg(not(any(windows, target_os = "macos")))]
pub fn _zbar_refcnt(cnt: refcnt_t, mut delta: i32) -> i32{
    let arc = Arc::new(Mutex::new(cnt));
    let rc = arc.lock().unwrap();
    *rc += delta;
    assert!(*rc >= 0);
    *rc
}

pub fn _zbar_refcnt_init(){}