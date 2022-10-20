use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use ark_ec::{
    pairing::{Pairing, PairingOutput},
    AffineRepr,
};

pub fn pairing() -> PairingOutput<Bls12_381> {
    let g1 = G1Affine::generator();
    let g2 = G2Affine::generator();
    
    Bls12_381::pairing(g1, g2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Fq12};
    
    use ark_ff::{BigInteger384, Field};
    use ark_serialize::CanonicalSerialize;
    use std::fs;
    use std::io::Write;

    #[test]
    fn it_works() {
        let path_bytes = "./fp12_bytes.txt";
        let path_base_field = "./fp12_base_field_elements.txt";

        let mut writer = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path_base_field)
            .unwrap();

        let result = pairing();
        let base_field_elements = Fq12::to_base_prime_field_elements(&result.0);

        // iterate through each element in the base field
        for base_field_element in base_field_elements {
            let bigint: BigInteger384 = base_field_element.0;

            writeln!(&mut writer, "{}", bigint).unwrap();
        }

        let d = Fq12::extension_degree() as usize;
        let mut serialized = vec![0u8; 48 * d];
        result
            .serialize_with_mode(&mut serialized[..], ark_serialize::Compress::Yes)
            .unwrap();
        println!("{:?}", serialized);
        fs::write(path_bytes, serialized).expect("Unable to write file");
    }
}
