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

      console.log(this.form.value.email);
      console.log(this.form.value.csr);  

      this.csrSent = window.btoa(this.form.value.csr);

      console.log(this.csrSent);

      let formData : CsrSent = {
        mail: this.form.value.email as string,
        csr_content: this.csrSent
      }


      this.apiCallService.sendCsr(formData).subscribe({
        next: data => {

          this.router.navigate(['/confirmation']);
          
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
      })
    }
  }


}

