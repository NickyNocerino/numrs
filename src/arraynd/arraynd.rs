use crate::*;

pub struct ArrayND<T>{
    shape: Vec<usize>,
    data: Vec<T>,
}

impl<T:Clone> ArrayND<T>{

    pub fn empty() -> Self{
        Self{
            shape: vec![0],
            data: vec![]
        }
    }
    pub fn new(shape: Vec<usize>, data: Vec<T>) -> Self{
        Self{
            shape:shape,
            data:data
        }
    }

    pub fn fill(shape: Vec<usize>, item: T) -> Self{
        let mut data = Vec::<T>::new();
        for _ in 0..shape.iter().fold(1, |acc, &x| acc * x){
            data.push(item.clone())
        }
        Self{shape:shape, data:data}
    }

    pub fn shape(&self) -> Vec<usize>{
        self.shape.clone()
    }

    pub fn get_item(&self, index: Vec<usize>) -> Result<T, ArrayNDError>{
        if !index.len() == self.shape.len() {
            return Err(
                ArrayNDError::MissmatchedDimensions(
                    format!(
                        "Atempted to get item {:?} from ArrayND of shape {:?}", 
                        index, 
                        self.shape
                    )
                )
            );
        }
        let mut true_index:usize = 0;
        let mut last_layer: usize = 1;
        for i in (0..index.len()).rev(){
            if index[i] >= self.shape[i] {
                return Err(
                    ArrayNDError::IndexOutOfBounds(
                        format!(
                            "Atempted to get item {:?} from ArrayND of shape {:?}", 
                            index, 
                            self.shape
                        )
                    )
                );
                
            }
            true_index += index[i] * last_layer;
            last_layer *= self.shape[i];
    
        }
        Ok(self.data[true_index].clone())
    }

    pub fn get_flat_data(&self) -> Vec<T> {
        self.data.clone()
    }

    pub fn add_dim(&mut self, n: usize){
        if n < self.shape.len(){
            self.shape.insert(n, 1);
        }
        else{
            for _ in 0..(n - self.shape.len() + 1){
                self.shape.push(1);
            }
        }
    }

    pub fn flatten(&mut self) {
        self.shape = vec![self.shape.iter().fold(1, |acc, &x| acc * x)];
    }

}

