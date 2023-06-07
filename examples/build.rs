
fn main() {

    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
         .compile(
             &["proto/helloworld/helloworld.proto"],
             &["proto/helloworld"],
         ).unwrap();
    
 }