use sea_orm::{ DatabaseConnection};

pub struct Database{
    pub connection: DatabaseConnection
}

impl Database{
    pub async fn new(&self) -> Self {
        
    }
}