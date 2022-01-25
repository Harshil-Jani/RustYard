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