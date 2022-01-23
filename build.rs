use image::io::Reader;
use image::imageops::colorops::huerotate;
use rand::random;

fn main() -> Result<(), std::io::Error> {
    match std::fs::create_dir("generated") {
        Err(e) if e.kind() != std::io::ErrorKind::AlreadyExists => return Err(e),
        _ => ()
    };

    // generate puppies from the template
    let template = Reader::open("templates/puppy.png")?.decode().unwrap();
    for i in 0..10 {
        huerotate(&template, random())
            .save(format!("generated/puppy-{i}.png"))
            .expect("couldn't write file!");
    }

    // give the field a natural color
    huerotate(
        &Reader::open(format!("templates/field.png"))?.decode().unwrap(),
        (random::<usize>() % 90 + 30) as i32
    ).save(format!("generated/field.png")).expect("couldn't write file!");

    // the ball can be any color
    huerotate(
        &Reader::open(format!("templates/ball.png"))?.decode().unwrap(),
        random()
    ).save(format!("generated/ball.png")).expect("couldn't write file!");

    // the goal can be any color
    huerotate(
        &Reader::open(format!("templates/goal.png"))?.decode().unwrap(),
        random()
    ).save(format!("generated/goal.png")).expect("couldn't write file!");

    Ok(())
}
