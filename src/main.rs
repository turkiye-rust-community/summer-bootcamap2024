fn main() {
    let mut degisken = "Hello Bootcamp2024";
    degisken = "Öğrencileri";
    println!("{}", degisken);

    //integer
    //int Integer ...
    //i -> integer, u -> unsigned integer
    //8bit -> 0 - 2**8-1
    //8bit -> -2**7 - 2**7-1
    //i , u
    //8, 16, 32, 64, 128
    //i8, u32
    //arch -> isize, usize
    // 32bit laptop, isize ->

    let int_deg = 12;
    let int_deg2: u16 = 12;
    println!("{}", int_deg);
    println!("{}", int_deg2);
    let arch_deg: usize = 13;
    let arch_deg2: isize = 13;
    //smart print

    //floating point
    //double, float
    //f32, f64

    let fp_deg = 12.2;
    let fp_deg2: f32 = 12.2;

    //boolean
    //true, false
    let b_deg = false;
    let b_deg2: bool = true;

    //character
    //char -> 1 byte, 2 bytes, 4 bytes
    let c_deg = '1';
    let rustacean = '🦀';
    println!("{}", rustacean);

    //String -> &str, String -> daha sonra bahsedilecek

    //Number literal
    let nl_deg = 98_222;
    println!("{}", nl_deg);
    let nl_deg2 = 0x888;
    println!("{}", nl_deg2);
    let nl_deg3: u32 = 0b101010101;
    println!("{}", nl_deg3);
    let nl_deg4 = 0o76571;
    println!("{:x}", nl_deg4);

    //Compound
    //Tuple (tip1, tip2, tip3)
    let t_deg = (12, 3.4, 'A');
    println!("{:?}", t_deg);

    let t_deg: (u16, f32, char) = (18988, 3.45, 'A');
    println!("{:?}", t_deg);
    println!("{}", t_deg.0);
    println!("{}", t_deg.2);

    //array -> boyutu sabit
    let arr_deg = [1; 10];
    let arr_deg2: [u16; 3] = [1, 2, 3];
    let arr_deg3: [u32; 10] = [12; 10];

    println!("deg2 : {}", arr_deg2[2]);
    //array_func_first(arr_deg);

    let ilk_eleman = array_func_first(arr_deg3);
    println!("ilk eleman: {}", ilk_eleman);
    println!("{:?}", arr_deg3);

    beni_cagir();
    beni_cagir2(4);

    let don_deg = beni_cagir_don();
    println!("{}", don_deg);
    println!("{}", beni_formatla(89));

    //Koşullar
    //Büyüktür -> > , Küçüktür -> <, Eşit = = -> ==

    if 18 > 17
    //true
    {
        println!("18 sayısı 17'den büyüktür");
    }

    if 1 == 1 {
        println!("İki sayı birbirine eşittir");
    }

    if 1 < 0 {
        println!("Ben hiç ekrana yazdırılmadım.");
    } else {
        println!("1 sayısı 0 sayısından küçük değildir. Büyük veya eşittir.");
    }

    let deg1 = if 5 > 6 {
        "5 Sayısı 6 Sayısından büyüktür"
    } else {
        "5 Sayısı 6 Sayısından küçük veya eşittir."
    };

    println!("deg1 : {}", deg1);

    let deg2 = if 5 > 6 {
        "5 Sayısı 6 Sayısından büyüktür"
    } else if 5 == 6 {
        "5 Sayısı 6 Sayısına eşittir."
    } else {
        "5 Sayısı 6 Sayısından küçüktür"
    };

    let deg3: i8 = if 5 > 6 { 1 } else { 0 };

    println!("deg2 : {}", deg2);

    let s_deg = 1;
    let sa_deg = 0;

    if s_deg < sa_deg {
        println!("1 sayısı 0 sayısından küçüktür");
    } else if s_deg == sa_deg {
        println!("1 sayısı 0 sayısına eşittir");
    } else {
        println!("1 sayısı 0 sayısından büyüktür.");
    }

    //Döngüler
    // 3 tip döngü
    // loop döngüsü -> loop -> içeride bir devrekesici, döngü kırıcı
    //while döngüsü -> while -> koşula göre devam etme durumu
    //for döngüsü -> for -> sınırları belli bir döngü

    /*for _ in 1..=10 {
    println!("Türkiye Rust Community");
    }*/

    let mut index = 0;

    /*while index < 10 {
    println!("Türkiye Rust Community");
    index = index + 1;
    }*/

    //index = 0;

    /*  loop {
    println!("Türkiye Rust Community");
    if index >= 9 {
        break;
    }
    index += 1;
    }*/
}

fn beni_cagir() {
    println!("Ben çağrıldım.");
}

fn beni_cagir2(param: u32) {
    println!("Gelen Parametre: {}", param);
}

fn beni_cagir_don() -> u32 {
    let donecek: u32 = 45;
    //
    donecek
}

fn beni_formatla(param: u32) -> String {
    format!("Dönen ve alınan değer: {}", param)
}

fn array_func_first(array: [u32; 10]) -> u32 {
    array[0]
}
