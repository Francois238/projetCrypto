import { Component } from '@angular/core';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { ApiCallService } from '../api-call.service';
import { CodeSent } from '../code-sent';
import { CertificateReceived } from '../certificate-received';
import * as ClipboardJS from 'clipboard';

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

  copyToClipboard() {
    const textToCopy = `Votre code otp est : ${this.otp}\n\nVotre certificat:\n${this.certificate}\n\nLe certificat de l'autorité:\n${this.certificateChain}`;
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

      console.log(this.form.value.otp);

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

          this.certificate = window.atob(certificatRecu);
          this.certificateChain = window.atob(data.certicate_chain);
          this.otp = data.otp;

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

