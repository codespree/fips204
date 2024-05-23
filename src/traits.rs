use rand_core::CryptoRngCore;
#[cfg(feature = "default-rng")]
use rand_core::OsRng;


/// The `KeyGen` trait is defined to allow trait objects.
pub trait KeyGen {
    /// A public key specific to the chosen security parameter set, e.g., ml-dsa-44, ml-dsa-65 or ml-dsa-87
    type PublicKey;
    /// A private (secret) key specific to the chosen security parameter set, e.g., ml-dsa-44, ml-dsa-65 or ml-dsa-87
    type PrivateKey;
    /// An expanded private key containing precomputed elements to increase (repeated) signing performance.
    type ExpandedPrivateKey;
    /// An expanded public key containing precomputed elements to increase (repeated) verify performance.
    type ExpandedPublicKey;

    /// Generates a public and private key pair specific to this security parameter set. <br>
    /// This function utilizes the **OS default** random number generator. This function operates
    /// in constant-time relative to secret data (which specifically excludes the OS random
    /// number generator, the `rho` value stored in the public key, and the hash-derived
    /// `rho_prime` values which is rejection-sampled/expanded into `s_1` and `s_2`).
    /// # Errors
    /// Returns an error when the random number generator fails.
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use fips204::ml_dsa_44; // Could also be ml_dsa_65 or ml_dsa_87.
    /// use fips204::traits::{KeyGen, SerDes, Signer, Verifier};
    ///
    /// let message = [0u8, 1, 2, 3, 4, 5, 6, 7];
    ///
    /// // Generate key pair and signature
    /// let (pk, sk) = ml_dsa_44::KG::try_keygen()?; // Generate both public and secret keys
    /// let sig = sk.try_sign(&message)?; // Use the secret key to generate a message signature
    /// # Ok(())}
    /// ```
    #[cfg(feature = "default-rng")]
    fn try_keygen() -> Result<(Self::PublicKey, Self::PrivateKey), &'static str> {
        Self::try_keygen_with_rng(&mut OsRng)
    }

    /// Generates a public and private key pair specific to this security parameter set. <br>
    /// This function utilizes the **provided** random number generator. This function operates
    /// in constant-time relative to secret data (which specifically excludes the provided random
    /// number generator, the `rho` value stored in the public key, and the hash-derived
    /// `rho_prime` values which is rejection-sampled/expanded into `s_1` and `s_2`).
    /// # Errors
    /// Returns an error when the random number generator fails.
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use fips204::ml_dsa_44; // Could also be ml_dsa_65 or ml_dsa_87.
    /// use fips204::traits::{KeyGen, SerDes, Signer, Verifier};
    /// use rand_chacha::rand_core::SeedableRng;
    ///
    /// let message = [0u8, 1, 2, 3, 4, 5, 6, 7];
    /// let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123);
    ///
    /// // Generate key pair and signature
    /// let (pk, sk) = ml_dsa_44::KG::try_keygen_with_rng(&mut rng)?;  // Generate both public and secret keys
    /// let sig = sk.try_sign(&message)?;  // Use the secret key to generate a message signature
    /// # Ok(())}
    /// ```
    fn try_keygen_with_rng(
        rng: &mut impl CryptoRngCore,
    ) -> Result<(Self::PublicKey, Self::PrivateKey), &'static str>;

    /// Generates an expanded private key from the normal/compressed private key.
    ///
    /// # Errors
    /// Propagates internal errors; potential for additional validation as FIPS 204 evolves.
    fn gen_expanded_private(
        sk: &Self::PrivateKey,
    ) -> Result<Self::ExpandedPrivateKey, &'static str>;

    /// Generates an expanded public key from the normal/compressed public key.
    ///
    /// # Errors
    /// Propagates internal errors; potential for additional validation as FIPS 204 evolves.
    fn gen_expanded_public(pk: &Self::PublicKey) -> Result<Self::ExpandedPublicKey, &'static str>;
}


/// The Signer trait is implemented for the `PrivateKey` struct on each of the security parameter sets
pub trait Signer {
    /// The signature is specific to the chosen security parameter set, e.g., ml-dsa-44, ml-dsa-65 or ml-dsa-87
    type Signature;

    /// Attempt to sign the given message, returning a digital signature on success, or an error if
    /// something went wrong. This function utilizes the default OS RNG and operates in constant time
    /// with respect to the `PrivateKey` only (not including rejection loop; work in progress).
    ///
    /// # Errors
    /// Returns an error when the random number generator fails; propagates internal errors.
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use fips204::ml_dsa_65; // Could also be ml_dsa_44 or ml_dsa_87.
    /// use fips204::traits::{KeyGen, SerDes, Signer, Verifier};
    ///
    /// let message = [0u8, 1, 2, 3, 4, 5, 6, 7];
    ///
    /// // Generate key pair and signature
    /// let (pk, sk) = ml_dsa_65::KG::try_keygen()?; // Generate both public and secret keys
    /// let sig = sk.try_sign(&message)?; // Use the secret key to generate a message signature
    /// # Ok(())}
    /// ```
    #[cfg(feature = "default-rng")]
    fn try_sign(&self, message: &[u8]) -> Result<Self::Signature, &'static str> {
        self.try_sign_with_rng(&mut OsRng, message)
    }

    /// Attempt to sign the given message, returning a digital signature on success, or an error if
    /// something went wrong. This function utilizes a supplied RNG and operates in constant time
    /// with respect to the `PrivateKey` only (not including rejection loop; work in progress).
    ///
    /// # Errors
    /// Returns an error when the random number generator fails; propagates internal errors.
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use fips204::ml_dsa_65; // Could also be ml_dsa_44 or ml_dsa_87.
    /// use fips204::traits::{KeyGen, SerDes, Signer, Verifier};
    /// use rand_chacha::rand_core::SeedableRng;
    ///
    /// let message = [0u8, 1, 2, 3, 4, 5, 6, 7];
    /// let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123);
    ///
    /// // Generate key pair and signature
    /// let (pk, sk) = ml_dsa_65::KG::try_keygen_with_rng(&mut rng)?;  // Generate both public and secret keys
    /// let sig = sk.try_sign_with_rng(&mut rng, &message)?;  // Use the secret key to generate a message signature
    /// # Ok(())}
    /// ```
    fn try_sign_with_rng(
        &self, rng: &mut impl CryptoRngCore, message: &[u8],
    ) -> Result<Self::Signature, &'static str>;
}


/// The Verifier trait is implemented for `PublicKey` on each of the security parameter sets
pub trait Verifier {
    /// The signature is specific to the chosen security parameter set, e.g., ml-dsa-44, ml-dsa-65
    /// or ml-dsa-87
    type Signature;

    /// Verifies a digital signature with respect to a `PublicKey`. This function operates in
    /// variable time.
    ///
    /// # Errors
    /// Returns an error on a malformed signature; propagates internal errors.
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use fips204::ml_dsa_65; // Could also be ml_dsa_44 or ml_dsa_87.
    /// use fips204::traits::{KeyGen, SerDes, Signer, Verifier};
    ///
    /// let message = [0u8, 1, 2, 3, 4, 5, 6, 7];
    ///
    /// // Generate key pair and signature
    /// let (pk, sk) = ml_dsa_65::KG::try_keygen()?; // Generate both public and secret keys
    /// let sig = sk.try_sign(&message)?; // Use the secret key to generate a message signature
    /// let v = pk.try_verify(&message, &sig)?; // Use the public to verify message signature
    /// # Ok(())}
    /// ```
    fn try_verify(&self, message: &[u8], signature: &Self::Signature)
        -> Result<bool, &'static str>;
}

/// The `SerDes` trait provides for validated serialization and deserialization of fixed- and correctly-size elements.
/// Note that FIPS 204 currently states that outside of exact length checks "ML-DSA is not designed to require any
/// additional public-key validity checks". Nonetheless, a `Result()` is returned during all deserialization operations
/// to preserve the ability to add future checks (and for symmetry across structures).
pub trait SerDes {
    /// The fixed-size byte array to be serialized or deserialized
    type ByteArray;

    /// Produces a byte array of fixed-size specific to the struct being serialized.
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use fips204::ml_dsa_65; // Could also be ml_dsa_44 or ml_dsa_87.
    /// use fips204::traits::{KeyGen, SerDes, Signer, Verifier};
    ///
    /// let message = [0u8, 1, 2, 3, 4, 5, 6, 7];
    ///
    /// // Generate key pair and signature
    /// let (pk, sk) = ml_dsa_65::KG::try_keygen()?; // Generate both public and secret keys
    /// let sig = sk.try_sign(&message)?; // Use the secret key to generate a message signature
    /// let pk_bytes = pk.into_bytes(); // Serialize the public key
    /// let sk_bytes = sk.into_bytes(); // Serialize the private key
    /// # Ok(())}
    /// ```
    fn into_bytes(self) -> Self::ByteArray;

    /// Consumes a byte array of fixed-size specific to the struct being deserialized; performs validation
    /// # Errors
    /// Returns an error on malformed input.
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use fips204::ml_dsa_87; // Could also be ml_dsa_44 or ml_dsa_65.
    /// use fips204::traits::{KeyGen, SerDes, Signer, Verifier};
    ///
    /// // Generate key pair and signature
    /// let (pk, sk) = ml_dsa_87::try_keygen()?; // Generate both public and secret keys
    /// let pk_bytes = pk.into_bytes(); // Serialize the public key
    /// let sk_bytes = sk.into_bytes(); // Serialize the private key
    /// let pk2 = ml_dsa_87::PublicKey::try_from_bytes(pk_bytes)?;
    /// let sk2 = ml_dsa_87::PrivateKey::try_from_bytes(sk_bytes)?;
    /// # Ok(())}
    /// ```
    fn try_from_bytes(ba: Self::ByteArray) -> Result<Self, &'static str>
    where
        Self: Sized;
}
