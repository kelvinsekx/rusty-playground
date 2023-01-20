fn main() {
    greet_world()
}

fn greet_world() {
    println!("Hello World");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

// this code is written by kelvinsekx to share
// as a learning journey. This isn't written by an expert
