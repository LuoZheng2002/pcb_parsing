

pub struct Resolution{
    pub unit: Unit,
    pub value: f64,
}

pub enum Unit{
    UM,
    // to do: add more units
}

pub struct Layer{
    pub name: String,
}


pub struct Structure{
    pub layers: Vec<Layer>,
}

pub struct ComponentInst{
    pub reference: String,
    pub position: (f64, f64),
    pub rotation: f64,
}
pub struct Component{
    pub name: String,
    pub instances: Vec<ComponentInst>,
}

pub struct Placement{
    pub components: Vec<Component>,
}

pub struct Pin{
    pub pad_stack_name: String,
    pub position: (f64, f64),
    pub pin_number: u32,
}

pub struct Image{
    pub name: String,
    pub pins: Vec<Pin>,
}
pub struct Shape{
    
}
pub struct PadStack{
    pub name: String,
    // pub shapes: 
}
pub struct Library{
    // pub images: 
}

pub struct DsnStruct{

}