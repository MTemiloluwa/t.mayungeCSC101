fn main() {
    let v = vec![101, 250, 330, 400];
    //vector v owns the object in heap
    let v2 = v;
    //println!("{:?}", v);
    //correction ro this would be 
    println!("{:?}", v2);
    //or if you want both variables to have the same heap 
  // let v2 = v.clone();
   println!("{:?}", v2);
}
