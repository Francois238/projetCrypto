
use std::fs;

use crate::api_error::ApiError;
use actix_web::{ post, web,  HttpResponse};
use base64::{engine::general_purpose, Engine};
use crate::generation::*;


#[post("/send_csr")]
pub async fn received_csr(data_certificat: web::Json<CsrReceived>) -> Result<HttpResponse, ApiError> {

    let data = data_certificat.into_inner();

    let csr_base64 = data.csr_content.clone();

    let csr_vec = general_purpose::STANDARD.decode(csr_base64.as_bytes()).unwrap();

    let csr = String::from_utf8(csr_vec).unwrap();

    let _req_certificat = valide_csr(csr.clone(), data.mail.clone())?;

    let _verification_mail = send_mail(data.mail.clone(), csr_base64).await?;

    Ok(HttpResponse::Ok().finish())

}

#[post("/send_code")]
pub async fn received_code(data_received: web::Json<CodeReceived>) -> Result<HttpResponse, ApiError> {

    let data = data_received.into_inner();

    let verif = verification_code(data.code)?;

    let csr_base64 = verif.0;

    let mail = verif.1;

    let csr_vec = general_purpose::STANDARD.decode(csr_base64.as_bytes()).unwrap();

    let csr = String::from_utf8(csr_vec).unwrap();

    let certificat = create_certificate(csr)?;

    let certificat_encoded = general_purpose::STANDARD.encode(certificat.as_bytes());

    let otp = save_certificate(mail, certificat_encoded.clone())?;

    let ca_chain_stored = fs::read_to_string("ca_chain.crt").expect("erreur pas de chain.crt");

    let ca_chain = general_purpose::STANDARD.encode(ca_chain_stored.as_bytes());


    let certificat_envoye = CertificatSend {
        certificate : certificat_encoded,
        certicate_chain : ca_chain,
        otp : otp
    };

    Ok(HttpResponse::Ok().json(certificat_envoye))



}




pub fn routes_user(cfg: &mut web::ServiceConfig) {
    cfg.service(received_csr);
    cfg.service(received_code);

}