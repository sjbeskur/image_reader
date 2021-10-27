
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

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut bytes, image::ImageOutputFormat::Png).unwrap();
    println!("Original image bytes: {}", bytes.len());


    let now = Instant::now();

    let threshold: u8 = 240;

    // filter the image u8 pixels and retrieve only the index and values 
    // where the values are greater than threshold
    let iv: Vec<_> = bytes.into_iter()
                    .enumerate()
                    .filter(|&(_,val)| val > threshold)
                    .map(|(index,val)| (index, val))
                    .collect();

    println!("filtered image bytes: {}\n", iv.len());

    println!("Filter time: {}(millis)", now.elapsed().as_millis());  
    


}

fn sanity_check(){
    let v = [true, false, true];
    let result: Vec<_> = v.iter()
        .enumerate()
        .filter(|&(_, &value)| !value )
        .map(|(i, v)| (i,v))
        .collect();

    println!("{:?}", result);

}
