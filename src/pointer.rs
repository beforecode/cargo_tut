pub fn run() {
    //Premitive Array
    let arr1: [i32;3] = [1,2,3];
    let arr2 = arr1;
    println!("{:?}", (arr1, arr2));

    // With non-premitives, if you assign another variable to a piece of data, the first variable 
    // will no longer hold that value, You'll need to use a refrence (&) to point to the source.
    let vec1: Vec<u32> = vec![1,2,3,4,5];
    let vec2 = &vec1;
    println!("Vec: {:?}", (&vec1, vec2));
}