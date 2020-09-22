pub mod enigma_m3{
    const LETTERCOUNT: usize = 26;
    const PLUGCOUNT: usize = 10;

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

        pub fn encrypt(&mut self, input: String, state_hr: i32, state_min: i32, state_sec: i32) -> String{
            let (r_hr, r_min, r_sec) = (&mut self.rotors.0, &mut self.rotors.1, &mut self.rotors.2);
            let (r_hr_state_start, r_min_state_start, r_sec_state_start) = (state_hr, state_min, state_sec);

            r_hr.state = r_hr_state_start;
            r_min.state = r_min_state_start;
            r_sec.state = r_sec_state_start;
            
            let mut output = String::new();

            for c in input.chars(){
                let input_signal = c as u8 - 97;

                println!("{}", input_signal);
                
                if input_signal > LETTERCOUNT as u8{
                    panic!("character not allowed");
                }

                r_sec.turn_down();
                println!("state of secs: {}", r_sec.state);
                if r_sec.state == 0{
                    r_min.turn_down();
                    if r_min.state == 0{
                        r_hr.turn_down();
                    }
                }
                
                

                let sec_out_f = r_sec.get_output_forward(input_signal as usize, 0);
                println!("{}", sec_out_f);
                let min_out_f = r_min.get_output_forward(sec_out_f as usize, r_sec.state);
                println!("{}", min_out_f);
                let hr_out_f = r_hr.get_output_forward(min_out_f as usize, r_min.state);
                println!("{}", hr_out_f);

                let ukw_out = self.ukw.get_output(hr_out_f as usize, r_hr.state);
                println!("{}", ukw_out);

                let hr_out_b = r_hr.get_output_backward(ukw_out as usize, self.ukw.rotor.state);
                println!("{}", hr_out_b);
                let min_out_b = r_min.get_output_backward(hr_out_b as usize, r_hr.state);
                println!("{}", min_out_b);
                let sec_out_b = r_sec.get_output_backward(min_out_b as usize, r_min.state);
                println!("{}", sec_out_b);

                output.push((sec_out_b as u8 + 96u8) as char);

                println!();
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
        character_mapping: [i32; LETTERCOUNT]
    }

    impl Rotor{
        fn new(wheelnumber: i32) -> Self{
            Rotor{
                state: 0,
                character_mapping: match wheelnumber{
                    1 => [4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9],
                    2 => [0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4],
                    3 => [1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12 ,20, 18, 16, 14],
                    4 => [4, 18, 14, 21, 15, 25, 9, 0, 24, 16, 20, 8, 17, 7, 23, 11, 13, 5, 19, 6, 10, 3, 2, 12, 22, 1],
                    5 => [21, 25, 1, 17, 6, 8, 19, 24, 20, 15, 18, 3, 13, 7, 11, 23, 0, 22, 12, 9, 16, 14, 5, 4, 2, 10],
                    6 => [9, 15, 6, 21, 14, 20, 12, 5, 24, 16, 1, 4, 13, 7, 25, 17, 3, 10, 0, 18, 23, 11, 8, 2, 19, 22],
                    7 => [13, 25, 9, 7, 6, 17, 2, 23, 12, 24, 18, 22, 1, 14, 20, 5, 0, 8, 21, 11, 15, 4, 10, 16, 3, 19],
                    8 => [5, 10, 16, 7, 19, 11, 23, 14, 2, 1, 9, 18, 15, 3, 25, 17, 0, 12, 4, 22, 13, 8, 20, 24, 6, 21],
                    _ => panic!("Incompatible Rotor")//[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                }
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
            (self.character_mapping.iter().position(|&x| x == input_signal as i32 + self.state - state_before).unwrap() as i32 ) % LETTERCOUNT as i32// as i32 - state_before + self.state + LETTERCOUNT as i32  % LETTERCOUNT as i32
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
                        'B' => [24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 0, 19],
                        'C' => [5, 21, 15, 9, 8, 0, 14, 24, 4, 3, 17, 25, 23, 22, 6, 2, 19, 10, 20, 16, 18, 1, 13, 12, 7, 11],
                        _ => panic!("Incompatible UKW")//[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    }
                }
            }
        }

        fn get_output(&self, input_signal: usize, state_before: i32) -> i32{
            self.rotor.character_mapping[(((input_signal as i32 + self.rotor.state - state_before) + LETTERCOUNT as i32) % LETTERCOUNT as i32 ) as usize]
        }
    }
}