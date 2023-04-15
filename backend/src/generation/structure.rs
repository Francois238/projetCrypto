use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CsrReceived { //Structure recue pour creer un certificat
    pub mail : String,
    pub csr_content: String,
}


#[derive(Serialize, Deserialize)]
pub struct CodeReceived { //Structure recue pour confirmer le mail
    pub code: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Confirmation { //Structure enregistree pour verifier mail
    pub mail : String,
    pub confirmation_code: String,
    pub req_csr : String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CertificatStored { //Structure enregistree pour stocker le certificat 
    pub mail : String,
    pub otp: String,
    pub certificat : String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CertificatSend{
    pub certificate : String,
    pub certicate_chain: String,
    pub otp : String,
}

