use std::fs;


use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use openssl::rand;

use crate::api_error::ApiError;
use crate::generation::Confirmation;


pub async fn send_mail(mail: String,  req_csr : String) -> Result<(), ApiError> {

    let confirmation_code = generate_code();

    store_confirmation(mail.clone(), confirmation_code.clone(), req_csr).await?;

    let nom = "isenprojetcrypto2023@gmail.com";  //mail cree pour le projet

    let email = Message::builder()
    .from(nom.parse().unwrap())
    .to(mail.parse().unwrap())
    .subject("Verification de votre mail")
    .header(ContentType::TEXT_PLAIN)
    .body(format!("Votre code de confirmation est : {}", confirmation_code))
    .unwrap();

let creds = Credentials::new("isenprojetcrypto2023@gmail.com".to_owned(), "pefekpnbfxfiwrjg".to_owned());

// Open a remote connection to gmail
let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::new(400, format!("Erreur lors de l'envoi du mail : {}", e))),
    }

}


fn generate_code() -> String {
    let mut code = [0u8; 4];
    rand::rand_bytes(&mut code).unwrap();

    let code_int = u32::from_be_bytes(code) % 1_000_000;
    format!("{:06}", code_int)
}


pub async fn store_confirmation(mail: String, confirmation_code : String, req_csr : String) -> Result<(), ApiError> {

    println!("Confirmation code : {}", confirmation_code);

    let confirmation = Confirmation {
        mail : mail.clone(),
        confirmation_code : confirmation_code.clone(),
        req_csr : req_csr
    };

    let mut tab_verif : Vec<Confirmation> = Vec::new();

    let tab_verif_file = fs::read_to_string("verification.json");

    if tab_verif_file.is_ok(){ //le fichier existe

        tab_verif = serde_json::from_str(&tab_verif_file.unwrap()).unwrap();

    }

    tab_verif.push(confirmation);

    let tab_verif = serde_json::to_string(&tab_verif).unwrap();

    fs::write("verification.json", tab_verif).expect("Unable to write file");
    
    Ok(())



}


pub fn verification_code(code: String, mail : &str) -> Result<String, ApiError> {

    let tab_verif_file = fs::read_to_string("verification.json");

    if tab_verif_file.is_ok(){ //le fichier existe

        let mut tab_verif : Vec<Confirmation>  = serde_json::from_str(&tab_verif_file.unwrap()).unwrap();

        for index in 0..tab_verif.len()  { //On parcourt le tableau

            let confirmation = &tab_verif[index];

            if confirmation.confirmation_code == code && confirmation.mail == mail  { //si ok

                let csr = confirmation.req_csr.clone();

                tab_verif.remove(index); //On supprime l'element du tableau

                let tab_verif = serde_json::to_string(&tab_verif).unwrap();

                fs::write("verification.json", tab_verif).expect("Unable to write file"); //On ecrit le nouveau tableau

                return Ok(csr);

            }
        }

    }

    Err(ApiError::new(400, "Code de confirmation incorrect".to_string()))

}