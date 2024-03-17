use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::{BLS12381Curve};
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;

fn generate_public_key(secret_key_hex: &str) -> Result<<BLS12381Curve as IsEllipticCurve>::PointRepresentation, &'static str> {
    // convert hex to u64
    let secret_key_int = u64::from_str_radix(secret_key_hex, 16).map_err(|_| "Invalid secret key format")?;

    // Get the curve generator point
    let generator = BLS12381Curve::generator();

    // Compute the public key by multiplying the generator point by the secret key
    let public_key = generator.operate_with_self(secret_key_int); 

    Ok(public_key)
}

fn main() {
    let secret_key_hex = "6C616D6264617370"; // Example secret key in hexadecimal
    match generate_public_key(secret_key_hex) {
        Ok(public_key) => println!("Public Key: {:?}", public_key),
        Err(e) => println!("Error: {}", e),
    }
}
