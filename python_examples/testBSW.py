from rabe_py import bsw


def run_bsw():
    pk, msk = bsw.setup()
    plaintext = "secret message"
    policy = '"A" and "B"'
    attributes = ["A", "B"]
    ciphertext = bsw.encrypt(pk, policy, plaintext)
    ciphertext = bsw.PyBswCiphertext(ciphertext.__str__())
    print(ciphertext)
    sk = bsw.keygen(pk, msk, attributes)
    print(bsw.decrypt(sk, ciphertext))

run_bsw()