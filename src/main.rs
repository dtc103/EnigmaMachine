mod enigma;
use enigma::enigma_m3;


fn main(){
    let mut enigma = enigma_m3::Enigma::new((3, 2, 1), [None, None, None, None, None, None, None, None, None, None], 'B');
    let cipher = enigma.encrypt(String::from("fn"), 0, 0, 0);

    println!("{}", cipher);

}
