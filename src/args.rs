use clap:: {                                                               
    args,                                                                  
    parser,                                                                
    subcommand                                                             
  };                                                                       
                                                                           
#[derive(Debug, Parser)]                                                   
#[clap(author, version, about)]                                            
pub struct CLI {                                                           
    #[clap(subcommand)]                                                    
    pub entity_type: EntityType,                                           
}                                                                          
                                                                           
#[derive(Debug, Subcommand)]                                               
pub enum EntityType {                                                      
    /// Displays the line where it contains the argument being past to it. 
    grrs(grrsCommand),                                                     
    ///Displays the word count line count and number of new lines.         
    wcs(wcsCommand)                                                        
}                                                                          
                                                                           
#[derive(Debug, Args)]                                                     
pub struct grrsCommand {                                                   
    ///Pattern you want to look for in the file.                           
    pub pattern: String,                                                   
    ///File path you want to look in.                                      
    pub path: std::path::PathBuf,                                                              
  }                                                                        
