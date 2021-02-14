// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    constants::tss::{
        TPM2_ALG_AES, TPM2_ALG_CAMELLIA, TPM2_ALG_CBC, TPM2_ALG_CFB, TPM2_ALG_CMAC, TPM2_ALG_CTR,
        TPM2_ALG_ECB, TPM2_ALG_ECC, TPM2_ALG_ECDAA, TPM2_ALG_ECDH, TPM2_ALG_ECDSA, TPM2_ALG_ECMQV,
        TPM2_ALG_ECSCHNORR, TPM2_ALG_ERROR, TPM2_ALG_HMAC, TPM2_ALG_KDF1_SP800_108,
        TPM2_ALG_KDF1_SP800_56A, TPM2_ALG_KDF2, TPM2_ALG_KEYEDHASH, TPM2_ALG_MGF1, TPM2_ALG_NULL,
        TPM2_ALG_OAEP, TPM2_ALG_OFB, TPM2_ALG_RSA, TPM2_ALG_RSAES, TPM2_ALG_RSAPSS,
        TPM2_ALG_RSASSA, TPM2_ALG_SHA1, TPM2_ALG_SHA256, TPM2_ALG_SHA384, TPM2_ALG_SHA3_256,
        TPM2_ALG_SHA3_384, TPM2_ALG_SHA3_512, TPM2_ALG_SHA512, TPM2_ALG_SM2, TPM2_ALG_SM3_256,
        TPM2_ALG_SM4, TPM2_ALG_SYMCIPHER, TPM2_ALG_TDES, TPM2_ALG_XOR,
    },
    tss2_esys::TPM2_ALG_ID,
    Error, Result, WrapperErrorKind,
};
use log::error;
use std::convert::TryFrom;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Algorithm {
    Error,
    Rsa,
    Tdes,
    // Sha, same as sha1
    Sha1,
    Hmac,
    Aes,
    Mgf1,
    KeyedHash,
    Xor,
    Sha256,
    Sha384,
    Sha512,
    Null,
    Sm3_256,
    Sm4,
    RsaSsa,
    RsaEs,
    RsaPss,
    Oaep,
    EcDsa,
    EcDh,
    EcDaa,
    Sm2,
    EcSchnorr,
    EcMqv,
    Kdf1Sp800_56a,
    Kdf2,
    Kdf1Sp800_108,
    Ecc,
    SymCipher,
    Camellia,
    Cmac,
    Ctr,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Ofb,
    Cbc,
    Cfb,
    Ecb,
}

impl TryFrom<TPM2_ALG_ID> for Algorithm {
    type Error = Error;
    fn try_from(tpm_alg_id: TPM2_ALG_ID) -> Result<Algorithm> {
        match tpm_alg_id {
            TPM2_ALG_AES => Ok(Algorithm::Aes),
            TPM2_ALG_CAMELLIA => Ok(Algorithm::Camellia),
            TPM2_ALG_CBC => Ok(Algorithm::Cbc),
            TPM2_ALG_CFB => Ok(Algorithm::Cfb),
            TPM2_ALG_CMAC => Ok(Algorithm::Cmac),
            TPM2_ALG_CTR => Ok(Algorithm::Ctr),
            TPM2_ALG_ECB => Ok(Algorithm::Ecb),
            TPM2_ALG_ECC => Ok(Algorithm::Ecc),
            TPM2_ALG_ECDAA => Ok(Algorithm::EcDaa),
            TPM2_ALG_ECDH => Ok(Algorithm::EcDh),
            TPM2_ALG_ECDSA => Ok(Algorithm::EcDsa),
            TPM2_ALG_ECMQV => Ok(Algorithm::EcMqv),
            TPM2_ALG_ECSCHNORR => Ok(Algorithm::EcSchnorr),
            TPM2_ALG_ERROR => Ok(Algorithm::Error),
            TPM2_ALG_HMAC => Ok(Algorithm::Hmac),
            TPM2_ALG_KDF1_SP800_108 => Ok(Algorithm::Kdf1Sp800_108),
            TPM2_ALG_KDF1_SP800_56A => Ok(Algorithm::Kdf1Sp800_56a),
            TPM2_ALG_KDF2 => Ok(Algorithm::Kdf2),
            TPM2_ALG_KEYEDHASH => Ok(Algorithm::KeyedHash),
            TPM2_ALG_MGF1 => Ok(Algorithm::Mgf1),
            TPM2_ALG_NULL => Ok(Algorithm::Null),
            TPM2_ALG_OAEP => Ok(Algorithm::Oaep),
            TPM2_ALG_OFB => Ok(Algorithm::Ofb),
            TPM2_ALG_RSA => Ok(Algorithm::Rsa),
            TPM2_ALG_RSAES => Ok(Algorithm::RsaEs),
            TPM2_ALG_RSAPSS => Ok(Algorithm::RsaPss),
            TPM2_ALG_RSASSA => Ok(Algorithm::RsaSsa),
            TPM2_ALG_SHA1 => Ok(Algorithm::Sha1),
            TPM2_ALG_SHA256 => Ok(Algorithm::Sha256),
            TPM2_ALG_SHA384 => Ok(Algorithm::Sha384),
            TPM2_ALG_SHA3_256 => Ok(Algorithm::Sha3_256),
            TPM2_ALG_SHA3_384 => Ok(Algorithm::Sha3_384),
            TPM2_ALG_SHA3_512 => Ok(Algorithm::Sha3_512),
            TPM2_ALG_SHA512 => Ok(Algorithm::Sha512),
            TPM2_ALG_SM2 => Ok(Algorithm::Sm2),
            TPM2_ALG_SM3_256 => Ok(Algorithm::Sm3_256),
            TPM2_ALG_SM4 => Ok(Algorithm::Sm4),
            TPM2_ALG_SYMCIPHER => Ok(Algorithm::SymCipher),
            TPM2_ALG_TDES => Ok(Algorithm::Tdes),
            TPM2_ALG_XOR => Ok(Algorithm::Xor),
            _ => {
                error!("Encounted an unknown TPM2_ALG_ID");
                Err(Error::local_error(WrapperErrorKind::InvalidParam))
            }
        }
    }
}

impl From<Algorithm> for TPM2_ALG_ID {
    fn from(algorithm: Algorithm) -> TPM2_ALG_ID {
        match algorithm {
            Algorithm::Aes => TPM2_ALG_AES,
            Algorithm::Camellia => TPM2_ALG_CAMELLIA,
            Algorithm::Cbc => TPM2_ALG_CBC,
            Algorithm::Cfb => TPM2_ALG_CFB,
            Algorithm::Ctr => TPM2_ALG_CTR,
            Algorithm::Ecb => TPM2_ALG_ECB,
            Algorithm::Ecc => TPM2_ALG_ECC,
            Algorithm::EcDaa => TPM2_ALG_ECDAA,
            Algorithm::EcDh => TPM2_ALG_ECDH,
            Algorithm::EcDsa => TPM2_ALG_ECDSA,
            Algorithm::EcMqv => TPM2_ALG_ECMQV,
            Algorithm::EcSchnorr => TPM2_ALG_ECSCHNORR,
            Algorithm::Error => TPM2_ALG_ERROR,
            Algorithm::Hmac => TPM2_ALG_HMAC,
            Algorithm::Kdf1Sp800_108 => TPM2_ALG_KDF1_SP800_108,
            Algorithm::Kdf1Sp800_56a => TPM2_ALG_KDF1_SP800_56A,
            Algorithm::Kdf2 => TPM2_ALG_KDF2,
            Algorithm::KeyedHash => TPM2_ALG_KEYEDHASH,
            Algorithm::Mgf1 => TPM2_ALG_MGF1,
            Algorithm::Null => TPM2_ALG_NULL,
            Algorithm::Oaep => TPM2_ALG_OAEP,
            Algorithm::Ofb => TPM2_ALG_OFB,
            Algorithm::Rsa => TPM2_ALG_RSA,
            Algorithm::RsaEs => TPM2_ALG_RSAES,
            Algorithm::RsaPss => TPM2_ALG_RSAPSS,
            Algorithm::RsaSsa => TPM2_ALG_RSASSA,
            Algorithm::Sha1 => TPM2_ALG_SHA1,
            Algorithm::Sha256 => TPM2_ALG_SHA256,
            Algorithm::Sha384 => TPM2_ALG_SHA384,
            Algorithm::Sha3_256 => TPM2_ALG_SHA3_256,
            Algorithm::Sha3_384 => TPM2_ALG_SHA3_384,
            Algorithm::Sha3_512 => TPM2_ALG_SHA3_512,
            Algorithm::Sha512 => TPM2_ALG_SHA512,
            Algorithm::Sm2 => TPM2_ALG_SM2,
            Algorithm::Sm3_256 => TPM2_ALG_SM3_256,
            Algorithm::Sm4 => TPM2_ALG_SM4,
            Algorithm::SymCipher => TPM2_ALG_SYMCIPHER,
            Algorithm::Tdes => TPM2_ALG_TDES,
            Algorithm::Xor => TPM2_ALG_XOR,
            Algorithm::Cmac => TPM2_ALG_CMAC,
        }
    }
}
