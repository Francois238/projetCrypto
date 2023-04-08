import { Component } from '@angular/core';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';

@Component({
  selector: 'app-download',
  templateUrl: './telechargement.component.html',
  styleUrls: ['./telechargement.component.css']
})
export class TelechargementComponent {
  form: FormGroup;
  fileSelected: boolean = false;
  fileToUpload: File | null = null;

  constructor(private fb: FormBuilder) {
    this.form = this.fb.group({
      file: ['', Validators.required]
    });
  }

  onSubmit() {
    if (!this.fileToUpload) {
      alert('Veuillez sélectionner un fichier à télécharger.');
      return;
    }

    // Envoyer le fichier au serveur ici
    console.log('Fichier téléchargé :', this.fileToUpload);

    // Réinitialiser le formulaire
    this.form.reset();
    this.fileSelected = false;
    this.fileToUpload = null;
  }

  onFileSelected(event: any) {
    const file = event.target.files[0];
    if (file) {
      this.fileSelected = true;
      this.fileToUpload = file;
    } else {
      this.fileSelected = false;
      this.fileToUpload = null;
    }
  }
}

