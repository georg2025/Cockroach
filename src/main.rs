extern crate image;

use image::{ImageBuffer, Luma};
use std::path::Path;

pub struct Cockroach {
        latitude: u16,
        longitude: u16,
        lookatdegree: u16,
    }

pub struct BitMatrix {
    data: Vec<Vec<u8>>,
}

trait GetandReverse {
    fn get_pixel (& mut self, latitude: u16, longitude: u16) -> bool;
    fn get_and_reverse(& mut self, latitude: u16, longitude: u16) -> bool;
    fn set_visited (& mut self, latitude: u16, longitude: u16);
}

impl GetandReverse for BitMatrix {
    fn get_pixel (& mut self, latitude: u16, longitude: u16) -> bool {
        let vec_position = longitude as usize;
        let bite_position  = (latitude / 8) as usize;
        let bit_position = latitude%8;
        let bit = self.data[vec_position][bite_position] >> bit_position & 1;
        if bit == 0 {
            return false
        }
        return true
    }
    fn get_and_reverse (& mut self, latitude: u16, longitude: u16) -> bool {
        let vec_position = longitude as usize;
        let bite_position  = (latitude / 8) as usize;
        let bit_position = latitude%8;
        let bit = self.data[vec_position][bite_position] >> bit_position & 1;
        let helpnumber: u8 = 1 << bit_position;
        self.data[vec_position][bite_position] ^= helpnumber;
        if bit == 0 {
            return false
        }
        return true
    }
    fn set_visited(& mut self, latitude: u16, longitude: u16) {
        let vec_position = longitude as usize;
        let bite_position  = (latitude / 8) as usize;
        let bit_position = latitude%8;
        let helpnumber: u8 = 1 << bit_position;
        self.data[vec_position][bite_position] |= helpnumber;
    }
}

fn main() {

    let mut total_black_pixels = 0;

    let mut our_field = BitMatrix {
        data: vec![vec![0; 128]; 1024]
    };
    let mut visited_field = BitMatrix {
        data: vec![vec![0; 128]; 1024]
    };

    let mut cockroach1 = Cockroach {
        latitude: 512,
        longitude: 512,
        lookatdegree: 0,
    };

    loop {
        match cockroach1.lookatdegree {
            0 => cockroach1.longitude+=1,
            90 => cockroach1.latitude-=1,
            180 => cockroach1.longitude-=1,
            _ => cockroach1.latitude+=1,
        }
        if cockroach1.longitude == 0 || cockroach1.latitude == 0 || cockroach1.longitude == 1025 || cockroach1.latitude == 1025 {
            break
        }

        let current_square = our_field.get_and_reverse(cockroach1.latitude-1, cockroach1.longitude-1);

        visited_field.set_visited(cockroach1.latitude-1, cockroach1.longitude-1);

        if current_square {
            if cockroach1.lookatdegree > 0 {
                cockroach1.lookatdegree -= 90
            } else {
                cockroach1.lookatdegree = 270
            }
            total_black_pixels-=1;
        } else {
            if cockroach1.lookatdegree < 270 {
                cockroach1.lookatdegree += 90
            } else {
                cockroach1.lookatdegree = 0
            }
            total_black_pixels+=1;
        }

    }


    println!("total black pixels = {}", total_black_pixels);




    let mut imagebuf1 = ImageBuffer::new(1024 as u32, 1024 as u32);
    let mut imagebuf2 = ImageBuffer::new(1024 as u32, 1024 as u32);

    for i in 0..1024 {
        for j in 0..1024 {
            let img_value1 = if visited_field.get_pixel(j as u16, i as u16) == true { 0 as u8 } else { 255 as u8 };
            imagebuf1.put_pixel(1023-j as u32, 1023-i as u32, Luma([img_value1]));
            let img_value2 = if our_field.get_pixel(j as u16, i as u16) == true { 0 as u8 } else { 255 as u8 };
            imagebuf2.put_pixel(1023-j as u32, 1023-i as u32, Luma([img_value2]));
        }
    }

    imagebuf1.save(Path::new("every_visited_pixel.png")).unwrap();
    imagebuf2.save(Path::new("pixels_at_the_end.png")).unwrap();



}
