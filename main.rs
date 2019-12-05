# [derive(Debug)]
struct point<T,U>{
    // x:T,
    // y:T,
    x:T,
    Y:U,
  
}
// fn main() {
//     let integar=point{x:8,y:9};
//     let float=point{x:8.1,y:1.9};
//     println!("{:#?}",integar);
//     println!("{:#?}",float);
// }
// for defining two values in structs using genrics by different datatypes
fn main(){
    let integar =point{x:8, Y:10.2};
    println!("{:#?}",integar);
}