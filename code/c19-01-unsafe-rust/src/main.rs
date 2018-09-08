use std::slice;

// The "C" part defines which application binary interface (ABI) the external function uses: the
// ABI defines how to call the function at the assembly level. The "C" ABI is the most common and
// follows the C programming language’s ABI.
extern "C" {
    fn abs(i: i32) -> i32;
}

static mut COUNTER: i32 = 0;

fn main() {
    let mut num = 5;
    // Raw pointers, still safe.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32; // mutable reference to the same location

    // Not so safe...
    let address = 0x012345usize;
    let r = address as *const i32;

    // We must use an unsafe block to dereference raw pointers
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("Address is: {}", *r); // segfault!
    }

    // Unsafe functions must be called inside unsafe blocks
    unsafe {
        dangerous_function();
    }

    // Calling a safe abstraction over unsafe code
    let mut v = vec![1,2,3,4,5,6,7];
    let (v1, v2) = slice_at_mut(&mut v, 3);
    println!("{:?} <-> {:?}", v1, v2);

    // Calling a C function
    unsafe {
        let x = -3;
        println!("The absolute value of {} according to C is {}", x, abs(x));
    }

    // Calling a safe abstraction over unsafe code
    print_counter();
    add_to_counter(4);
    print_counter();
}

unsafe fn dangerous_function() { }

fn slice_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut[i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid < len);
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

fn add_to_counter(i: i32) {
    unsafe {
        COUNTER += i;
    }
}

fn print_counter() {
    unsafe {
        println!("Counter: {}", COUNTER);
    }
}

// traits can be unsafe if one of their methods is unsafe
unsafe trait Foo {
}

// they can be implemented unsafely
unsafe impl Foo for i32 {
}

// Unsafe traits include `Send` and `Sync`

// Callable from other languages (this isn't unsafe, just an aside):
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called from C!");
}
