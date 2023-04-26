use std::fs;

use openssl::{x509::{X509Req, X509Extension}, rand, pkey::Id, nid::Nid};
use uuid::Uuid;

use crate::api_error::ApiError;
use base64::{engine::general_purpose, Engine};

use super::CertificatStored;

pub fn valide_csr(csr_content: String ,mail_user : String) -> Result<X509Req, ApiError> {

    let csr = X509Req::from_pem(csr_content.as_bytes()); //convertir le csr en objet rust 
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

    let email = email.map_err(|_| ApiError::new(400, "L'adresse e-mail du CSR n'est pas un UTF-8".to_string()))?;

    if email.to_string() != mail_user {
        return Err(ApiError::new(400, "L'adresse e-mail du CSR ne correspond pas à l'adresse e-mail du compte".to_string()));
    }

    println!("Adresse e-mail : {}", email);

    
    //extraire la clé publique du csr
    
    let public_key = csr.public_key();

    let public_key = public_key.map_err(|_| ApiError::new(400, "La clé publique du CSR n'est pas valide".to_string()))?;
    
    //verifier si la cle est assez robuste 

    let mut robuste = false;

    if (public_key.id() == Id::RSA && public_key.bits() >= 2048) || (public_key.id() == Id::EC && public_key.bits() >= 256){
        robuste = true;
    }

    if robuste == false {
        return Err(ApiError::new(400, "La clé utilisee n'est pas valide. Veuillez utiliser au minimum du RSA 2048 ou ECC 256".to_string()));
    }

    let valide = csr.verify(&public_key);

    let valide = valide.map_err(|_| ApiError::new(400, "Le CSR n'est pas valide".to_string()))?; //en cas d erreur

    if valide == false {
        return Err(ApiError::new(400, "Le CSR n'est pas valide".to_string()));
    }

    Ok(csr)

}


pub fn create_certificate(csr_content : String) -> Result<String, ApiError> {

    //extraction de la clé privée et du certificat de l'ACI

    let ca_key_string = fs::read_to_string("ACI.key").expect("Should have been able to read the file");

    let ca_key = openssl::pkey::PKey::private_key_from_pem(ca_key_string.as_bytes()).unwrap();

    let ca_cert_string = fs::read_to_string("ACI.crt").expect("Should have been able to read the file");

    let ca_cert = openssl::x509::X509::from_pem(ca_cert_string.as_bytes()).unwrap();


    //convertion String en X509Req
    let csr = X509Req::from_pem(csr_content.as_bytes()); //converti le csr en pem

    let csr = csr.map_err(|_| ApiError::new(400, "Le CSR n'est pas un PEM".to_string()))?;


    let mut buf = [0; 20];
    rand::rand_bytes(&mut buf).unwrap();

    let serial = openssl::bn::BigNum::from_slice(&buf).unwrap().to_asn1_integer().unwrap();


       //creer un certificat avec les infos du csr

    let cert = openssl::x509::X509::builder();

    let mut cert = cert.map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;
   
    cert.set_version(2).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;
   
    //mettre aussi le serial number du certificat
   
    cert.set_serial_number(&serial).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;
   
       //issuer du certificat
   
    let issuer = ca_cert.subject_name();
   
    cert.set_issuer_name(issuer).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;

    let subject = csr.subject_name();
   
    cert.set_subject_name(subject).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;

    let public_key = csr.public_key().map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;
   
    cert.set_pubkey(&public_key).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;
   
    cert.set_not_before(&openssl::asn1::Asn1Time::days_from_now(0).unwrap()).unwrap(); //certificat non encore valide
   
    cert.set_not_after(&openssl::asn1::Asn1Time::days_from_now(90).unwrap()).unwrap(); //generer certificat valide 3 mois
   
    //ajouter l'extension key usage au certificat pour la signature

      
    let mut key_usage = openssl::x509::extension::KeyUsage::new();
   
    key_usage.digital_signature(); 
   
    cert.append_extension(key_usage.build().unwrap()).unwrap();

    //rajouter url pour l'ocsp

    let ocsp_url = "http://localhost:9999";
    let info_access = Nid::INFO_ACCESS;
    let info = format!("OCSP;URI:{}", ocsp_url);
    let  ocsp_ext = X509Extension::new_nid(None, None, info_access, info.as_str()).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat extension ocsp 1".to_string()))?;
    cert.append_extension(ocsp_ext).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat  extension ocsp 2".to_string()))?;
   
    //le certificat ne fait que de la signature de mail
   
    let mut extended_key_usage = openssl::x509::extension::ExtendedKeyUsage::new();
   
    extended_key_usage.email_protection();
   
    cert.append_extension(extended_key_usage.build().unwrap()).unwrap();
   
    cert.sign(&ca_key, openssl::hash::MessageDigest::sha256()).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;
   
    let cert = cert.build();
   
    let cert = cert.to_pem().map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;
   
    let certificat = String::from_utf8(cert).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?; //lecture du certificat en string


    Ok(certificat)



}





pub fn generate_otp_revocation() -> Result<String, ApiError> {
    const OTP_LEN: usize = 18;

    let mut password = vec![0; OTP_LEN];
    rand::rand_bytes(&mut password).unwrap();


    let encoded_password =  general_purpose::URL_SAFE.encode(password).replace("_", "a").replace("-", "b"); //pour n avoir que des chiffres et lettres

    let otp = encoded_password[..16].to_string(); //pour enlever les = de la fin au cas ou

    Ok(otp)

    
}


pub fn generate_name_certificate() -> Result<String, ApiError> {
    
    let name =  Uuid::new_v4(); //utilisation d'un uuid pour le nom du certificat

    let name = name.to_string();

    Ok(name)

    
}


pub fn save_certificate(mail: String, certificat : String) -> Result<String, ApiError> {

    let otp = generate_otp_revocation()?; //generation d'un otp de révocation

    let file_path = generate_name_certificate()?; //generation d'un nom de fichier

    let file_path = format!("certificate_stored/{}.crt", file_path); //ajout de l'extension .pem

    fs::write(file_path.clone(), certificat).map_err(|_| ApiError::new(500, "Erreur lors de la création du certificat".to_string()))?;
    

    let certifif = CertificatStored {
        mail : mail.clone(),
        otp : otp.clone(),
        certificat : file_path
    };

    let mut tab_cert : Vec<CertificatStored> = Vec::new();

    let tab_cert_file = fs::read_to_string("liste_certificats.json");

    if tab_cert_file.is_ok(){ //le fichier existe

        tab_cert = serde_json::from_str(&tab_cert_file.unwrap()).unwrap();

    }

    tab_cert.push(certifif);

    let tab_cert = serde_json::to_string(&tab_cert).unwrap();

    fs::write("liste_certificats.json", tab_cert).expect("Unable to write file");


    Ok(otp)



}


 