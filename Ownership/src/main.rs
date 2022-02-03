fn main(){
    /*
    ------------Ownership Rules in Rust----------
    1. Each value in Rust has a varibale that is it's owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope the value will be dropped.
    */

    {   // s is not valid here as it is not declared here.
        let s = "hello"; // s is valid from now onwards. | It is stored on Stack.
        // do random stuff with s.
        println!("{}",s);
    } // The scope is now ended and the value of s is dropped from now onwards.
    /* 
        In the above scope s was a normal string literal. And String Literals are 
        stored directly into the binary and are of fixed size. So, To have a string 
        Dynamic in size and the one which we can mutate, We must use String Type. See
        the next block for it.
    */

    {
        let s = String::from("hello_1"); // This is stored onto Heap.
        println!("{}",s);
        /*
            In Other lang such as C++ and C, We manually have to allocate and de-allocate
            the memory onto the Heap. But in Rust, Using appropriate Syntax RustC will
            automatically allocate the space into the Heap and When the scope of the variable 
            will end, Then the created instance inside heap will automatically be de-allocated.
        */
    }

    let x = 5; // Variable created in Stack.
    let y = x; // Copy x into y. | Variable created in Stack.
    println!("{}",x); 
    println!("{}",y);

    {
        let s1 = "Harshil"; // Present in Stack
        let s2 = s1; // Copied S1 into S2 from Stack
        println!("{}",s1); 
        println!("{}",s2);
    }

    {
       let s1 = String::from("Harshil"); // Present in Heap
       let s2 = s1;  // Moved (Not Shallow Copy)
       // New Instance of S1 will not be created in Rust. (Also Called as Cloning)
       // Neither S2 and S1 Both point to the Address of ALlocated Heap Space.
       // Here simply, The Instance of S1 will be passed on to S2. And S1 will get dropped.
       // println!("{}",s1);  //--> This line will give Borrowed Value Error.
       println!("{}",s2);
    }

    {
        let s1 = String::from("Rustican");
        let s2 = s1.clone();
        println!("{}",s1);
        println!("{}",s2);
    }
    {
        let s1 = String::from("Indian!"); // Heap
        takes_ownerShip(s1);
        // println!("{}",s1); --> This line will not be executed as the ownership of the variable has been taken away.
    
        let n = 5; // Stack
        makes_copy(n);
        println!("{}",n);
    }
    {
        let s1 = gives_ownership();
        println!("{}",s1);
        let s2 = String::from("Demet Ozdemir");
        let s3 = takes_and_gives_back(s2);
        // println!("{}",s2) --> Won't Work as Ownership of S2 is taken.
        println!("{}",s3);
    }

   // -------------------------------------References and Borrowing-----------------------------------------------
    
   // This is definately not the way I want things to work. This is cumbersome. So References and Alias will help me get over.
    {
        let s1 = String::from("Phenomenality");
        let (s2,len) = calculate_length(s1);  
        println!("The length of {} is {}",s2,len);
    }
    // Optimized Method of Above Implementation
    {
        let s1 = String::from("Incarnation");
        let length = calculate_length_again(&s1);
        println!("The length of {} is {}",s1,length);
    }

    {
        let mut s1 = String::from("Linux");
        change(&mut s1);
        println!("{}",s1);
    }
    {
        let mut s = String::from("Open Source");
        let r1 = &mut s;
        let r2 = &mut s;
       // println!("{},{}",r1,r2); This fails because a same mutable variable cannot be borrowed by more than one values.
    }
    // To fix the above problem change the mutable variable to immutable so you can borrow it more than once.
    {
        let mut s = String::from("Open Source");
        let r1 = &s; // The immutable reference of S, taken by r1 starts here.
        let r2 = &s; // The immutable reference of S, taken by r2 starts here.
        println!("{},{}",r1,r2); // The immutable reference of S, taken by r1 and r2 ends here.
        let r3 = &mut s; // As the immutable reference was ended in above line, You can start mutable reference.
        println!("{}",r3);// It was not possible to execute it in between when it was not ended.
    }
     /*
    ------------References Rules in Rust----------
    1. At any given time you can have either one mutable reference or any number of immutable reference in a particular scope.
    2. References must be valid. (refer dangle function below)
    */
    {
        //let reference_to_nothing = dangle();
        //println!("{}",reference_to_nothing);
    }

    /*
    -------------------------------------------Slicing------------------------------
    Slices let you reference a contiguous sequence of elements in a collection
    rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
    */
    {
        let s1 = String::from("Harshil Jani");
        let word = first_word_index(&s1);
        let name = &s1[..word];
        let surname = &s1[word+1..];
        println!("{} {}",name,surname);
    }
    {
        let s1 = String::from("Harshil Jani");
        let first_name = first_word(&s1);
        println!("{}",first_name);
    }
}

fn takes_ownerShip(s: String){
    println!("{}",s);
}

fn makes_copy(k: u32){
    println!("{}",k);
}

fn gives_ownership() -> String{
    let some_str = String::from("Can Yaman!");
    some_str // Equivalent to ---> { return some_str; }
}

fn takes_and_gives_back(s: String) -> String{
    let something = s;
    something
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s,length)
}

fn calculate_length_again(s: &String) -> usize{
    let length = s.len();
    length
}

fn change(s: &mut String){
    s.push_str(" Foundation");
}

/*
fn dangle() -> &String{
    let s = String::from("Harshil"); // The reference to S gets dropped here itself as the scope ends.
    &s // Cannot return anything that is memory waste.
}
*/

fn first_word_index(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (index,&items) in bytes.iter().enumerate(){
        if items== b' '{
            return index;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (index,&items) in bytes.iter().enumerate(){
        if items == b' '{
            return &s[0..index];
        }
    }
    s
}

/*
The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
The Rust language gives you control over your memory usage in the same way as other systems programming 
languages, but having the owner of data automatically clean up that data when the owner goes out of scope 
means you don’t have to write and debug extra code to get this control.
*/