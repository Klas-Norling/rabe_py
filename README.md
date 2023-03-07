# Rabe_py

This is a rust/python implementation of rabe, covering keygeneration, encryption and decryption. Rabe is a rust library implementing several Attribute Based Encryption (ABE) schemes using a modified version of the `bn` library of zcash (type-3 pairing / Baretto Naering curve). The modification of `bn` brings in `serde` or `borsh` instead of the deprecated `rustc_serialize`.

# Implemented Ciphertext Policy Schemes (CP-ABE)

## BDABE CP-ABE

Georg Bramm, Mark Gall, Julian Schütte , "Blockchain based Distributed Attribute-based Encryption". In Proceedings of the 15th International Joint Conference on e-Business and Telecommunications (ICETE 2018) - Volume 2: SECRYPT, pages 99-110. Available from https://doi.org/10.5220/0006852602650276

## AC17 CP-ABE

Shashank Agrawal, Melissa Chase, "FAME: Fast Attribute-based Message Encryption", (Section 3). In Proceedings of the 2017 ACM SIGSAC Conference on Computer and Communications Security 2017. Available from https://eprint.iacr.org/2017/807.pdf

## AW11 CP-ABE

Lewko, Allison, and Brent Waters, "Decentralizing Attribute-Based Encryption.", (Appendix D). In Eurocrypt 2011. Available from http://eprint.iacr.org/2010/351.pdf

## BSW CP-ABE

John Bethencourt, Amit Sahai, Brent Waters, "Ciphertext-Policy Attribute-Based Encryption" In IEEE Symposion on Security and Privacy, 2007. Available from https://doi.org/10.1109/SP.2007.11

# Implemented Key Policy Schemes (KP-ABE)

## AC17 KP-ABE

Shashank Agrawal, Melissa Chase, "FAME: Fast Attribute-based Message Encryption". In Proceedings of the 2017 ACM SIGSAC Conference on Computer and Communications Security 2017. Available from https://eprint.iacr.org/2017/807.pdf

## LSW KP-ABE 

Allison Lewko, Amit Sahai and Brent Waters, "Revocation Systems with Very Small Private Keys". In IEEE Symposium on Security and Privacy, 2010. SP'10. Available from http://eprint.iacr.org/2008/309.pdf

# Setup

In order to build the system write the follwing (standing in the root folder): <br /> 
$ python -m venv .env <br />
$ source .env/bin/activate <br />
$ maturin develop <br />
<br />
To install the the package write the following while standing in the root folder: <br />
$ pip install .

# Requirements

Python >= 3.7
Maturin >=0.14,<0.15