use image::DynamicImage;

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        println!("Missing arguments.");
        print_usage_and_exit();
    }

    //
    let infile = args.remove(0);
    let outfile = args.remove(0);

    let mut img: DynamicImage = image::open(infile).expect("Failed to open INFILE.");

    // Loop until args is empty
    while args.len() != 0 {
        let subcommand: String = args.remove(0);
        match subcommand.as_str() {
            // EXAMPLE FOR CONVERSION OPERATIONS
            "blur" => {
                if args.len() < 1 {
                    println!("Missing blur_amount");
                    print_usage_and_exit();
                }

                let blur_amount: f32 = args
                    .remove(0)
                    .parse()
                    .expect("Failed to parse blur_amount for BLUR");

                img = img.blur(blur_amount);
            }

            // **OPTION**
            // Brighten -- see the brighten() function below
            "brighten" => {
                if args.len() < 1 {
                    println!("Missing brighten_amount");
                    print_usage_and_exit();
                }

                let brighten_amount: i32 = args
                    .remove(0)
                    .parse()
                    .expect("Failed to parse brighten_amount for BRIGHTEN");

                img = img.brighten(brighten_amount);
            }

            // **OPTION**
            // Crop -- see the crop() function below
            "crop" => {
                if args.len() < 4 {
                    println!("Missing crop arguments");
                    print_usage_and_exit();
                }

                let x: u32 = args.remove(0).parse().expect("Failed to parse x for CROP");
                let y: u32 = args.remove(0).parse().expect("Failed to parse y for CROP");
                let width: u32 = args
                    .remove(0)
                    .parse()
                    .expect("Failed to parse width for CROP");
                let height: u32 = args
                    .remove(0)
                    .parse()
                    .expect("Failed to parse height for CROP");

                img = img.crop(x, y, width, height)
            }

            // **OPTION**
            // Rotate -- see the rotate() function below
            "rotate" => {
                if args.len() < 1 {
                    println!("Missing rotation_amount");
                    print_usage_and_exit();
                }

                let rotation_amount: u32 = args.remove(0)
                    .parse()
                    .expect("Failed to parse rotation_amount for ROTATE");
                if rotation_amount == 90 {
                    img = img.rotate90();
                } else if rotation_amount == 180 {
                    img = img.rotate180();
                } else if rotation_amount == 270 {
                    img = img.rotate270();
                } else {
                    println!("Invalid rotation_amount. Must be 90, 180, or 270.");
                    print_usage_and_exit();
                }
            }

            // **OPTION**
            // Invert -- see the invert() function below
            "invert" => {
                img.invert();
            }

            "grayscale" => {
                img = img.grayscale();
            }

            // For everything else...
            _ => {
                println!("Unknown subcommand: {}", subcommand);
                print_usage_and_exit();
            }
        }
    }

    // Save the image to OUTFILE
    img.save(outfile)
        .expect("Failed saving OUTFILE");
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!();
    println!("INFILE OUTFILE <CHAIN OF FILTERS>");
    println!();
    println!("Available filters:");
    println!("    blur BLUR_AMOUNT");
    println!("    brighten BRIGHTEN_AMOUNT");
    println!("    crop X Y WIDTH HEIGHT");
    println!("    rotate DEGREES{{90, 180, 270}}");
    println!("    invert");
    println!("    grayscale");

    std::process::exit(-1);
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
