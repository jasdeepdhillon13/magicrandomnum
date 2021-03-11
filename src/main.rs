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

    
   for y in 0..5000000{

     random_num2=0;
    temp_num=0; 
    temp_num1=0;
    temp_num2=0;
    temp_num3=0;
    temp_num4=0;

    bit1 =0; 
    random_num1 =0; 

       
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
      
      if temp_num >= 127 && temp_num1 >= 63 && temp_num2 >= 31 && temp_num3 >= 7 && temp_num4 >= 3 {
        println!("edge case 1:{}", random_num2);
      }
      

    }

      println!("Random number 1:{} Random Number 2:{}", random_num1, random_num2); 

   
}
