import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AppComponent } from './app.component';
import { ConfirmationComponent } from './confirmation/confirmation.component';
import { SignatureRequestComponent } from './signature-request/signature-request.component';
import { SupprimerComponent } from './supprimer/supprimer.component';
import { TelechargementComponent } from './telechargement/telechargement.component';
import { ContactComponent } from './contact/contact.component';

const routes: Routes = [

  { path: 'app-routing', component: AppComponent },
  //{ path: 'signature-request', component: SignatureRequestComponent },
  { path: '', component: SignatureRequestComponent },
  { path: 'supprimer', component: SupprimerComponent},
  { path: 'confirmation', component: ConfirmationComponent },
  { path: 'contact', component: ContactComponent},
  { path: 'telechargement', component: TelechargementComponent }

];



@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})

export class AppRoutingModule { }
