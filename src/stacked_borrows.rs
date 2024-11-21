pub fn basic_borrow() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        // Access things in "borrow stack" order
        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{}", data);
    }
}

pub fn pointer_mess() {
    #![allow(unused)]
    unsafe {
        let mut data = [0; 10];
        let ref1_at_0 = &mut data[0]; // Reference to 0th element
        let ptr2_at_0 = ref1_at_0 as *mut i32; // Ptr to 0th element
        let ptr3_at_0 = ptr2_at_0; // Ptr to 0th element
        let ptr4_at_0 = ptr2_at_0.add(0); // Ptr to 0th element
        let ptr5_at_0 = ptr3_at_0.add(1).sub(1); // Ptr to 0th element

        // An absolute jumbled hash of ptr usages
        *ptr3_at_0 += 3;
        *ptr2_at_0 += 2;
        *ptr4_at_0 += 4;
        *ptr5_at_0 += 5;
        *ptr3_at_0 += 3;
        *ptr2_at_0 += 2;
        *ref1_at_0 += 1;

        // Should be [20, 0, 0, ...]
        println!("{:?}", &data[..]);
    }
}
