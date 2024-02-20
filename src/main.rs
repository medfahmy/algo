fn main() {
    // let mut data = 10;
    // let ref1 = &mut data;
    // let ref2 = &mut *ref1;
    //
    // *ref2 += 2;
    // *ref1 += 1;

    // println!("{}", data);

    // unsafe {
    //     let mut data = 10;
    //     let ref1 = &mut data;
    //     let ptr2 = ref1 as *mut _;
    //
    //     *ref1 += 1;
    //     *ptr2 += 2;
    //
    //     println!("{}", data);
    // }

    unsafe {
        let mut data = 0;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        *ptr2 += 2;

        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{}", data);
    }
}
