extern crate rayon;
use rayon::prelude::*;

fn main() {
    let n_obs = 25;
    let chunk_size = 5;
    
    let segments: Vec<usize> = (0..n_obs).collect();

    segments
        .par_chunks(chunk_size)
	.enumerate()
        .for_each(|(chunk_idx, chunk)| {
 	    for (idx, &segment) in chunk.iter().enumerate() {
	        let pos = chunk_idx * chunk_size + idx;

		for line in 0..5 {
		    println!("chunk_index, index, pos, segment, line: {}, {}, {}, {}, {}", &chunk_idx, &idx, &pos, &segment, &line)
		}
            }
        });
}
