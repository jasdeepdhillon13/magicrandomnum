fn main() {
    let mut random_num1:u8=0; 
    let mut bit1:u8 =0; 

    let mut random_num2:u8=0;
    let mut temp_num:u8=0;
    let mut temp_num1:u8=0;
    let mut temp_num2:u8=0;
    let mut temp_num3:u8=0;
    let mut temp_num4:u8=0; 
    let mut bit2:u8 =0; 

    
 

       
      for x in 0..8{
        bit1 = magic_num::magic_number(); 
        random_num1 |= bit1 << x; 
      }

      for x in 0..7{
        bit2 = magic_num::magic_number(); 
        temp_num |= bit2 << x; 
      }
  
      for x in 0..6{
          bit2 = magic_num::magic_number(); 
          temp_num1 |= bit2 << x; 
      }
  
      for x in 0..5{
          bit2 = magic_num::magic_number(); 
          temp_num2 |= bit2 << x; 
      }
  
      for x in 0..3{
          bit2 = magic_num::magic_number(); 
          temp_num3 |= bit2 << x; 
      }

      for x in 0..2{
        bit2 = magic_num::magic_number(); 
        temp_num4 |= bit2 << x; 
      }

        
  
      random_num2 = temp_num + temp_num1 + temp_num2 + temp_num3 + temp_num4;     

      println!("Random number 1:{} Random Number 2:{}", random_num1, random_num2); 

   
}
