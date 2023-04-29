import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SignatureRequestComponent } from './signature-request.component';

describe('SignatureRequestComponent', () => {
  let component: SignatureRequestComponent;
  let fixture: ComponentFixture<SignatureRequestComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ SignatureRequestComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SignatureRequestComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
