# AC17 Scheme mapped to python from rust.
# This file has examples of how to use this library.
from rabe_py import bdabe



def run_bdabe():
    """
    Implements the encryption and decryption methods in the BDABE Scheme
    Returns a plaintext
    """
    (pk, msk) = bdabe.setup()
    sak = bdabe.authgen(pk, msk, "aa1")
    uk = bdabe.keygen(pk, sak, "u1")
    attr_pk = bdabe.request_attribute_pk(pk, sak, "aa1::A")
    uk.append(bdabe.request_attribute_sk(uk, sak, "aa1::A"))
    policy = '"aa1::A" or "aa1::B"'
    plaintext = "test plaintext"
    ct = bdabe.encrypt(pk, attr_pk, policy, plaintext)
    plaintext_after = bdabe.decrypt(pk, uk, ct)
    print("".join(chr(i) for i in plaintext_after))

run_bdabe()

