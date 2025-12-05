use std::fs;

 
pub(crate) 
fn write_benchmark(day: u8, score: String){ 
    let content = match fs::read_to_string("readme.md") {
        Ok(contents) => {contents},
        Err(err) => {println!("[Error] Failed to open 'readme.md': {err}");return;},
    };

    let parts: Vec<_> = content.split("id=\"benchmark\"").collect();
    let mut benchmark_parts = "".to_string();
    for line in parts[1].lines(){
        if line.contains(&format!("|{:<2}|",day)) {
            // Write score
            let mut cols: Vec<String> = line.replace("🌑", "⭐").split("|").map(|f| f.to_string()).collect();  
            // cols: ["", "2 ", "⭐", "⭐", " 358.66ms", ""]
            cols[4] = format!(" {} ",score);
            if benchmark_parts != "" {benchmark_parts += "\n";}
            benchmark_parts += &cols.join("|");
            continue;
        }
        if benchmark_parts != "" {benchmark_parts += "\n";}
        benchmark_parts += line;
    }
    let body = parts[0].to_owned() + "id=\"benchmark\"" + &benchmark_parts +"id=\"benchmark\""+ parts[2];
    
    match fs::write("readme.md", body) {
        Ok(_) => {},
        Err(err) => {println!("[Error] Failed to write to 'readme.md': {err}")},
    };
}
