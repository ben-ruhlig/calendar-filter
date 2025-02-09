use anyhow::Result;

pub fn run_filtered() -> Result<()> {
    println!("TODO: Implement calendar run_filtered logic");
    Ok(())
}

pub fn run_all() -> Result<()> {
    println!("TODO: Implement calendar run_all logic");
    Ok(())
}

pub fn run_course(course: &Option<String>) -> Result<()> {
    println!("TODO: Implement calendar run_course logic");
    if let Some(course) = course {
        println!("Course: {}", course);
    } else {
        println!("No course argument was provided");
    }
    Ok(())
}
