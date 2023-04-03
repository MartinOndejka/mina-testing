import { Field, Poseidon, PrivateKey, Scalar, Signature, isReady, shutdown } from "snarkyjs";

// private key is the same as in rust file
const base58Key = "EKEWXMfTkhRuA5nXjPvJGAJtDETABcpnyzH4yaBosepbxoPw4rne";

const main = async () => {
  const privateKey = PrivateKey.fromBase58(base58Key);
  const publicKey = privateKey.toPublicKey();

  const msg = Field(42);

  console.log(Poseidon.hash([msg]).toBigInt());

  console.log(Signature.create(privateKey, [msg]).toJSON());

  const rustSig = new Signature(
    Field("2355699371162162418732455224022155827035515518102597379507638182244539807417"),
    Scalar.fromBigInt(BigInt("7173154303036995994640947449569687151947065351124071738617980679368330425992"))
  );

  console.log(rustSig.verify(publicKey, [Field(42)]).toBoolean());
};

isReady.then(main).then(() => shutdown());
