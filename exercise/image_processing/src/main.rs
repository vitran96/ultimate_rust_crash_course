use image::DynamicImage;

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args()
        .skip(1)
        .collect();
    if args.len() < 3 {
        print_usage_and_exit();
    }
    
    //
    let infile = args.remove(0);
    let outfile = args.remove(0);
    
    let mut img: DynamicImage = image::open(infile)
        .expect("Failed to open INFILE.");

    let subcommand = args.remove(0);

    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            let blur_amount: f32 = args.remove(0)
                .parse()
                .unwrap_or(2.0);
            blur(infile, outfile, blur_amount);
        }

        // **OPTION**
        // Brighten -- see the brighten() function below
        "brighten" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let brighten_amount: i32 = args.remove(0)
                    .parse()
                    .unwrap_or(10);
            brighten(infile, outfile, brighten_amount);
        }

        // **OPTION**
        // Crop -- see the crop() function below
        "crop" => {
            if args.len() < 6 {
                print_usage_and_exit();
            }
            
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0)
                    .parse()
                    .expect("Failed to parse x for CROP");
            let y: u32 = args.remove(0)
                    .parse()
                    .expect("Failed to parse y for CROP");
            let width: u32 = args.remove(0)
                    .parse()
                    .expect("Failed to parse width for CROP");
            let height: u32 = args.remove(0)
                    .parse()
                    .expect("Failed to parse height for CROP");

            crop(infile, outfile, x, y, width, height);
        }

        // **OPTION**
        // Rotate -- see the rotate() function below
        "rotate" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let rotation_amount: u32 = args.remove(0)
                    .parse()
                    .unwrap_or(90);
            rotate(infile, outfile, rotation_amount)
                .expect("Failed to rotate image");
        }

        // **OPTION**
        // Invert -- see the invert() function below
        "invert" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert(infile, outfile);
        }

        "grayscale" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    // println!("blur INFILE OUTFILE");
    // println!("brighten INFILE OUTFILE");
    // println!("crop INFILE OUTFILE X Y WIDTH HEIGHT");
    // println!("rotate INFILE OUTFILE DEGREES{{90, 180, 270}}");

    // println!("grayscale INFILE OUTFILE");
    std::process::exit(-1);
}

fn blur(img: DynamicImage, blur_amount: f32) -> DynamicImage {
    // Here's how you open an existing image file
    img.blur(blur_amount)
}

fn brighten(img: DynamicImage, brighten_amount: i32) -> DynamicImage {
    img.brighten(brighten_amount)
}

fn crop(
    mut img: DynamicImage
    , outfile: String
    , x: u32
    , y: u32
    , width: u32
    , height: u32
) -> DynamicImage {
    img.crop(x, y, width, height)
}

fn rotate(infile: DynamicImage, rotation_amount: u32) {
    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.
    // let img2: DynamicImage;
    // if rotation_amount == 90 {
    //     img2 = img.rotate90();
    // } else if rotation_amount == 180 {
    //     img2 = img.rotate180();
    // } else if rotation_amount == 270 {
    //     img2 = img.rotate270();
    // } else {
    //     panic!("Invalid rotation amount")
    // }

    // Ok(String::from("Ok"))
}

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let mut img: DynamicImage = image::open(infile).expect("Failed to open INFILE.");
    
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();
    
    // See blur() for an example of how to save the image.
    img.save(outfile)
        .expect("Failed writing OUTFILE.");
}

fn grayscale(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
