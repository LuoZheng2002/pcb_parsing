

pub struct Resolution{
    pub unit: Unit,
    pub value: f64,
}

pub enum Unit{
    UM,
    // to do: add more units
}

pub struct Layer{
    
}

pub struct Structure{
    pub layers: Vec<Layer>,
}

pub struct DsnStruct{

}