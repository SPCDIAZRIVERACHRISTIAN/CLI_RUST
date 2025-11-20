pub fn wcs(content: &str) {                                         
    let new_lines = content.lines().count();                        
    let mut word_count = 0;                                         
    for line in content.lines() {                                   
        let count = line.split_whitespace().count();                
        word_count += count;                                        
    }                                                               
                                                                    
    println!("new lines: {} word count: {}", new_lines, word_count);
}                                                                   
