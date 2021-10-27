
use std::env;
use std::error::Error;
use image::io::Reader as ImageReader;
use std::time::Instant;

fn main()  {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Invalid args");
        std::process::exit(1);
    }

    let file = &args[1];


    let now = Instant::now();
    let img = ImageReader::open(file).unwrap().decode().unwrap();
    println!("Image read time: {}(millis)", now.elapsed().as_millis());  
    //  1ms     =  1,000,000 ns	
    //  0.1 ms  =    100,000 ns	
    // 	0.01 ms =     10,000 ns

    //println!("{:?}", img);
    //let img2 = ImageReader::new(Cursor::new(bytes)).decode()?;
//    img.save("empty.jpg").unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut bytes, image::ImageOutputFormat::Png).unwrap();

    println!("original image bytes: {}", bytes.len());

    let bytes2 = bytes.clone();

    let mapped: Vec<u8> = bytes.into_iter()
                    .filter(|x| x > &225)
                    .collect();
//    println!("{:?}", mapped);
    println!("filtered image bytes: {}", mapped.len());

    let now = Instant::now();

    let r: Vec<_> = bytes2.into_iter()
                    .enumerate()
                    .filter(|&(i,val)| val > 250)
                    .map(|(index,val)| (index, val))
                    .collect();
    println!("Filter time: {}(millis)", now.elapsed().as_millis());  

    for (i,v) in r{
        //println!("({}) ,", i)
    }    
    
    let v = [true, false, true];

    let result: Vec<_> = v.iter()
        .enumerate()
        .filter(|&(_, &value)| !value )
        .map(|(i, v)| (i,v))
        .collect();

    println!("{:?}", result);
    println!("Total time: {}(millis)", now.elapsed().as_millis());  


}
