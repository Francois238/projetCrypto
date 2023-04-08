use std::fs;

use openssl::{x509::X509Req, rand};
use serde::{Serialize, Deserialize};

use crate::api_error::ApiError;




#[derive(Serialize, Deserialize, Clone)]
pub struct DataReceived { //Structure inseree en BDD pour ajouter un user
    pub mail : String,
    pub csr_content: String,
}

pub fn valide_csr(csr_content: String ,mail_user : String) -> Result<X509Req, ApiError> {

   // let csr_content = fs::read_to_string("mycertif.csr").expect("Should have been able to read the file");

    println!("CSR : {}", csr_content);

    let csr = X509Req::from_pem(csr_content.as_bytes()); //converti le csr en pem

    if csr.is_err() {
        return Err(ApiError::new(400, "Le CSR n'est pas un PEM".to_string())); //en cas d erreur
    }

    let csr = csr.unwrap();

        // Extraire le nom commun (CN) du sujet
    let subject = csr.subject_name();

    let cn = subject.entries_by_nid(openssl::nid::Nid::COMMONNAME).next();

    if cn.is_none() {
        return Err(ApiError::new(400, "Le CSR ne contient pas de CN".to_string()));
    }

    let cn = cn.unwrap().data().as_utf8();

    if cn.is_err() {
        return Err(ApiError::new(400, "Le CN du CSR n'est pas un UTF-8".to_string()));
    }

    let cn = cn.unwrap();

    println!("Nom commun (CN) : {}", cn);
        
        // Extraire l'adresse e-mail du sujet
    let email = subject.entries_by_nid(openssl::nid::Nid::PKCS9_EMAILADDRESS).next();


    if email.is_none() {
        return Err(ApiError::new(400, "Le CSR ne contient pas d'adresse e-mail".to_string()));
    }

    let email = email.unwrap().data().as_utf8();

    if email.is_err() {
        return Err(ApiError::new(400, "L'adresse e-mail du CSR n'est pas un UTF-8".to_string()));
    }

    let email = email.unwrap();

    if email.to_string() != mail_user {
        return Err(ApiError::new(400, "L'adresse e-mail du CSR ne correspond pas à l'adresse e-mail du compte".to_string()));
    }

    println!("Adresse e-mail : {}", email);

    
    //extraire la clé publique du csr
    
    let public_key = csr.public_key();

    if public_key.is_err() {
        return Err(ApiError::new(400, "La clé publique du CSR n'est pas valide".to_string()));
    }

    let public_key = public_key.unwrap();
    
    let valide = csr.verify(&public_key);

    if valide.is_err() {
        return Err(ApiError::new(400, "Le CSR n'est pas valide".to_string()));
    }

    let valide = valide.unwrap();

    if valide == false {
        return Err(ApiError::new(400, "Le CSR n'est pas valide".to_string()));
    }

    Ok(csr)

}


pub fn create_certificate(csr : X509Req) -> Result<String, ApiError> {

    let ca_key_string = fs::read_to_string("ACI.key").expect("Should have been able to read the file");

    let ca_key = openssl::pkey::PKey::private_key_from_pem(ca_key_string.as_bytes()).unwrap();

    let ca_cert_string = fs::read_to_string("ACI.crt").expect("Should have been able to read the file");

    let ca_cert = openssl::x509::X509::from_pem(ca_cert_string.as_bytes()).unwrap();


    let mut buf = [0; 20];
    rand::rand_bytes(&mut buf).unwrap();

    let serial = openssl::bn::BigNum::from_slice(&buf).unwrap().to_asn1_integer().unwrap();


       //creer un certificat avec les infos du csr

    let cert = openssl::x509::X509::builder();

    if cert.is_err() {
        return Err(ApiError::new(500, "Erreur lors de la création du certificat".to_string()));
    }
   
    let mut cert = cert.unwrap();
   
    cert.set_version(2).unwrap();
   
    //mettre aussi le serial number du certificat
   
    cert.set_serial_number(&serial).unwrap();
   
       //issuer du certificat
   
    let issuer = ca_cert.issuer_name();
   
    cert.set_issuer_name(issuer).unwrap();

    let subject = csr.subject_name();
   
    cert.set_subject_name(subject).unwrap();

    let public_key = csr.public_key().unwrap();
   
    cert.set_pubkey(&public_key).unwrap();
   
    cert.set_not_before(&openssl::asn1::Asn1Time::days_from_now(0).unwrap()).unwrap();
   
    cert.set_not_after(&openssl::asn1::Asn1Time::days_from_now(365).unwrap()).unwrap();
   
    //ajouter l'extension key usage au certificat pour la signature
      
    let mut key_usage = openssl::x509::extension::KeyUsage::new();
   
    key_usage.digital_signature();
   
    cert.append_extension(key_usage.build().unwrap()).unwrap();
   
    //le certificat ne fait que de la signature de mail
   
    let mut extended_key_usage = openssl::x509::extension::ExtendedKeyUsage::new();
   
    extended_key_usage.email_protection();
   
    cert.append_extension(extended_key_usage.build().unwrap()).unwrap();
   
    cert.sign(&ca_key, openssl::hash::MessageDigest::sha256()).unwrap();
   
    let cert = cert.build();
   
    let cert = cert.to_pem().unwrap();
   
    let certificat = String::from_utf8(cert.clone()).unwrap(); //lecture du certificat en string
   
    println!("Certificat : {}", certificat);
   
    fs::write("cert.pem", cert).expect("Unable to write file");


    Ok(certificat)



}