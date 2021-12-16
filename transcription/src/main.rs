
#[derive(Debug)]
struct DNA {
    nucl: String,
}
#[derive(Debug)]
struct RNA {
    nucl: String,
}
use std::io;
//use std::char;
impl DNA {
    fn new()-> DNA {
    println!("Input DNA code");   
    let mut new_dna=String::new();
    io::stdin().read_line(&mut new_dna).expect("Failed to read");
    new_dna=new_dna.trim().to_string();
    for i in new_dna.chars() {
        if i != 'A'&& i != 'C'&& i !='G'&& i !='T' {
          panic!("It is not DNA");
        };
    };
        DNA {
        nucl: new_dna,
    }
    }
} 
impl RNA {
    fn new (dna: &DNA)-> RNA {
        let mut new_rna=String::new();
        for i in dna.nucl.chars() {
            if i == 'A' { new_rna.push_str("U");
            } else if i == 'T' { new_rna.push_str("A");
            } else if i == 'C' { new_rna.push_str("G");
            } else { new_rna.push_str("C");
            }
        }
        RNA {
            nucl: new_rna
        }
    }
}




fn main() {
   let dna=DNA::new();
   println!("{:?}", dna);
   let rna=RNA::new(&dna);
   println!("{:?}", rna);
}
