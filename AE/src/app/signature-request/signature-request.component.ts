import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ApiCallService } from '../api-call.service';
import { Router } from '@angular/router';
import { CsrSent } from '../csr-sent';

@Component({
  selector: 'app-signature-request',
  templateUrl: './signature-request.component.html',
  styleUrls: ['./signature-request.component.css']
})
export class SignatureRequestComponent {
  title = 'AE';
  form!: FormGroup;

  csrSent = '';

  messageErreur='';

  constructor(private fb: FormBuilder, private apiCallService: ApiCallService, protected router: Router) {
    this.createForm();
  }

  createForm() {
    this.form = this.fb.group({
      email: ['', [Validators.required, Validators.email]],
      csr: ['', Validators.required]
    });
  }

  onSubmit() {
    if (this.form.valid) {
      // Submit the form

      let mailContent = this.form.value.email as string

      let CsrContent = this.form.value.csr as string

      let mailContenttrim = mailContent.trim(); //enlever les espaces debut et fin au cas ou

      let CsrContenttrim = CsrContent.trim(); //enlever les espaces debut et fin au cas ou

      this.apiCallService.setMail(mailContenttrim); //enregistrer le mail dans le service

      this.csrSent = window.btoa(CsrContenttrim); //encoder en base64

      console.log(this.csrSent);

      let formData : CsrSent = {
        mail: mailContenttrim,
        csr_content: this.csrSent
      }


      this.apiCallService.sendCsr(formData).subscribe({
        next: data => {

          this.router.navigate(['/confirmation']);

        },

        error: err => {

          if(err.status <500){
            this.messageErreur = err.error.message;
          }

          else{
            this.messageErreur = 'Erreur interne';
          }
        }
      })
    }
  }


}

