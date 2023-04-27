import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
//import { CsrFormComponent } from './csr-form/csr-form.component';
import { SignatureRequestComponent } from './signature-request/signature-request.component';
import { ReactiveFormsModule } from '@angular/forms';
import { ConfirmationComponent } from './confirmation/confirmation.component';
import { TelechargementComponent } from './telechargement/telechargement.component';
import { SupprimerComponent } from './supprimer/supprimer.component';
import {HttpClientModule} from '@angular/common/http';
import { ContactComponent } from './contact/contact.component';

@NgModule({
  declarations: [
    AppComponent,
    //CsrFormComponent,
    SignatureRequestComponent,
    ConfirmationComponent,
    TelechargementComponent,
    SupprimerComponent,
    ContactComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    ReactiveFormsModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})

export class AppModule { }
