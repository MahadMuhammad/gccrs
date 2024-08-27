//@ check-fail

extern "C" {
    // N.B., mutability can be easily incorrect in FFI calls -- as
    // in C, the default is mutable pointers.
    fn ffi(c: *mut u8);
    fn int_ffi(c: *mut i32);
}

fn static_u8() -> &'static u8 {
    &8
}

unsafe fn ref_to_mut() {
    let num = &3i32;

    let _num = &mut *(num as *const i32 as *mut i32);
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *(num as *const i32).cast_mut();
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *std::ptr::from_ref(num).cast_mut();
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *std::ptr::from_ref({ num }).cast_mut();
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *{ std::ptr::from_ref(num) }.cast_mut();
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *(std::ptr::from_ref({ num }) as *mut i32);
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *(num as *const i32).cast::<i32>().cast_mut();
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *(num as *const i32).cast::<i32>().cast_mut().cast_const().cast_mut();
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *(std::ptr::from_ref(static_u8()) as *mut i8);
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *std::mem::transmute::<_, *mut i32>(num);
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *(std::mem::transmute::<_, *mut i32>(num) as *mut i32);
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *std::cell::UnsafeCell::raw_get(
// { dg-error "" "" { target *-*-* } .-1 }
        num as *const i32 as *const std::cell::UnsafeCell<i32>
    );

    let deferred = num as *const i32 as *mut i32;
    let _num = &mut *deferred;
// { dg-error "" "" { target *-*-* } .-1 }
    let deferred = (std::ptr::from_ref(num) as *const i32 as *const i32).cast_mut() as *mut i32;
    let _num = &mut *deferred;
// { dg-error "" "" { target *-*-* } .-1 }
    let deferred_rebind = deferred;
    let _num = &mut *deferred_rebind;
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *(num as *const _ as usize as *mut i32);
// { dg-error "" "" { target *-*-* } .-1 }
    let _num = &mut *(std::mem::transmute::<_, *mut _>(num as *const i32) as *mut i32);
// { dg-error "" "" { target *-*-* } .-1 }

    static NUM: &'static i32 = &2;
    let num = NUM as *const i32 as *mut i32;
    let num = num;
    let num = num;
    let _num = &mut *num;
// { dg-error "" "" { target *-*-* } .-1 }

    let cell = &std::cell::UnsafeCell::new(0);
    let _num = &mut *(cell as *const _ as *mut i32);
// { dg-error "" "" { target *-*-* } .-1 }

    unsafe fn generic_ref_cast_mut<T>(this: &T) -> &mut T {
        &mut *((this as *const _) as *mut _)
// { dg-error "" "" { target *-*-* } .-1 }
    }

    fn as_mut<T>(x: &T) -> &mut T {
        unsafe { &mut *std::cell::UnsafeCell::raw_get(x as *const _ as *const _) }
// { dg-error "" "" { target *-*-* } .-1 }
    }

    fn as_mut_i32(x: &i32) -> &mut i32 {
        unsafe { &mut *std::cell::UnsafeCell::raw_get(x as *const _ as *const _) }
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

unsafe fn assign_to_ref() {
    let s = String::from("Hello");
    let a = &s;
    let num = &3i32;

    *(a as *const _ as *mut _) = String::from("Replaced");
// { dg-error "" "" { target *-*-* } .-1 }
    *(a as *const _ as *mut String) += " world";
// { dg-error "" "" { target *-*-* } .-1 }
    *std::ptr::from_ref(num).cast_mut() += 1;
// { dg-error "" "" { target *-*-* } .-1 }
    *std::ptr::from_ref({ num }).cast_mut() += 1;
// { dg-error "" "" { target *-*-* } .-1 }
    *{ std::ptr::from_ref(num) }.cast_mut() += 1;
// { dg-error "" "" { target *-*-* } .-1 }
    *(std::ptr::from_ref({ num }) as *mut i32) += 1;
// { dg-error "" "" { target *-*-* } .-1 }
    *std::mem::transmute::<_, *mut i32>(num) += 1;
// { dg-error "" "" { target *-*-* } .-1 }
    *(std::mem::transmute::<_, *mut i32>(num) as *mut i32) += 1;
// { dg-error "" "" { target *-*-* } .-1 }
    std::ptr::write(
// { dg-error "" "" { target *-*-* } .-1 }
        std::mem::transmute::<*const i32, *mut i32>(num),
        -1i32,
    );
    *((&std::cell::UnsafeCell::new(0)) as *const _ as *mut i32) = 5;
// { dg-error "" "" { target *-*-* } .-1 }

    let value = num as *const i32 as *mut i32;
    *value = 1;
// { dg-error "" "" { target *-*-* } .-1 }
    let value = num as *const i32;
    let value = value as *mut i32;
    *value = 1;
// { dg-error "" "" { target *-*-* } .-1 }
    let value = num as *const i32 as *mut i32;
    *value = 1;
// { dg-error "" "" { target *-*-* } .-1 }
    let value_rebind = value;
    *value_rebind = 1;
// { dg-error "" "" { target *-*-* } .-1 }
    *(num as *const i32).cast::<i32>().cast_mut() = 2;
// { dg-error "" "" { target *-*-* } .-1 }
    *(num as *const _ as usize as *mut i32) = 2;
// { dg-error "" "" { target *-*-* } .-1 }
    std::ptr::write(value, 2);
// { dg-error "" "" { target *-*-* } .-1 }
    std::ptr::write_unaligned(value, 2);
// { dg-error "" "" { target *-*-* } .-1 }
    std::ptr::write_volatile(value, 2);
// { dg-error "" "" { target *-*-* } .-1 }

    unsafe fn generic_assign_to_ref<T>(this: &T, a: T) {
        *(this as *const _ as *mut _) = a;
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

#[repr(align(16))]
struct I64(i64);

#[repr(C)]
struct Mat3<T> {
    a: Vec3<T>,
    b: Vec3<T>,
    c: Vec3<T>,
}

#[repr(C)]
struct Vec3<T>(T, T, T);

unsafe fn bigger_layout() {
    {
        let num = &mut 3i32;

        let _num = &*(num as *const i32 as *const i64);
// { dg-error "" "" { target *-*-* } .-1 }
        let _num = &mut *(num as *mut i32 as *mut i64);
// { dg-error "" "" { target *-*-* } .-1 }
        let _num = &mut *(num as *mut i32 as *mut I64);
// { dg-error "" "" { target *-*-* } .-1 }
        std::ptr::write(num as *mut i32 as *mut i64, 2);
// { dg-error "" "" { target *-*-* } .-1 }

        let _num = &mut *(num as *mut i32);
    }

    {
        let num = &mut [0i32; 3];

        let _num = &mut *(num as *mut _ as *mut [i64; 2]);
// { dg-error "" "" { target *-*-* } .-1 }
        std::ptr::write_unaligned(num as *mut _ as *mut [i32; 4], [0, 0, 1, 1]);
// { dg-error "" "" { target *-*-* } .-1 }

        let _num = &mut *(num as *mut _ as *mut [u32; 3]);
        let _num = &mut *(num as *mut _ as *mut [u32; 2]);
    }

    {
        let num = &mut [0i32; 3] as &mut [i32];

        let _num = &mut *(num as *mut _ as *mut i128);
// { dg-error "" "" { target *-*-* } .-1 }
        let _num = &mut *(num as *mut _ as *mut [i64; 4]);
// { dg-error "" "" { target *-*-* } .-1 }

        let _num = &mut *(num as *mut _ as *mut [u32]);
        let _num = &mut *(num as *mut _ as *mut [i16]);
    }

    {
        let mat3 = Mat3 { a: Vec3(0i32, 0, 0), b: Vec3(0, 0, 0), c: Vec3(0, 0, 0) };

        let _num = &mut *(&mat3 as *const _ as *mut [[i64; 3]; 3]);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let _num = &*(&mat3 as *const _ as *mut [[i64; 3]; 3]);
// { dg-error "" "" { target *-*-* } .-1 }

        let _num = &*(&mat3 as *const _ as *mut [[i32; 3]; 3]);
    }

    {
        let mut l: [u8; 2] = [0,1];
        let w: *mut [u16; 2] = &mut l as *mut [u8; 2] as *mut _;
        let w: *mut [u16] = unsafe {&mut *w};
// { dg-error "" "" { target *-*-* } .-1 }
    }

    {
        fn foo() -> [i32; 1] { todo!() }

        let num = foo();
        let _num = &*(&num as *const i32 as *const i64);
// { dg-error "" "" { target *-*-* } .-1 }
        let _num = &*(&foo() as *const i32 as *const i64);
// { dg-error "" "" { target *-*-* } .-1 }
    }

    {
        fn bar(_a: &[i32; 2]) -> &[i32; 1] { todo!() }

        let num = bar(&[0, 0]);
        let _num = &*(num as *const i32 as *const i64);
        let _num = &*(bar(&[0, 0]) as *const i32 as *const i64);
    }

    {
        fn foi<T>() -> T { todo!() }

        let num = foi::<i32>();
        let _num = &*(&num as *const i32 as *const i64);
// { dg-error "" "" { target *-*-* } .-1 }
    }

    {
        let x: Box<dyn Send> = Box::new(0i32);
        let _z = unsafe { &*(&*x as *const dyn Send as *const i32) };
    }

    unsafe fn from_ref(this: &i32) -> &i64 {
        &*(this as *const i32 as *const i64)
    }

    // https://github.com/rust-lang/rust/issues/124685
    unsafe fn slice_index(array: &mut [u8], offset: usize) {
        let a1 = &mut array[offset];
        let a2 = a1 as *mut u8;
        let a3 = a2 as *mut u64;
        unsafe { *a3 = 3 };
    }

    unsafe fn field_access(v: &mut Vec3<i32>) {
        let r = &mut v.0;
        let ptr = r as *mut i32 as *mut Vec3<i32>;
        unsafe { *ptr = Vec3(0, 0, 0) }
    }

    unsafe fn deref(v: &mut Vec3<i32>) {
        let r = &mut v.0;
        let r = &mut *r;
        let ptr = &mut *(r as *mut i32 as *mut Vec3<i32>);
        unsafe { *ptr = Vec3(0, 0, 0) }
    }
}

const RAW_PTR: *mut u8 = 1 as *mut u8;
unsafe fn no_warn() {
    let num = &3i32;
    let mut_num = &mut 3i32;
    let a = &String::from("ffi");

    *(num as *const i32 as *mut i32);
    println!("{}", *(num as *const _ as *const i16));
    println!("{}", *(mut_num as *mut _ as *mut i16));
    ffi(a.as_ptr() as *mut _);
    int_ffi(num as *const _ as *mut _);
    int_ffi(&3 as *const _ as *mut _);
    let mut value = 3;
    let value: *const i32 = &mut value;
    *(value as *const i16 as *mut i16) = 42;
    *RAW_PTR = 42; // RAW_PTR is defined outside the function body,
                   // make sure we don't ICE on it when trying to
                   // determine if we should lint on it or not.
    let cell = &std::cell::UnsafeCell::new(0);
    let _num = &mut *(cell.get() as *mut i32);

    fn safe_as_mut<T>(x: &std::cell::UnsafeCell<T>) -> &mut T {
        unsafe { &mut *std::cell::UnsafeCell::raw_get(x as *const _ as *const _) }
    }

    fn cell_as_mut(x: &std::cell::Cell<i32>) -> &mut i32 {
        unsafe { &mut *std::cell::UnsafeCell::raw_get(x as *const _ as *const _) }
    }

    #[repr(transparent)]
    struct DoesContainUnsafeCell(std::cell::UnsafeCell<i32>);
    fn safe_as_mut2(x: &DoesContainUnsafeCell) -> &mut DoesContainUnsafeCell {
        unsafe { &mut *std::cell::UnsafeCell::raw_get(x as *const _ as *const _) }
    }
}

fn main() {}

