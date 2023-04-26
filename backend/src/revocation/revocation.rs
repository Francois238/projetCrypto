
use std::fs::{self, File};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::api_error::ApiError;

use crate::generation::CertificatStored;





pub fn revocation_ext(mail : String, otp : String, motif_revocation : String) -> Result<(), ApiError> {

    let tab_cert_file = fs::read_to_string("liste_certificats.json").map_err(|_| ApiError::new(404, "Votre certificat n'existe pas".to_string()))?;

    let mut tab_cert : Vec<CertificatStored> = serde_json::from_str(&tab_cert_file).unwrap();

    let mut index = 0;

    let mut found = false;

    for cert in tab_cert.iter() {

        if cert.mail == mail && cert.otp == otp {

            found = true;

            break;

        }

        index += 1;

    }

    if found {

        let certificat = tab_cert.remove(index);

        revocation(certificat.certificat.clone())?; //on révoque le certificat

        fs::remove_file(certificat.certificat).map_err(|_| ApiError::new(404, "Votre certificat n'existe pas".to_string()))?;

        let tab_cert = serde_json::to_string(&tab_cert).unwrap();

        fs::write("liste_certificats.json", tab_cert).map_err(|_| ApiError::new(500, "erreur interne".to_string()))?;

        store_motif(motif_revocation)?; //on enregistre le motif de révocation

        

        Ok(())

    } else {

        Err(ApiError::new(404, "Votre certificat n'existe pas".to_string()))

    }

    

}


fn store_motif(motif : String) -> Result<(), ApiError> {

    let mut tab_motif : Vec<String> = Vec::new();

    let tab_motif_file = fs::read_to_string("liste_motifs.json");

    if tab_motif_file.is_ok(){ //le fichier existe

        tab_motif = serde_json::from_str(&tab_motif_file.map_err(|_| ApiError::new(500, "Impossible de lire fichier motif revocation".to_string()))?).map_err(|_| ApiError::new(500, "Impossible de lire fichier motif revocation".to_string()))?; //on recupere le contenu du fichier

    }

    tab_motif.push(motif);

    let tab_motif = serde_json::to_string(&tab_motif).map_err(|_| ApiError::new(500, "Impossible d'enregistrer fichier motif revocation".to_string()))?;

    fs::write("liste_motifs.json", tab_motif).map_err(|_| ApiError::new(500, "Impossible d'enregistrer fichier motif revocation".to_string()))?;

    Ok(())

}



fn revocation(file : String) -> Result<(), ApiError> {

     //venir recuperer l'id du processus du serveur ocsp pour pouvoir le tuer et le relancer

    let id = fs::read_to_string("ocsp_process_id").map_err(|_| ApiError::new(500, "Impossible de lire le process id".to_string()))?;

    let id = id.parse::<u32>().map_err(|_| ApiError::new(500, "Impossible de lire le process id".to_string()))?;


//commande a execute : openssl ca -revoke file -keyfile ACI.key -cert ACI.crt

    Command::new("openssl")
    .arg("ca")
    .arg("-revoke")
    .arg(file)
    .arg("-keyfile")
    .arg("ACI.key")
    .arg("-cert")
    .arg("ACI.crt")
    .spawn()
    .map_err(|_| ApiError::new(500, "Impossible de révoquer le certificat".to_string()))?;

    kill_ocsp_server(id)?; //tue le processus du serveur ocsp

    run_ocsp_server()?; // relance le serveur ocsp

    Ok(())


}


pub fn run_ocsp_server() -> Result<(), ApiError>{


    let file = Path::new("demoCA/index.txt");

    if !file.exists() { //si le fichier n'existe pas on le crée
        let _index = File::create("demoCA/index.txt").map_err(|_| ApiError::new(500, "Impossible de créer le fichier index.txt".to_string()))?;
    }
    

    //executer commande suivant : openssl ocsp -index ../demoCA/index.txt -port 9999 -rsigner ocsp.crt -rkey ocsp.key -CA ACI.crt -text -out /tmp/ocsp.log

    let process = Command::new("openssl")
    .arg("ocsp")
    .arg("-index")
    .arg("demoCA/index.txt")
    .arg("-port")
    .arg("9999")
    .arg("-rsigner")
    .arg("ocsp.crt")
    .arg("-rkey")
    .arg("ocsp.key")
    .arg("-CA")
    .arg("ACI.crt")
    .arg("-text")
    .arg("-out")
    .arg("/tmp/ocsp.log")
    .spawn()
    .map_err(|_| ApiError::new(500, "Impossible de lancer le serveur ocsp".to_string()))?;

    let id = process.id();

    fs::write("ocsp_process_id", id.to_string()).map_err(|_| ApiError::new(500, "Impossible d'enregistrer le process id".to_string()))?;

    Ok(())

}


fn kill_ocsp_server(process : u32) -> Result<(), ApiError> {

    Command::new("kill")
    .arg(process.to_string())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .map_err(|_| ApiError::new(500, "Impossible de tuer le serveur ocsp".to_string()))?;

    Ok(())

}