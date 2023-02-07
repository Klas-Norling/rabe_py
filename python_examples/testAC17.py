# AC17 Scheme mapped to python from rust.
# This file has examples of how to use this library.
from rabe_py import ac17


def kp_ac17():
    # Getting pk, msk
    (pk, msk) = ac17.setup()

    # print(pk)
    # pk = ac17.PyAc17PublicKey(pk.__str__())

    # Setting up attributes and policies as well as the plaintext
    plaintext = "secret"
    attributes = ["A", "B"]
    policy = '("A" and "B")'

    # Encrypt the text
    ciphertext = ac17.kp_encrypt(pk, attributes, plaintext)

    # Generating secret key
    secret_key = ac17.kp_keygen(msk, policy)

    # Decrypt the text
    plaintext_after = ac17.kp_decrypt(secret_key, ciphertext)
    print(plaintext_after)


def cp_ac17():
    # Getting pk, msk
    (pk, msk) = ac17.setup()

    # Setting up plaintext, attributes and policies
    plaintext = "secret from second"
    attribues = ["A", "B"]
    policy = '"A" and "B"'

    # Encrypting the text
    ciphertext = ac17.cp_encrypt(pk, policy, plaintext)

    # Generating secret key
    secret_key = ac17.cp_keygen(msk, attribues)

    # Decrypting and printing the text
    plaintext_after = ac17.cp_decrypt(secret_key, ciphertext)
    print(plaintext_after)


def other_stuff():
    (pk, msk) = ac17.setup()

    # This is an example of how to set a value as the publicKey, this is allowed atm with all keys and values
    pk = ac17.PyAc17PublicKey(pk.__str__())
    msk = ac17.PyAc17PublicKey(pk.__str__())

    # All of the classes are printable as well
    print(pk)
    print(msk)


kp_ac17()
cp_ac17()
# other_stuff()
