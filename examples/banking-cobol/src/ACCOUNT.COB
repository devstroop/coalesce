       IDENTIFICATION DIVISION.
       PROGRAM-ID. ACCOUNT-MANAGER.
       AUTHOR. COALESCE-DEMO.
       
       ENVIRONMENT DIVISION.
       
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-ACCOUNT-RECORD.
           05 WS-ACCOUNT-ID     PIC X(10).
           05 WS-CUSTOMER-NAME  PIC X(30).
           05 WS-BALANCE        PIC 9(8)V99.
           05 WS-ACCOUNT-TYPE   PIC X(10).
           05 WS-STATUS         PIC X(6).
           
       01 WS-TRANSACTION.
           05 WS-TRANS-TYPE     PIC X(10).
           05 WS-TRANS-AMOUNT   PIC 9(6)V99.
           
       01 WS-OVERDRAFT-LIMIT   PIC 9(6)V99 VALUE 1000.00.
       01 WS-MIN-BALANCE       PIC 9(4)V99 VALUE 100.00.
       
       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM INITIALIZE-ACCOUNT.
           PERFORM PROCESS-TRANSACTIONS.
           PERFORM DISPLAY-ACCOUNT-STATUS.
           STOP RUN.
           
       INITIALIZE-ACCOUNT.
           MOVE "ACC001" TO WS-ACCOUNT-ID.
           MOVE "JOHN DOE" TO WS-CUSTOMER-NAME.
           MOVE 5000.00 TO WS-BALANCE.
           MOVE "CHECKING" TO WS-ACCOUNT-TYPE.
           MOVE "ACTIVE" TO WS-STATUS.
           DISPLAY "Account initialized for " WS-CUSTOMER-NAME.
           
       PROCESS-TRANSACTIONS.
           MOVE "DEPOSIT" TO WS-TRANS-TYPE.
           MOVE 1500.00 TO WS-TRANS-AMOUNT.
           PERFORM PROCESS-DEPOSIT.
           
           MOVE "WITHDRAW" TO WS-TRANS-TYPE.
           MOVE 2000.00 TO WS-TRANS-AMOUNT.
           PERFORM PROCESS-WITHDRAWAL.
           
       PROCESS-DEPOSIT.
           ADD WS-TRANS-AMOUNT TO WS-BALANCE.
           DISPLAY "Deposited: $" WS-TRANS-AMOUNT.
           DISPLAY "New Balance: $" WS-BALANCE.
           
       PROCESS-WITHDRAWAL.
           IF WS-TRANS-AMOUNT > WS-BALANCE + WS-OVERDRAFT-LIMIT
               DISPLAY "Insufficient funds for withdrawal"
               DISPLAY "Available: $" WS-BALANCE + WS-OVERDRAFT-LIMIT
           ELSE
               SUBTRACT WS-TRANS-AMOUNT FROM WS-BALANCE
               DISPLAY "Withdrawn: $" WS-TRANS-AMOUNT
               DISPLAY "New Balance: $" WS-BALANCE
               PERFORM CHECK-MINIMUM-BALANCE
           END-IF.
           
       CHECK-MINIMUM-BALANCE.
           IF WS-BALANCE < WS-MIN-BALANCE
               DISPLAY "WARNING: Balance below minimum"
               IF WS-BALANCE < 0
                   MOVE "OVERDRAWN" TO WS-STATUS
               END-IF
           END-IF.
           
       DISPLAY-ACCOUNT-STATUS.
           DISPLAY "=== ACCOUNT SUMMARY ===".
           DISPLAY "Account ID: " WS-ACCOUNT-ID.
           DISPLAY "Customer: " WS-CUSTOMER-NAME.
           DISPLAY "Type: " WS-ACCOUNT-TYPE.
           DISPLAY "Balance: $" WS-BALANCE.
           DISPLAY "Status: " WS-STATUS.
