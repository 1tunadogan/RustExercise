use std::f32::consts::PI;

fn main(){
    // age adında bir i32 değişkeni tanımlayın ve değerini 25 olarak atayın. Bu değişkeni ekrana yazdırın.
    let age : i32 = 25;
    println!("Değişkenin değeri {}", age);

    // name adında bir String değişkeni tanımlayın ve isminizi bu değişkene atayın. Bu değişkeni ekrana yazdırın.
    let name = "'Tunahan'";
    println!("Benim adım {}", name);
    
    // temperature adında bir f32 değişkeni tanımlayın ve değerini 23.5 olarak atayın. Bu değişkeni ekrana yazdırın.
    let temperature : f32 = 23.5;
    println!("Sıcaklık {}", temperature);

    // is_student adında bir bool değişkeni tanımlayın ve değerini true olarak atayın. Bu değişkeni ekrana yazdırın.
    let is_student = true;
    println!("Bu kişi {}", is_student);

    /* x adında bir i32 ve y adında bir i32 değişkeni tanımlayın ve değerlerini sırasıyla 10 ve 5 olarak atayın. 
    Bu değişkenleri kullanarak x ve y değerlerinin toplamını ekrana yazdırın. */ 
    let x : i32 ;
    let y : i32;
    x = 10;
    y = 5;
    println!("Bu iki değerin toplamı {}", x+y);  

    /* radius adında bir f32 değişkeni tanımlayın ve değerini 5.0 olarak atayın. 
    Bu değişkeni kullanarak çemberin çevresini hesaplayın ve sonucu ekrana yazdırın. (Çemberin çevresi: 2 * π * r) */

    let radius: f32 = 5.0;
    let pi: f32  = 3.14;
    let cevre: f32 = 2.0 * pi * radius;
    println!("Çevre = {}", cevre);

    /* is_even adında bir bool değişkeni tanımlayın ve x adında bir i32 değişkeni tanımlayın. 
    x'in değerini kullanıcıdan alın ve is_even değişkenine x'in çift olup olmadığına göre true veya false atayın. 
    is_even değişkenini ekrana yazdırın. */
    use std::io;

    let mut x = String::new();

    println!("Lütfen bir sayı girin:");

    io::stdin()
        .read_line(&mut x)
        .expect("Okuma işlemi başarısız oldu.");

    let x: i32 = x.trim().parse().expect("Geçersiz sayı");

    let is_even: bool = x % 2 == 0;

    println!("Girilen sayı çift mi? {}", is_even);

    /* num1 ve num2 adında iki i32 değişkeni tanımlayın ve değerlerini kullanıcıdan alın. 
    Bu iki sayının toplamını, farkını, çarpımını ve bölümünü ekrana yazdırın. */
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Lütfen bir sayı girin:");

    io::stdin()
        .read_line(&mut num1)
        .expect("Okuma işlemi başarısız oldu.");

    println!("Lütfen bir sayı girin:");

    io::stdin()
            .read_line(&mut num2)
            .expect("Okuma işlemi başarısız oldu.");
    
    let num1: i32 = num1.trim().parse().expect("Geçersiz sayı");
    let num2: i32 = num2.trim().parse().expect("Geçersiz sayı");

    let toplam: i32 = num1 + num2;
    let fark: i32 = num1 - num2;
    let ex: i32 = num1 * num2;
    let bolum: i32= num1 / num2;
    println!("Toplam = {}, Fark = {}, Çarpım = {}, Bölüm = {}", toplam, fark, ex, bolum);

}