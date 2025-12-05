use std::fs;

 
pub(crate) 
fn write_benchmark(day: u8, score: String){ 
    let mut content = match fs::read_to_string("readme.md") {
        Ok(contents) => {contents},
        Err(err) => {println!("[Error] Failed to open 'readme.md'");return;},
    };

    let parts: Vec<_> = content.split("id=\"benchmark\"").collect();
    let mut benchmark_parts = "".to_string();
    for line in parts[1].lines(){
        if line.contains(&format!("|{:<2}|",day)) {
            let mut line = line.replace("🌑", "⭐");
            // Check if it already contains some time score
            if line.contains("Time: ") {
                let parts: Vec<_> = line.split("Time: ").collect();
                line = format!("{}Time: {}|",parts[0],score);
            }else{
                line.pop();
                line += &format!(" Time: {}|",score);
            }
            if benchmark_parts != "" {benchmark_parts += "\n";}
            benchmark_parts += &line;
            continue;
        }
        if benchmark_parts != "" {benchmark_parts += "\n";}
        benchmark_parts += line;
    }
    let body = parts[0].to_owned() + "id=\"benchmark\"" + &benchmark_parts +"id=\"benchmark\""+ parts[2];
    
    match fs::write("readme.md", body) {
        Ok(_) => {},
        Err(_) => {},
    };
}
