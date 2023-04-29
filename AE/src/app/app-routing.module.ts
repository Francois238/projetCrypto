import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AppComponent } from './app.component';
import { ConfirmationComponent } from './confirmation/confirmation.component';
import { SignatureRequestComponent } from './signature-request/signature-request.component';
import { SupprimerComponent } from './supprimer/supprimer.component';
import { ContactComponent } from './contact/contact.component';

const routes: Routes = [

  { path: 'app-routing', component: AppComponent },
  { path: '', component: SignatureRequestComponent },
  { path: 'supprimer', component: SupprimerComponent},
  { path: 'confirmation', component: ConfirmationComponent },
  { path: 'contact', component: ContactComponent}

];



@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})

export class AppRoutingModule { }
