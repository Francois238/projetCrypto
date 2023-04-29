# __Projet cryptographie 2023 : Réalisation d'une PKI pour signature de mail__

## Installation

Le projet a été réalisé dans un environnement linux, les installations suivantes sont valides pour ubuntu.

* De rust :  
  
 `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`  
   
Vous aurez également besoin de ce paquet pour la compilation (normalement déjà installé sous ubuntu) :  
  

`sudo apt install build-essential`  
  
Vérifiez également que openssl est installé sinon entrez : `sudo apt-get install openssl`  
  
* De angular :  
  
Si vous n'avez pas angular d'installé, lancez les commandes suivantes :  
* pour installer le gestionnaire de paquets npm  
`sudo apt install npm ` 
* pour installer le gestionnaire de version de nodeJS  
`sudo npm install -g n `  
* pour installer nodeJS  
`sudo n stable `   
* pour installer la CLI angular  
`sudo npm install -g @angular/cli `  
  
## Lancement des programmes  

* du backend en rust :

Dans un terminal, accécez au dossier backend puis lancez : `cargo run`  
Lors du 1er lancement, le programme va être compilé ce qui peut prendre un certain temps  

* du front-end en angular :  
  
Dans un second terminal, accéder au dossier AE puis
* Pour la 1ère éxecution, entrez  
  `npm i ` pour installer les dépendances nécessaires  
* Pour chaque démarrage du front-end , lancez  
  `ng serve `

## Utilisation  

Dans votre navigateur, vous accédez au site via [http://localhost:4200/](http://localhost:4200/)  
  
Vous aurez besoin de générer une paire de clé, si vous ne l'avez pas fait, voici la commande :   
`openssl ecparam -name prime256v1 -genkey -out mykey.key`  
  
Pour créer votre CSR, entrez la commande suivante :  
`openssl req -new -key mykey.key -out moncertificat.csr`  
  
Sur la page *Demande de certificat*, entrez votre mail et copier-coller votre CSR. Vous serez redirigé vers une page de confirmation pour valider un code que vous recevez par mail.
  
Une fois votre certificat reçu, conservez soigneusement votre code OTP si vous devez révoquer votre certificat.  
Enregistrez votre certificat dans un fichier au format .crt, de même pour le certificat de l'autorité.
  
* Si c'est votre 1ère délivrance de certificat par notre autorité, veuillez importer le certificat de notre autorité dans les autorités de certifications de votre client mail.

Convertissez votre certificat au format pkcs12 afin qu'il soit valide pour votre client mail.  
La commande est :  
`openssl pkcs12 -export -in moncertificat.crt -inkey mykey.key -out moncertificat.p12`  
  
Si vous utilisez Thunderbird, décochez la case *Interroger le répondeur OCSP pour confirmer la validité de vos certificats* situé à la fin de la page *Vie privée et sécurité*  
  
Allez ensuite dans *Paramètres des comptes* -> sélectionnez votre mail puis sur *Chiffrement de bout en bout*. Dans la section S/MIME, cliquez sur sélectionner un certificat puis choisissez votre certificat.  
Cochez la case *Signer les messages nons chiffés* afin de signer par défaut les mails non chiffrés.

  
