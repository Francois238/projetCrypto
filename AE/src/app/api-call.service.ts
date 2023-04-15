import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { CsrSent } from './csr-sent';
import { CodeSent } from './code-sent';
import { CertificateReceived } from './certificate-received';

@Injectable({
  providedIn: 'root'
})
export class ApiCallService {

  constructor(private http: HttpClient) { }

  public sendCsr(csr: CsrSent): Observable<void> {

    const headers = { 'content-type': 'application/json'}
    const body=JSON.stringify(csr);

    const url = 'http://localhost:8080/send_csr';
    return this.http.post<void>(url, body ,{'headers':headers})
    
  }


  public SendCode(code: CodeSent): Observable<CertificateReceived> {

    const headers = { 'content-type': 'application/json'}
    const body=JSON.stringify(code);

    const url = 'http://localhost:8080/send_code';
    return this.http.post<CertificateReceived>(url, body ,{'headers':headers})
    
  }
}
