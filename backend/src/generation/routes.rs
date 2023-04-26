
use std::fs;

use crate::{api_error::ApiError, revocation};
use actix_web::{ post, web,  HttpResponse};
use base64::{engine::general_purpose, Engine};
use regex::Regex;
use crate::generation::*;


#[post("/send_csr")]
pub async fn received_csr(data_certificat: web::Json<CsrReceived>) -> Result<HttpResponse, ApiError> {

    let data = data_certificat.into_inner();

    let csr_base64 = data.csr_content.clone();

    let mail = data.mail.clone();

    let regex = Regex::new(r"(?x)
    ^(?P<login>[^@\s]+)@
    ([[:word:]]+\.)*
    [[:word:]]+$
    ").unwrap();

    if !regex.is_match(&mail) { //verification que le mail envoye est bien un format mail
        return Err(ApiError::new(400, "L'adresse e-mail n'est pas valide".to_string()));
    }

    let csr_vec = general_purpose::STANDARD.decode(csr_base64.as_bytes()).unwrap();

    let csr = String::from_utf8(csr_vec).unwrap();

    let _req_certificat = valide_csr(csr.clone(), data.mail.clone())?;

    let _verification_mail = send_mail(data.mail.clone(), csr_base64).await?;

    Ok(HttpResponse::Ok().finish())

}

#[post("/send_code")]
pub async fn received_code(data_received: web::Json<CodeReceived>) -> Result<HttpResponse, ApiError> {

    let data = data_received.into_inner();

    let mail = data.mail.clone();

    let csr_base64 = verification_code(data.code, &mail)?;

    let csr_vec = general_purpose::STANDARD.decode(csr_base64.as_bytes()).unwrap();

    let csr = String::from_utf8(csr_vec).unwrap();

    let certificat = create_certificate(csr)?;

    let certificat_encoded = general_purpose::STANDARD.encode(certificat.clone().as_bytes());

    let otp = save_certificate(mail,  certificat)?;

    let ca_chain_stored = fs::read_to_string("ca_chain.crt").expect("erreur pas de chain.crt");

    let ca_chain = general_purpose::STANDARD.encode(ca_chain_stored.as_bytes());


    let certificat_envoye = CertificatSend {
        certificate : certificat_encoded,
        certicate_chain : ca_chain,
        otp : otp
    };

    Ok(HttpResponse::Ok().json(certificat_envoye))



}


#[post("/revocation")]
pub async fn received_revocation(data_received: web::Json<RevocationReceived>) -> Result<HttpResponse, ApiError> {

    let data = data_received.into_inner();

    let _ = revocation::revocation_ext(data.mail, data.code, data.motif)?;

    Ok(HttpResponse::Ok().finish())

}




pub fn routes_user(cfg: &mut web::ServiceConfig) {
    cfg.service(received_csr);
    cfg.service(received_code);
    cfg.service(received_revocation);

}