use crate::*;

#[derive(Clone, Debug)]
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
    pub fn new(shape: &[usize], data: Vec<T>) -> Result<Self, ArrayNDError>{
        if !(shape.iter().fold(1, |acc, &x| acc * x) == data.len()){
            return Err(ArrayNDError::MissmatchedDimensions(format!( "Shape: {:?} does not match data, len = {:?}",
                shape,
                data.len()
            )))
        }
        Ok(Self{
            shape:shape.to_vec(),
            data:data
        })
    }

    pub fn fill(shape: &[usize], item: T) -> Self{
        let mut data = Vec::<T>::new();
        for _ in 0..shape.iter().fold(1, |acc, &x| acc * x){
            data.push(item.clone())
        }
        Self{shape:shape.to_vec(), data:data}
    }

    pub fn shape(&self) -> Vec<usize>{
        self.shape.clone()
    }

    pub fn true_index_from_position(&self, position:&[usize]) -> Result<usize, ArrayNDError> {
        if !(position.len() == self.shape.len()) {
            return Err(
                ArrayNDError::MissmatchedDimensions(
                    format!(
                        "Atempted to get index of position {:?} from ArrayND of shape {:?}", 
                        position, 
                        self.shape
                    )
                )
            );
        }
        let mut true_index:usize = 0;
        let mut last_layer: usize = 1;
        for i in (0..position.len()).rev(){
            if position[i] >= self.shape[i] {
                return Err(
                    ArrayNDError::IndexOutOfBounds(
                        format!(
                            "Atempted to get index of position {:?} from ArrayND of shape {:?}", 
                            position, 
                            self.shape
                        )
                    )
                );
                
            }
            true_index += position[i] * last_layer;
            last_layer *= self.shape[i];
    
        }
        Ok(true_index)
    }
    
    pub fn position_from_true_index(&self, index:usize) -> Result<Vec<usize>, ArrayNDError> {
        if index >= self.data.len() {
            return Err(ArrayNDError::IndexOutOfBounds(format!( "true index {:?} is out of bounds for ArrayND of shape {:?}",
                index,
                self.shape,
            )))
        }
        let mut position = vec![0; self.shape.len()];
        let mut current_index = index;
        for (i, &dim_size) in self.shape.iter().enumerate().rev() {
            position[i] = current_index % dim_size;
            current_index /= dim_size;
        }

        Ok(position)
    }

    pub fn get_item(&self, index: &[usize]) -> Result<T, ArrayNDError>{
        match self.true_index_from_position(index) {
            Ok(true_index) => { return Ok(self.data[true_index].clone());},
            Err(e) => {return Err(e);}
        }
        
    }

    pub fn slice(&self, slice:&[(usize, usize)]) -> Result<Self, ArrayNDError>{
        if !(slice.len() == self.shape.len()) {
            return Err(ArrayNDError::MissmatchedDimensions(format!("Illegal operation, cannot take slice {:?} from ArrayND of shape {:?}", slice, self.shape)));
        }
        for (i, size) in self.shape.iter().enumerate(){
            let (slice_start, slice_end) = slice[i];
            if (slice_start == slice_end) || (slice_start > slice_end) || (slice_end > *size) {
                return Err(ArrayNDError::IllegalSlice(format!("Illegal operation, cannot take slice {:?} out of ArrayND of shape {:?}", slice, self.shape)));
            }
        }
        let mut slice_shape = vec![0;slice.len()];

        for (i, (slice_start, slice_end)) in slice.iter().enumerate() {
            slice_shape[i] = slice_end - slice_start;
        }

        let mut slice_data = Vec::<T>::new();
        for i in 0..self.data.len() {
            if let Ok(position) = self.position_from_true_index(i) {
                let mut position_in_slice = true;
                for (j, index) in position.iter().enumerate() {
                    let (slice_start, slice_end) = slice[j];
                    if !((*index >= slice_start) && (*index < slice_end)) {
                        position_in_slice = false;
                    }
                }
                if position_in_slice{
                    slice_data.push(self.data[i].clone());
                }
            }
        }

        let mut out = Self {
            shape: slice_shape,
            data:slice_data,
        };
        //out.squeeze(); //TODO: think about if you want this
        Ok(out)

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

    pub fn added_dim_clone(&mut self, n: usize) -> Self {
        let mut shape = self.shape.clone();
        if n < self.shape.len(){
            shape.insert(n, 1);
        }
        else{
            for _ in 0..(n - self.shape.len() + 1){
                shape.push(1);
            }
        }
        Self{
            shape:shape,
            data:self.data.clone()
        }
    }

    pub fn transpose(&mut self, dims:&[usize]) {
        if dims.len() == self.shape.len() {
            //TODO
        }
        else {
            panic!("Illegal operation, attempting to transpose dim {:?} of shape {:?}", dims, self.shape);
        }

    }

    pub fn flatten_dim(&mut self, dim:usize){
        let mut shape = self.shape.clone();
        if dim < self.shape.len() - 1 {
            shape[dim + 1] *= shape[dim];
            shape.remove(dim);
        }
        else {
            panic!("Illegal operation, attempting to flatten dim {:?} of shape {:?}", dim, self.shape);
        }

    }

    pub fn flattened_dim_clone(&mut self, dim:usize) -> Self{
        let mut shape = self.shape.clone();
        if dim < self.shape.len() - 1 {
            shape[dim + 1] *= shape[dim];
            shape.remove(dim);
            return Self{
                shape:shape,
                data:self.data.clone()
            }
            
        }
        else {
            panic!("Illegal operation, attempting to flatten dim {:?} of shape {:?}", dim, self.shape);
        }

    }

    pub fn flatten(&mut self) {
        self.shape = vec![self.shape.iter().fold(1, |acc, &x| acc * x)];
    }

    pub fn flattened_clone(&self) -> Self {
        Self {
            shape: vec![self.shape.iter().fold(1, |acc, &x| acc * x)],
            data: self.data.clone(),
        }
    }

    pub fn squeeze(&mut self){
        self.shape.retain(|x| *x != 1_usize);
    }

    pub fn squeezed_clone(&self) -> Self {
        let mut new_shape = self.shape.clone();
        new_shape.retain(|x| *x != 1_usize);
        Self {
            shape: new_shape,
            data: self.data.clone()
        }
    }

}

mod tests {
    use super::*;

    #[test]
    fn test_fill() {

        let zeros = ArrayND::fill(&[3, 3, 3], 0.0_f64);

        assert_eq!(zeros.shape(), vec![3, 3, 3]);

        for value in zeros.get_flat_data(){
            assert_eq!(value, 0.0);
        }
    }

    #[test]
    fn test_slice() {
        let zeros = ArrayND::fill(&[3, 3, 3], 0.0_f64);
        let slice = zeros.slice(&[(0, 3),(0, 1), (0, 2)]).expect("legal Slice");
        assert_eq!(slice.shape(), vec![3, 1, 2]);
    }
}