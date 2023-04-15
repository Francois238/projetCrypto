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

  constructor(private formBuilder: FormBuilder, private apiCallService: ApiCallService) {
    this.form = this.formBuilder.group({
      otp: ['', Validators.required]
    });
  }

  onSubmit() {
    if (this.form.valid) {
      
      console.log(this.form.value.otp);

      let otpData = this.form.value.otp as string

      let OtpTtrim = otpData.trim(); //enlever les espaces debut et fin au cas ou
      
      let formData : CodeSent = {
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
          console.log('There was an error!', err.error.message);

          this.messageErreur = err.error.message;
          }

          else{
            console.log('Erreur interne');
            this.messageErreur = 'Erreur interne';
          }
        }
      });
    }


  }
}

