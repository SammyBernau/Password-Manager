# Sammy's Safe

- This is a password/account manager that I wrote using the ChaCha20poly1305 algorithm. 
- The user creates a master password that is then saved encrypted and saved into a json which is then used to store all the accounts the user chooses to enter
- The master password is treated as the master key for both entering into the app and decrypting their encrypted account passwords