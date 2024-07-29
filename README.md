# __Cryptography project 2023: Implementation of a PKI for mail signature__.

## Installation

The project was carried out in a Linux environment, the following installations are valid for ubuntu.

* For rust:  
  
 `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`  
   
You'll also need this package for compilation (should be already installed on ubuntu):  

`sudo apt install build-essential`  
  
Also check that openssl is installed, otherwise enter: `sudo apt-get install openssl`  
  
* For angular :  
  
- to install the npm package manager  
`sudo apt install npm ` 
- to install the nodeJS version manager  
`sudo npm install -g n `  
- to install nodeJS  
`sudo n stable `   
- to install the angular CLI  
`sudo npm install -g @angular/cli `  
  
## Launch of the solution

* backend in rust :

In a terminal, access the backend folder and run: `cargo run`  
On the 1st run, the program will be compiled, which may take some time.  

* front-end in angular :  
  
In a second terminal, access the AE folder then
* For the 1st run, enter  
  `npm i ` to install the necessary dependencies  
* For each front-end startup, run  
  `ng serve `

## Use  

In your browser, access the site via [http://localhost:4200/](http://localhost:4200/)  
  
You'll need to generate a key pair, if you haven't done so, here's the command:   
`openssl ecparam -name prime256v1 -genkey -out mykey.key`  
  
To create your CSR, enter the following command:  
`openssl req -new -key mykey.key -out mycertificate.csr`  
  
On the *Request a certificate* page, enter your e-mail address, copy and paste your CSR. You will be redirected to a confirmation page to validate a code you receive by e-mail.
  
Once you've received your certificate, carefully save your OTP code in case you need to revoke your certificate.  
Save your certificate in a .crt file, as well as the authority's certificate.
  
* If this is your 1st certificate issued by our authority, please import our authority's certificate into your mail client's certification authorities.

Convert your certificate to pkcs12 format so that it is valid for your mail client.  
The command is :  
`openssl pkcs12 -export -in moncertificat.crt -inkey mykey.key -out moncertificat.p12`  
  
If you're using Thunderbird, uncheck the box *Query the OCSP responder to confirm the validity of your certificates* located at the end of the *Privacy and security* page.  
  
Then go to *Account settings* -> select your mail then *End-to-end encryption*. In the S/MIME section, click on select a certificate, then choose your certificate.  
Check the *Sign unencrypted messages* box to sign unencrypted e-mails by default.

  
