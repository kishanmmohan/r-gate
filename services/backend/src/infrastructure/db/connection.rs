use sea_orm::{ConnectionTrait, Database as SeaORMDatabase, DatabaseConnection, Schema};
use crate::infrastructure::config::BaseSettings;


#[derive(Debug)]
pub struct Database{
    connection: DatabaseConnection
}

impl Database{
    pub async fn new(settings:BaseSettings) -> Result<Self, Box<dyn std::error::Error>>{
        let connection_string = settings.construct_db_url();
        let connection = SeaORMDatabase::connect(connection_string).await;
        
        match connection { 
            Ok(conn) => { Ok(Self{connection: conn}) }
            Err(err) => { Err(format!("Unable to connect to database: {}", err).into()) }
        }
        
    }
    
    // pub async fn init_tables(&self) -> anyhow::Result<bool>{
    // 
    //     let builder = self.connection.get_database_backend();
    //     let schema = Schema::new(builder);
    //     let stmt = builder.build(&schema.create_table_from_entity(UserEntity));
    //     self.connection.execute(stmt).await?;
    //     
    //     Ok(true)
    // }


}