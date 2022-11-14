use image::{self, GenericImageView};

fn convolve(a: [[f32; 3]; 3], b: [[f32; 3]; 3]) -> f32
{

    let mut r = 0.;

    for i in 0..3 {
        
        for j in 0..3 {
            
            r += a[i][j] * b[i][j];
        }
    }


    return r;
}

fn main() {

    let img = image::open("test.jpg").unwrap();


    let dimensions = img.dimensions();

    let mut imgbuff = img.into_rgb8();


    // axis kernels
    let g_x: [[f32; 3]; 3] = [
        [-1., 0., 1.],
        [-2., 0., 2.],
        [-1., 0., 1.]
    ];
    let g_y: [[f32; 3]; 3] = [
        [-1., -2., -1.],
        [0., 0., 0.],
        [1., 2., 1.]
    ];





    for h in (0..dimensions.1).step_by(3) {
        
        for w in (0..dimensions.0).step_by(3) {
            


            // image kernel
            let mut kernel: [[f32; 3]; 3] = [
                [0.,0.,0.],
                [0.,0.,0.],
                [0.,0.,0.]
            ];



            for i in 0..3 {
                
                for j in 0..3 {
    

                    if (w as i32 + i - 1) < 0 || (h as i32 + j - 1) < 0 || 
                    (w as i32 + i - 1) > dimensions.0 as i32 -1 || (h as i32 + j - 1) > dimensions.1 as i32 -1
                    {
                        continue;
                    }

                    let x = (w as i32 + i - 1) as u32;
                    let y = (h as i32 + j - 1) as u32;


                    // grayscale
                    let mut average = 0.;
                    for c in 0..3 {
                        average += imgbuff.get_pixel(x, y)[c] as f32;
                        
                    }
                    kernel[i as usize][j as usize] = average / 3.;
    
                }
            }


            // convolution
            let gx_tmp = convolve(g_x, kernel);
            let gy_tmp = convolve(g_y, kernel);


            // calcul de gradient
            let g = (gx_tmp.powf(2.) + gy_tmp.powf(2.)).sqrt();          


            for i in 0..3 as i32 {
            
                for j in 0..3 as i32 {
                   
                    if (w as i32 + i - 1) < 0 || (h as i32 + j - 1) < 0 || 
                    (w as i32 + i - 1) > dimensions.0 as i32 -1 || (h as i32 + j - 1) > dimensions.1 as i32 -1
                        {
                            continue;
                        }
                            
                    let x = (w as i32 + i - 1) as u32;
                    let y = (h as i32 + j - 1) as u32;

                    for c in 0..3 {


                    
                        // on prend le négatif
                        imgbuff[(x,y)][c] = g as u8;


                    }

                }
            }



        }

    }        


    
    imgbuff.save("output.jpg").unwrap();



    println!("Hello, world!");
}
