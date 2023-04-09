use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CsrReceived { //Structure inseree en BDD pour ajouter un user
    pub mail : String,
    pub csr_content: String,
}


#[derive(Serialize, Deserialize)]
pub struct DataStored { //Structure inseree en BDD pour ajouter un user
    pub mail : String,
    pub serial_number: String,
}


#[derive(Serialize, Deserialize)]
pub struct CodeReceived { //Structure inseree en BDD pour ajouter un user
    pub mail : String,
    pub code: String,
}
