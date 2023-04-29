import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Revocation } from '../revocation';
import { Router } from '@angular/router';
import { ApiCallService } from '../api-call.service';

@Component({
  selector: 'app-supprimer',
  templateUrl: './supprimer.component.html',
  styleUrls: ['./supprimer.component.css']
})
export class SupprimerComponent {

  form!: FormGroup;
  mail = '';
  otp = '';
  raison='';
  message='';
  

    constructor(private fb: FormBuilder, private apiCallService: ApiCallService, protected router: Router) {
    this.createForm();
  }

  createForm() {
    this.form = this.fb.group({
      email: ['', [Validators.required, Validators.email]],
      otp: ['', [Validators.required]],
      raison: ['', Validators.required]
    });
  }

  onSubmit() {
    if (this.form.valid) {
      let mailContent = this.form.value.email as string
      let OtpContent = this.form.value.otp as string
      let RaisonContent = this.form.value.raison as string

      let OtpTtrim = OtpContent.trim(); //enlever les espaces debut et fin au cas ou
      let mailTrim = mailContent.trim();
      let RaisonTrim = RaisonContent.trim();

      let formData : Revocation = {
        mail: mailTrim,
        code: OtpTtrim,
        motif: RaisonTrim
      }

      this.apiCallService.SendRevocation(formData).subscribe({
        next: (data : void)=> {
          this.message = 'Votre certificat a été révoqué avec succès';

        },
        error: err => {

          if(err.status <500){

          this.message = err.error.message;
          }

          else{
            this.message = 'Erreur interne';
          }
        }
      });




    }
  }


}
