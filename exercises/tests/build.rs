//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    let your_command = format!(
<<<<<<< HEAD
        "Your command here with {}, please checkout exercises/tests/build.rs",
=======
        "rustc-env=TEST_FOO={}",
>>>>>>> 1f3cbd2539077ab6471e6235afeea21f46df7860
        timestamp
    );
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
<<<<<<< HEAD
    let your_command = "Your command here, please checkout exercises/tests/build.rs";
    println!("cargo:{}", your_command);
=======
   // let yaour_command = "Your command here , please checkout exercises/tests/build.rs";
    //println!("cargo:{}", yaour_command);
    println!("cargo:rustc-cfg=feature=\"pass\"");
>>>>>>> 1f3cbd2539077ab6471e6235afeea21f46df7860
}
