use image::{GenericImageView};
use std::{env};
fn main() {
    let mut bool_val = false;
    let mut bool_val2 = false;
    let args: Vec<String> = env::args().collect();
    match (&args[2]).parse::<u32>(){
        Ok(_i) => bool_val=true,
        Err(..) => println!("this was not an integer"),
        };

    match (&args[3]).parse::<u32>(){
        Ok(_i) => bool_val2=true,
        Err(..) => println!("this was not an integer"),
        };


    if bool_val && bool_val2{
       foo(&args[1], &args[2], &args[3]); }    
/*    let img = image::open(&args[1]).unwrap();
    println!("dimensions of original image are {:?}", img.dimensions());
    let newimg = img.resize(10,10, image::imageops::Nearest);
    newimg.save("newimage.png").unwrap();
    println!("dimensions {:?}", newimg.dimensions());   */
}

fn foo(x:&String, y:&String, z:&String){
    println!("{}", y);
        let y_int = y.parse::<u32>().unwrap();
        let z_int = z.parse::<u32>().unwrap();
        let img = image::open(x).unwrap();
        println!("dimensions of original image are {:?}", img.dimensions());
        let newimg = img.resize(y_int,z_int, image::imageops::Nearest);
        newimg.save("newimage.png").unwrap();
        println!("dimensions {:?}", newimg.dimensions());

    }
