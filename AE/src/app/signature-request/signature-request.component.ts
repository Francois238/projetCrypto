import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';

@Component({
  selector: 'app-signature-request',
  templateUrl: './signature-request.component.html',
  styleUrls: ['./signature-request.component.css']
})
export class SignatureRequestComponent {
  form!: FormGroup;

  constructor(private fb: FormBuilder) {
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
    }
  }


}

