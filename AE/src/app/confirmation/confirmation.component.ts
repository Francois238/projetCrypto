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
      opt: ['', Validators.required]
    });
  }

  onSubmit() {
    if (this.form.valid) {
      
      console.log(this.form.value.opt);
      
      let formData : CodeSent = {
        code: this.form.value.opt as string
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

