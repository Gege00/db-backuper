use tiberius::{Client, Config, AuthMethod, error::Error};
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tokio::net::TcpStream;
use std::{fs, str::FromStr};
use serde::{Serialize, Deserialize};
use serde_json::Result as SerdeResult;




#[derive(Serialize, Deserialize)]
pub struct BackUpConfig{

    pub dbAddress: String,
    pub dbPort: String,
    pub dbUser: String,
    pub dbPassword: String,
    pub defaultDBs: Vec<String>

}

fn load_config(file_path:String) -> BackUpConfig {

    let content = fs::read_to_string(file_path).expect("Couldn't read the file!");

    let config: BackUpConfig = serde_json::from_str(&content).unwrap();

   

    return config;

} 


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let back_up_config = load_config("D:/dev/db-backuper/config.json".to_string());
    
    print!("DB Address: {}", back_up_config.dbAddress);
    let mut config = Config::new();

    config.host(back_up_config.dbAddress);
    config.port(back_up_config.dbPort.parse::<u16>().unwrap());
    config.authentication(AuthMethod::sql_server(back_up_config.dbUser, back_up_config.dbPassword));

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let rows  = client.simple_query("SELECT * FROM cars").await?.into_first_result().await?;

    print!("Row count: {}", rows.len());

    

    Ok(())
}