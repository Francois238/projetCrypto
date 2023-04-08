use crate::api_error::ApiError;

use actix_web::{ post, web,  HttpResponse};
use base64::{engine::general_purpose, Engine};
use crate::generation::*;

#[post("/login")]
pub async fn create_certificate_api(data_certificat: web::Json<DataReceived>) -> Result<HttpResponse, ApiError> {

    let data = data_certificat.into_inner();

    let cert = data.csr_content.clone();

    let cer = general_purpose::STANDARD.decode(cert.as_bytes()).unwrap();

    let cer = String::from_utf8(cer).unwrap();

    let req_certificat = valide_csr(cer, data.mail)?;

    let certificat = create_certificate(req_certificat)?;

    Ok(HttpResponse::Ok().json(certificat))


}



pub fn routes_user(cfg: &mut web::ServiceConfig) {
    cfg.service(create_certificate_api);

}