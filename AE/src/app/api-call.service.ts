import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { CsrSent } from './csr-sent';
import { CodeSent } from './code-sent';
import { CertificateReceived } from './certificate-received';
import { Revocation } from './revocation';

@Injectable({
  providedIn: 'root'
})
export class ApiCallService {

  mail : string ='';

  constructor(private http: HttpClient) { }

  public setMail(mail: string): void {
    this.mail = mail;
  }

  public getMail(): string {
    return this.mail;
  }

  public sendCsr(csr: CsrSent): Observable<void> { //envoyer le csr au serveur

    const headers = { 'content-type': 'application/json'}
    const body=JSON.stringify(csr);

    const url = 'http://localhost:8080/send_csr';
    return this.http.post<void>(url, body ,{'headers':headers})
    
  }


  public SendCode(code: CodeSent): Observable<CertificateReceived> { //envoyer le code au serveur

    const headers = { 'content-type': 'application/json'}
    const body=JSON.stringify(code);

    const url = 'http://localhost:8080/send_code';
    return this.http.post<CertificateReceived>(url, body ,{'headers':headers})
    
  }

  public SendRevocation(revocation : Revocation) : Observable<void> { //envoyer la demande de révocation au serveur

    const headers = { 'content-type': 'application/json'}
    const body=JSON.stringify(revocation);

    const url = 'http://localhost:8080/revocation';
    return this.http.post<void>(url, body ,{'headers':headers})
    
  }
}
