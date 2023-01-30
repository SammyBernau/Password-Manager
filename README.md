#&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Sammy's Safe


### What did I make?

I made a password manager that uses the AES-GCM encryption algorithm and SHA256 hash function to store user entered account information. 
    
- SHA256 is used to securely store the master password as a hash value, enabling validation upon user entry. 
- AES-GCM is used in conjunction with the user password (as a plain text saved at runtime which is then removed after) to encrypt/decrypt the user's entries. 

***

#### Version 1.0.1 => Spring 2023
<pre>
- Updated the encryption algorithm to AES-GCM over ChaCha20poly1305
  - Reason 1: ChaCha20poly1305 is excellent when it comes to speed; however, the scale of this project does not cause a noticeable impact on encryption/decryption time
  - Reason 2: AES-GCM is better suited for bulk encryption as ChaCha20poly1305 is aimed more real-time communication (largely seen in mobile devices)
  - Reason 3: AES-GCM is still the industry standard and has the evidence to back it up. ChaCha20poly1305 is still fairly new

- Updated the master password validation system
  - Uses SHA256 hash function to store and later compare user entered master password for when they want to login
  - Reason 1: The original way didn't follow the industry standard
  - Reason 2: Encrypting the master password involved using the master password as input for its own iv which ruined the randomization of the algorithm (large security risk)
</pre>


#### Version 1.0 => Fall 2022
<pre>
- This is a password/account manager that I wrote using the ChaCha20poly1305 algorithm
- The user creates a master password that is then saved and encrypted into a json which is then used to store all the accounts
- The master password is treated as the master key for both entering into the app and decrypting their encrypted account passwords
</pre>