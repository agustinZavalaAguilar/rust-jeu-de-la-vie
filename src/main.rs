use std::array;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)] // Ajoutez cet attribut à votre énumération

enum CellState {
    Alive,
    Dead
    
}


fn main() {
    
  let mut  matrix = start_matrix();
  const MAX_GEN: i8 = 100;
  let mut generation = 1 ;
     while generation <= MAX_GEN {
         print!("\x1B[2J\x1B[1;1H");
       
        let matrix_next_gen = feed_next_gen(& matrix);

         println!("Génération {} : ", generation);
         for row in 0..matrix_next_gen.len() {
             println!("{:?}", matrix_next_gen[row]);
         }
        if matrix == matrix_next_gen { 
            let mut matrix_null = true;
            for y in 0..matrix.len().try_into().unwrap() {
                for x in 0..matrix[0].len().try_into().unwrap() {
                    if matrix_next_gen[y][x] != CellState::Dead {
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
fn start_matrix() -> Vec<Vec<CellState>>{


    print!("\x1B[2J\x1B[1;1H");
    // let size_x: i8 = 3;
    // let size_y: i8 = 3;

    let mut matrix: Vec<Vec<CellState>> =  vec![
        vec![CellState::Dead; 3],
        vec![CellState::Dead; 3],
        vec![CellState::Dead; 3],  
    ]; 
    matrix[0][1] = CellState::Alive;
    matrix[1][1] = CellState::Alive;
    matrix[2][1] = CellState::Alive;
    
     println!("Génération 0 (initial) : ");
    for row in 0..matrix.len() {
      println!("{:?}", matrix[row]);
    }

    thread::sleep(Duration::from_millis(1000));
    
    return matrix ; 
}


// Véfification des cellules autour
fn check_neighbors(matrix: & Vec<Vec<CellState>>, x:usize ,y:usize) ->i8{
    
    let mut neighbor_count:i8 = 0;

    if x != 0 { // Verify matrix limits to the left

        if  matrix[y][x-1] == CellState::Alive {
            neighbor_count +=1;
        }

        if y != 0  {
            if  matrix[y-1][x-1] == CellState::Alive {
                neighbor_count +=1;
            } 
        }

        if y+1 != matrix.len() {
            if  matrix[y+1][x-1] == CellState::Alive {
                neighbor_count +=1;
            } 

        }

    }
    
    if x+1 != matrix[0].len() { // Verify matrix limits to the right
        if matrix[y][x+1] == CellState::Alive {
            neighbor_count +=1;
        }

        if y != 0 {
            if matrix[y-1][x+1] == CellState::Alive {
                neighbor_count +=1;
            }
        }

        if y+1 != matrix.len() {
            if matrix[y+1][x+1] == CellState::Alive {
                neighbor_count +=1;
            }
        }
    }

    if y != 0 { // Verify matrix upper limits 
        if matrix[y-1][x] == CellState::Alive {
            neighbor_count +=1
        }
    }

    if y+1 != matrix.len() { // Verify matrix lower limits 
        if matrix[y+1][x] == CellState::Alive {
            neighbor_count +=1 
        }
    }

    neighbor_count
}

// Application des regles
fn feed_next_gen(matrix: &Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {
    let mut matrix_next_gen = matrix.clone();
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len(){
            let neighbors = check_neighbors(& matrix, x, y);
            //println!("'neighbors outside condition:'{}", neighbors);
            if matrix[y as usize][x as usize] == CellState::Alive {
                //println!("'neighbors 1st condition :  '{}", neighbors);
                if neighbors < 2 || neighbors > 3 {   
                    //println!("'hello :'{}", "hello");
                    matrix_next_gen[y][x as usize] = CellState::Dead;                 
                } 
            } else {
                if neighbors == 3 {
                    //println!("'neighbors 2nd condition :  '{}", neighbors);

                    matrix_next_gen[y as usize][x as usize] = CellState::Alive;
                }
            }
        }

    }

    //println!("'Next gen = '{:?}", matrix_next_gen);
    matrix_next_gen
}






