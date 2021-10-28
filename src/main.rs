
use std::env;
use image::io::Reader as ImageReader;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Invalid args");
        std::process::exit(1);
    }
    let file = &args[1];

    let mut now = Instant::now();
    let img = read_image(file).expect(&format!("Failed to open open file {}", file));
    println!("Image read time: {}(millis)", now.elapsed().as_millis());  

    now = Instant::now();
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut bytes, image::ImageOutputFormat::Png)?;    
    let original_bytes = bytes.len();
    println!("Writing image bits to buffer in: {}(millis)", now.elapsed().as_millis());  

    now = Instant::now();

    let threshold: u8 = 240;
    // filter the image u8 pixels and retrieve only the index and values 
    // where the values are greater than threshold
    let iv: Vec<_> = bytes.into_iter()
                    .enumerate()
                    .filter(|&(_,val)| val > threshold)
                    .map(|(index,val)| (index, val))
                    .collect();
    println!("Original image bytes: {}", original_bytes);
    println!("Filtered image bytes: {}", iv.len());
    println!("Filter time: {}(millis)", now.elapsed().as_millis());     

    Ok(())
}


fn read_image(file: &String) -> Result<image::DynamicImage, image::ImageError> {
    let img_reader = ImageReader::open(file)?; //
    let img = img_reader.decode()?; 
    Ok(img)
}


#[test]
fn sanity_check_vector_filter(){
    let v = [true, false, true, true, false, true];
    let result: Vec<_> = v.into_iter()
        .enumerate()
        .filter(|&(_, value)| value == true )
        .map(|(i, v)| (i,v))
        .collect();

    println!("{:?}", result);
    let expected: Vec<(usize,bool)> = vec![ (0,true), (2,true),(3,true), (5,true)];
    println!("{:?}", expected);
    assert_eq!(result, expected);
}
