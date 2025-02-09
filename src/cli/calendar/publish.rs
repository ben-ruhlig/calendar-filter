use anyhow::Result;

pub fn publish_setup() -> Result<()> {
    println!("TODO: Implement publish setup");
    Ok(())
}

pub fn publish_course(course: &Option<String>) -> Result<()> {
    if let Some(course) = course {
        println!("TODO: Implement publish course");
        println!("Course: {}", course);
    } else {
        println!("TODO: Implement better error message");
        println!("Invalid: Must provide a course by ID or Name");
    }
    Ok(())
}

pub fn publish_all() -> Result<()> {
    println!("TODO: Implement publish all");
    Ok(())
}

pub fn publish_filtered() -> Result<()> {
    println!("TODO: Implement publish filtered");
    Ok(())
}
