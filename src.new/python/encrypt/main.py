import argparse
import rsa
import multiprocessing

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-E", "--encrypt", type=str)
    parser.add_argument("-D", "--decrypt", type=str)
    args = parser.parse_args()
    if args.encrypt:
        pubkey, privkey = rsa.newkeys(1024, poolsize=(multiprocessing.cpu_count()*2+2))
        pub = pubkey.save_pkcs1()
        with open("public.pem", "wb+") as f:
            f.write(pub)
        priv = privkey.save_pkcs1()
        with open("private.pem", "wb+") as f:
            f.write(priv)
        enstr = rsa.encrypt(args.encrypt.encode("utf-8"), pubkey)
        print(enstr)
        print(privkey)
        del pubkey, privkey, pub, priv, enstr
    if args.decrypt:
        p = input("Please enter P:")
        with open("private.pem", "wb+") as f:
            f.write(rsa.PrivateKey(args.decrypt, 65537, args.private_exponent).save_pkcs1())
            some = rsa.PrivateKey.load_pkcs1(f.read())
        print(rsa.decrypt(bytes(args.decrypt.encode("utf-8")), some))
        del privkey, some
