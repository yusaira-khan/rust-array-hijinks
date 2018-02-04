struct Matrix_2x2f {
    data: [f32;4],
    len: usize,
    nrows:usize,
    ncols:usize
}

impl Matrix_2x2f{
     fn zeros() -> Matrix_2x2f{
        Matrix_2x2f{
            len:4,
            data :[0.0;4],
            nrows:2,
            ncols:2
        }
    }
    fn with_data( d : [f32;4])-> Matrix_2x2f{
         Matrix_2x2f{
            len:4,
            data :d,
            nrows:2,
            ncols:2
        }
    }
    fn identity( )-> Matrix_2x2f{
         Matrix_2x2f{
            len:4,
            data :[1.0,0.0,0.0,1.0],
            nrows:2,
            ncols:2
        }
    }
    fn add(&self, other:Matrix_2x2f)-> Matrix_2x2f{
       let mut ret = [0.0;4];
       for r in 0..self.nrows{
            for c in 0..self.ncols{
            let i = r*self.ncols+c;
            
            ret[i]=self.data[i]+other.data[i];
            }
        }
        Matrix_2x2f::with_data(ret)
    }
    
}

impl std::fmt::Display for Matrix_2x2f {
    // This trait requires `fmt` with this exact signature.
     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for r in 0..self.nrows{
            try!(write!(f,"\n| "));
            // println!("| ");
            for c in 0..self.ncols{
            let val=self.data[r*self.ncols+c];
                try!(write!(f,"{} | ",val));
            }
        }
        write!(f,"")
    }
    
}
fn get_i32() -> i32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i32>().unwrap()
}

fn main() {
    let m1 = Matrix_2x2f::identity();
    let m2 = Matrix_2x2f::with_data([1.0,2.0,3.0,5.0]);
    println!("{}",m1);
        println!("{}",m2);
        println!("{}",m1.add(m2))

}
