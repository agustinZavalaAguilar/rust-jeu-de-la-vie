use std::array;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;


fn main() {
  let mut  matrix = start_matrix();
  const MAX_GEN: i8 = 10;
  let mut generation = 1 ;
     while generation <= MAX_GEN {
         print!("\x1B[2J\x1B[1;1H");
       
        let matrix_next_gen = feed_next_gen(matrix);

         println!("Génération {} : ", generation);
         for row in 0..matrix_next_gen.len() {
             println!("{:?}", matrix_next_gen[row]);
         }
        if matrix == matrix_next_gen { 
            let mut matrix_null = true;
            for y in 0..matrix.len().try_into().unwrap() {
                for x in 0..matrix[0].len().try_into().unwrap() {
                    if matrix_next_gen[y][x] != 0 {
                        matrix_null = false;
                    }
                }
            }
            if matrix_null == true {
                println!("Toute les cellules sont mortes !");
                break;
            }else{
                println!("situation stable");
                break;
            } 
        }
        thread::sleep(Duration::from_millis(1000));
        //println!("'Next gen = '{:?}", matrix_next_gen);
         matrix = matrix_next_gen;

       

        generation = generation +1;
     }
    //println!("'Next gen = '{:?}", matrix);
   
}

// Créer un vector avec 3 tableaux
fn start_matrix() -> [[i8; 3]; 3]{
    print!("\x1B[2J\x1B[1;1H");
    let size_x: i8 = 3;
    let size_y: i8 = 3;

    let mut matrix: [[i8; 3]; 3] = [[0; 3]; 3];
    matrix[1][0] = 1;
    matrix[1][1] = 0;
    matrix[1][2] = 1;
     println!("Génération 0 (initial) : ");
    for row in 0..matrix.len() {
        println!("{:?}", matrix[row]);
    }
    thread::sleep(Duration::from_millis(1000));
    
    return matrix ; 
}

// Véfification des cellules autour
fn check_neighbors(matrix: [[i8; 3]; 3], x:i8 ,y:i8) ->i8{
    let mut limit_left :i8;
    let mut limit_superior:i8;
    let mut limit_right: i8;
    let mut limit_inferior: i8 ;

    if x == 0 {
        limit_left = 0;
    } else {
        limit_left = -1;
    }

    if y == 0 {
        limit_superior = 0;
    } else {
        limit_superior = -1;
    }

    if x+1 == matrix[0].len().try_into().unwrap() {
        limit_right = 0;
    } else {
        limit_right = 1;
    }

    if y+1 == matrix.len().try_into().unwrap() {
        limit_inferior = 0;
    } else {
        limit_inferior = 1;
    }

    let mut neighbor_count:i8 = 0;

    for i in limit_superior..=limit_inferior {
        for j in limit_left..=limit_right {
    

            if matrix[(y+i) as usize][(x+j) as usize] == 1 {
                if y == (y+i) && x == (x+j) { 
                    // ne fait rien car c'est la cellule de ref
                }else{
                    neighbor_count += 1;
                }
            }
        }
   
    }
    neighbor_count
}

// Application des regles
fn feed_next_gen(matrix: [[i8; 3]; 3]) -> [[i8; 3]; 3] {
    let mut matrix_next_gen = matrix.clone();
    for y in 0..matrix.len().try_into().unwrap() {
        for x in 0..matrix[0].len().try_into().unwrap() {
            let neighbors = check_neighbors(matrix, x, y);
            //println!("'neighbors outside condition:'{}", neighbors);
            if matrix[y as usize][x as usize] == 1 {
                //println!("'neighbors 1st condition :  '{}", neighbors);
                if neighbors < 2 || neighbors > 3 {   
                    //println!("'hello :'{}", "hello");
                    matrix_next_gen[y as usize][x as usize] = 0;                 
                } 
            } else {
                if neighbors == 3 {
                    //println!("'neighbors 2nd condition :  '{}", neighbors);
                    matrix_next_gen[y as usize][x as usize] = 1;
                }
            }
        }

    }

    //println!("'Next gen = '{:?}", matrix_next_gen);
    matrix_next_gen
}






