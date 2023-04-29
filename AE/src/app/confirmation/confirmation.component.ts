import { Component } from '@angular/core';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { ApiCallService } from '../api-call.service';
import { CodeSent } from '../code-sent';
import { CertificateReceived } from '../certificate-received';

@Component({
  selector: 'app-confirmation',
  templateUrl: './confirmation.component.html',
  styleUrls: ['./confirmation.component.css']
})
export class ConfirmationComponent {
  form: FormGroup;

  otp = '';
  certificate = '';
  messageErreur='';
  certificateChain = '';
  mail = '';

  copyToClipboardCertificatClient() { //copier le certificat client dans le presse papier
    const textToCopy = `${this.certificate}`;
    const textarea = document.createElement('textarea');
    textarea.textContent = textToCopy;
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand('copy');
    document.body.removeChild(textarea);
  }

  copyToClipboardCertificatAuthority() { //copier le certificat de l'autorité dans le presse papier
    const textToCopy = `${this.certificateChain}`;
    const textarea = document.createElement('textarea');
    textarea.textContent = textToCopy;
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand('copy');
    document.body.removeChild(textarea);
  }

  constructor(private formBuilder: FormBuilder, private apiCallService: ApiCallService) {

    this.mail = this.apiCallService.getMail();

    this.form = this.formBuilder.group({
      email: ['', [Validators.required, Validators.email]],
      otp: ['', Validators.required]
    });
  }

  onSubmit() {
    if (this.form.valid) {

      let otpData = this.form.value.otp as string

      let OtpTtrim = otpData.trim(); //enlever les espaces debut et fin au cas ou
      let mailTril = this.mail.trim();

      let formData : CodeSent = {
        mail: mailTril,
        code: OtpTtrim
      }

      this.apiCallService.SendCode(formData).subscribe({
        next: (data : CertificateReceived)=> {
          console.log(data);

          let certificatRecu = data.certificate;

          this.certificate = window.atob(certificatRecu); //décoder le certificat reçu en base64
          this.certificateChain = window.atob(data.certicate_chain); //décoder le certificat de l'autorité recu en base64
          this.otp = data.otp; //otp de revocation

        },
        error: err => {

          if(err.status <500){

            this.messageErreur = err.error.message;
          }

          else{
            this.messageErreur = 'Erreur interne';
          }
        }
      });
    }


  }
}

