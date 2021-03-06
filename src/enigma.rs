pub mod enigma_m3{

    const LETTERCOUNT: usize = 26;
    const PLUGCOUNT: usize = 10;

    const UKWA: [i32; LETTERCOUNT] = [4, 9, 12, 25, 0, 11, 24, 23, 21, 22, 1, 5, 2, 17, 16, 20, 14, 13, 19, 18, 15, 8, 10, 7, 6, 3];
    const UKWB: [i32; LETTERCOUNT] = [24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 0, 19];
    const UKWC: [i32; LETTERCOUNT] = [5, 21, 15, 9, 8, 0, 14, 24, 4, 3, 17, 25, 23, 22, 6, 2, 19, 10, 20, 16, 18, 1, 13, 12, 7, 11];

    const ROTORI: [i32; LETTERCOUNT] = [4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9];
    const ROTORII: [i32; LETTERCOUNT] = [0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4];
    const ROTORIII: [i32; LETTERCOUNT] = [1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12 ,20, 18, 16, 14];
    const ROTORIV: [i32; LETTERCOUNT] = [4, 18, 14, 21, 15, 25, 9, 0, 24, 16, 20, 8, 17, 7, 23, 11, 13, 5, 19, 6, 10, 3, 2, 12, 22, 1];
    const ROTORV: [i32; LETTERCOUNT] = [21, 25, 1, 17, 6, 8, 19, 24, 20, 15, 18, 3, 13, 7, 11, 23, 0, 22, 12, 9, 16, 14, 5, 4, 2, 10];
    const ROTORVI: [i32; LETTERCOUNT] = [9, 15, 6, 21, 14, 20, 12, 5, 24, 16, 1, 4, 13, 7, 25, 17, 3, 10, 0, 18, 23, 11, 8, 2, 19, 22];
    const ROTORVII: [i32; LETTERCOUNT] = [13, 25, 9, 7, 6, 17, 2, 23, 12, 24, 18, 22, 1, 14, 20, 5, 0, 8, 21, 11, 15, 4, 10, 16, 3, 19];
    const ROTORVIII: [i32; LETTERCOUNT] = [5, 10, 16, 7, 19, 11, 23, 14, 2, 1, 9, 18, 15, 3, 25, 17, 0, 12, 4, 22, 13, 8, 20, 24, 6, 21];

    
    const CARRYI: [i32; 2] = [16, -1];
    const CARRYII: [i32; 2] = [4, -1];
    const CARRYIII: [i32; 2] = [21, -1];
    const CARRYIV: [i32; 2] = [9, -1];
    const CARRYV: [i32; 2] = [25, -1];
    const CARRYVI: [i32; 2] = [12, 25];
    const CARRYVII: [i32; 2] = [12, 25];
    const CARRYVIII: [i32; 2] = [12, 25];
    

    pub struct Enigma{
        rotors: (Rotor, Rotor, Rotor),
        plate: Plate,
        ukw: UKW,
    }

    impl Enigma{
        pub fn new(rotors: (i32, i32, i32), plate: [Option<(char, char)>; PLUGCOUNT], ukw: char) -> Self{
            let (r_hr_num, r_min_num, r_sec_num) = rotors;

            let r_hr = Rotor::new(r_hr_num);
            let r_min = Rotor::new(r_min_num);
            let r_sec = Rotor::new(r_sec_num);

            Enigma{
                rotors: (r_hr, r_min, r_sec),
                plate: Plate::new(plate),
                ukw: UKW::new(ukw),
            }
        }

        pub fn encrypt(&mut self, input: &str, state_hr: i32, state_min: i32, state_sec: i32) -> String{
            let (r_hr, r_min, r_sec) = (&mut self.rotors.0, &mut self.rotors.1, &mut self.rotors.2);
            let (r_hr_state_start, r_min_state_start, r_sec_state_start) = (state_hr, state_min, state_sec);

            r_hr.state = r_hr_state_start;
            r_min.state = r_min_state_start;
            r_sec.state = r_sec_state_start;
            
            let mut output = String::new();

            for c in input.chars(){
                let input_signal = c as i32 - 97;

                if input_signal as u8 > LETTERCOUNT as u8 || input_signal < 0{
                    panic!("character not allowed");
                }

                r_sec.turn_down();
                if r_sec.state == 0{
                    r_min.turn_down();
                    if r_min.state == 0{
                        r_hr.turn_down();
                    }
                }
                
                let sec_out_f = r_sec.get_output_forward(input_signal as usize, 0);
                let min_out_f = r_min.get_output_forward(sec_out_f as usize, r_sec.state);
                let hr_out_f = r_hr.get_output_forward(min_out_f as usize, r_min.state);

                let ukw_out = self.ukw.get_output(hr_out_f as usize, r_hr.state);

                let hr_out_b = r_hr.get_output_backward(ukw_out as usize, self.ukw.rotor.state);
                let min_out_b = r_min.get_output_backward(hr_out_b as usize, r_hr.state);
                let sec_out_b = r_sec.get_output_backward(min_out_b as usize, r_min.state);
                let final_output = ((sec_out_b - r_sec.state ) + LETTERCOUNT as i32) % LETTERCOUNT as i32;

                output.push((final_output as u8 + 97u8) as char);

            }
            output 
        }
    }

    struct Plate{
        character_mappings: [Option<(char, char)>;PLUGCOUNT]
    }

    impl Plate{
        fn new(mapping: [Option<(char, char)>; PLUGCOUNT]) -> Self{
            Plate{character_mappings: mapping}
        }

        fn get_character(input: char) -> char{
            'c'
        }
    }

    struct Rotor{
        state: i32,
        character_mapping: [i32; LETTERCOUNT],
        carries: Option<[i32; 2]>
    }

    impl Rotor{
        fn new(wheelnumber: i32) -> Self{
            let (character_mapping, carries) = match wheelnumber{
                1 => (ROTORI, Some(CARRYI)),
                2 => (ROTORII, Some(CARRYII)),
                3 => (ROTORIII, Some(CARRYIII)),
                4 => (ROTORIV, Some(CARRYIV)),
                5 => (ROTORV, Some(CARRYV)),
                6 => (ROTORVI, Some(CARRYVI)),
                7 => (ROTORVII, Some(CARRYVII)),
                8 => (ROTORVIII, Some(CARRYVIII)),
                _ => panic!("Incompatible Rotor")//[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            };

            Rotor{
                state: 0,
                character_mapping: character_mapping,
                carries: carries
            }
        }

        fn turn_up(&mut self){
            self.state = (self.state + LETTERCOUNT as i32 - 1) % LETTERCOUNT as i32;
        }

        fn turn_down(&mut self){
            self.state = (self.state + LETTERCOUNT as i32 + 1) % LETTERCOUNT as i32;
        }

        fn get_output_forward(&self, input_signal: usize, state_before: i32) -> i32{
            self.character_mapping[(((input_signal as i32 + self.state - state_before) + LETTERCOUNT as i32) % LETTERCOUNT as i32) as usize]
        }

        fn get_output_backward(&self, input_signal: usize, state_before: i32) -> i32{
            self.character_mapping.iter().position(|&x| x == ((input_signal as i32 + self.state - state_before) + LETTERCOUNT as i32) % LETTERCOUNT as i32).unwrap() as i32
        }
    }

    struct UKW{
        rotor: Rotor
    }

    impl UKW{
        fn new(variant: char) -> Self{
            UKW{
                rotor: Rotor{
                    state: 0,
                    character_mapping: match variant{
                        'A' => UKWA,
                        'B' => UKWB,
                        'C' => UKWC,
                        _ => panic!("Incompatible UKW")//[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    },
                    carries: None
                }
            }
        }

        fn get_output(&self, input_signal: usize, state_before: i32) -> i32{
            self.rotor.character_mapping[(((input_signal as i32 + self.rotor.state - state_before) + LETTERCOUNT as i32) % LETTERCOUNT as i32 ) as usize]
        }
    }
}