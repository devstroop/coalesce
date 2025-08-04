       IDENTIFICATION DIVISION.
       PROGRAM-ID. CALCULATE-INTEREST.
       AUTHOR. COALESCE-DEMO.
       
       ENVIRONMENT DIVISION.
       
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-PRINCIPAL       PIC 9(7)V99.
       01 WS-RATE           PIC 99V99.
       01 WS-TIME           PIC 99.
       01 WS-INTEREST       PIC 9(7)V99.
       01 WS-COMPOUND-AMT   PIC 9(8)V99.
       01 WS-COUNTER        PIC 99.
       
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY "=== BANKING INTEREST CALCULATOR ===".
           DISPLAY "Enter Principal Amount: ".
           ACCEPT WS-PRINCIPAL.
           DISPLAY "Enter Interest Rate (%): ".
           ACCEPT WS-RATE.
           DISPLAY "Enter Time Period (years): ".
           ACCEPT WS-TIME.
           
           PERFORM CALCULATE-SIMPLE-INTEREST.
           PERFORM CALCULATE-COMPOUND-INTEREST.
           
           STOP RUN.
           
       CALCULATE-SIMPLE-INTEREST.
           COMPUTE WS-INTEREST = 
               WS-PRINCIPAL * WS-RATE * WS-TIME / 100.
           DISPLAY "Simple Interest: $" WS-INTEREST.
           
       CALCULATE-COMPOUND-INTEREST.
           MOVE WS-PRINCIPAL TO WS-COMPOUND-AMT.
           PERFORM VARYING WS-COUNTER FROM 1 BY 1 
               UNTIL WS-COUNTER > WS-TIME
               COMPUTE WS-COMPOUND-AMT = WS-COMPOUND-AMT * 
                   (1 + WS-RATE / 100)
           END-PERFORM.
           COMPUTE WS-INTEREST = WS-COMPOUND-AMT - WS-PRINCIPAL.
           DISPLAY "Compound Interest: $" WS-INTEREST.
           DISPLAY "Final Amount: $" WS-COMPOUND-AMT.
