extern crate math;
extern crate renderer;

use renderer::texture::*;
use renderer::texture::filter::*;

use math::algorithm::noise::md::*;
use math::algorithm::noise::ds::*;

use math::statistics::average::*;

use std::mem;

/*
this is only for this case => return Pointer from HeapBox = http://jakegoulding.com/rust-ffi-omnibus/objects/

#[no_mangle]
pub extern fn noise_texture_ds_free(ptr: *mut u8) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}*/

#[no_mangle]
pub extern fn noise_texture_ds_middlefunc() -> fn(x1: f32, x2: f32, x3: f32, x4: f32) -> f32 {
	middle_quad
}

#[no_mangle]
pub extern fn noise_texture_ds(width: i32, height: i32, maxred: f32, red: f32, startseed: f32,
							   middlefunc: fn(x1: f32, x2: f32, x3: f32, x4: f32) -> f32)
 -> *mut u8 {	
	let w: i32 = width;
	let h: i32 = height;
	let maxreduction: f32 = maxred; //1.
	let reduction: f32 = red;//0.5;
	let mut ds: DiamondSquare = DiamondSquare::new(w as u32, h as u32);
	ds.length = w-1;
	ds.startseed = startseed;//1.;
	ds.seed = maxreduction;
	ds.reduction = reduction;
	ds.middlefunc = middlefunc;
	ds.create();
	
	let mut texture: Texture =  Texture::new(w as u32, h as u32);
	texture.from_noise(&ds.noise);
    let mut u8_vec = Vec::with_capacity((w*h*3) as usize);

    for color in &texture.buffer {
            u8_vec.push(color.r as u8);
            u8_vec.push(color.g as u8);
            u8_vec.push(color.b as u8);
	}

    
    let u_ptr = u8_vec.as_ptr();

    mem::forget(u_ptr);

    u_ptr as *mut u8

	/*
	if cfg!(feature = "filter_arith") {
		let texfilter = ArithmeticBoxFilter::new(2, 1.0);
		let mut tex_arith = texture.clone();
		tex_arith.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_arith.write_as_ppm("target/sd_noise_arith_filter.ppm");
		}
	}
	
	if cfg!(feature = "filter_gauss") {
		let texfilter = GaussFilter::new(2, 1.0);

		let mut tex_gauss = texture.clone();
		tex_gauss.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_gauss.write_as_ppm("target/sd_noise_gauss_filter.ppm");
		}
	}
	*/
}
