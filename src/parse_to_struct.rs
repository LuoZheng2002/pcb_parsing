use crate::{dsn_struct::{Boundary, Component, DsnStruct, Layer, Placement, Resolution, Structure}, s_expr::SExpr};


fn parse_layer(s_expr: &Vec<SExpr>) -> Result<Layer, String> {
    let first_item = s_expr.first().ok_or("Expected at least one item in the layer scope")?;
    let first_item = first_item.as_atom().ok_or("Expected an atom as the first item in the layer scope")?;
    if first_item != "layer" {
        return Err(format!("Expected 'layer' as the first item, found: {}", first_item));
    }
    let second_item = s_expr.get(1).ok_or("Expected a second item in the layer scope")?;
    let second_item = second_item.as_atom().ok_or("Expected an atom as the second item in the layer scope")?;
    Ok(Layer{
        name: second_item.to_string(),
    })
}

fn parse_boundary(s_expr: &Vec<SExpr>) -> Result<Boundary, String> {
    let first_item = s_expr.first().ok_or("Expected at least one item in the boundary scope")?;
    let first_item = first_item.as_atom().ok_or("Expected an atom as the first item in the boundary scope")?;
    if first_item != "boundary" {
        return Err(format!("Expected 'boundary' as the first item, found: {}", first_item));
    }
    let second_item = s_expr.get(1).ok_or("Expected a second item in the boundary scope")?;
    let second_list = second_item.as_list().ok_or("Expected a list as the second item in the boundary scope")?;
    if second_list.len() < 3{
        return Err("Expected at least three items in the boundary list".to_string());
    }
    let mut points: Vec<(f64, f64)> = Vec::new();
    let mut prev_number: Option<f64> = None;
    for item in second_list.iter().skip(3){
        match prev_number{
            Some(prev_num)=>{
                let number = item.as_atom().ok_or("Expected an atom in the boundary list")?;
                let number = number.parse::<f64>().map_err(|e| format!("Failed to parse boundary number: {}", e))?;
                points.push((prev_num, number));
                prev_number = None;
            },
            None =>{
                let number = item.as_atom().ok_or("Expected an atom in the boundary list")?;
                let number = number.parse::<f64>().map_err(|e| format!("Failed to parse boundary number: {}", e))?;
                prev_number = Some(number);
            }
        }
    }
    if prev_number.is_some() {
        return Err("Expected an even number of items in the boundary list".to_string());
    }
    let boundary = Boundary(points);
    Ok(boundary)
}

fn parse_structure(s_expr: &Vec<SExpr>) -> Result<Structure, String> {
    // Placeholder for structure parsing logic
    // This function should parse the structure part of the S-expression
    // and populate the DsnStruct accordingly.
    let first_item = s_expr.first().ok_or("Expected at least one item in the structure scope")?;
    let first_item = first_item.as_atom().ok_or("Expected an atom as the first item in the structure scope")?;
    if first_item != "structure" {
        return Err(format!("Expected 'structure' as the first item, found: {}", first_item));
    }
    let mut layers: Vec<Layer> = Vec::new();
    let mut boundary: Option<Boundary> = None;
    for item in s_expr.iter().skip(1) {
        let expr_list = item.as_list().ok_or(format!("Expected a list in the structure scope, found: {:?}", item))?;
        let first_item = expr_list.first().ok_or("Expected at least one item in the structure item")?;
        let first_item = first_item.as_atom().ok_or("Expected an atom as the first item in the structure item")?;
        match first_item.as_str() {
            "layer"=>{
                let layer = parse_layer(expr_list)?;
                layers.push(layer);
            },
            "boundary" => {
                boundary = Some(parse_boundary(expr_list)?);
            },
            _=> {
                return Err(format!("Unknown structure item: {}", first_item));
            }
        }
    }
    let boundary = boundary.ok_or("Expected a boundary in the structure scope")?;
    Ok(Structure {
        layers,
        boundary,
    })
}

fn parse_placement(s_expr: &Vec<SExpr>) -> Result<Placement, String> {
    // Placeholder for placement parsing logic
    // This function should parse the placement part of the S-expression
    // and populate the DsnStruct accordingly.
    let first_item = s_expr.first().ok_or("Expected at least one item in the placement scope")?;
    let first_item = first_item.as_atom().ok_or("Expected an atom as the first item in the placement scope")?;
    if first_item != "placement" {
        return Err(format!("Expected 'placement' as the first item, found: {}", first_item));
    }
    let mut components: Vec<Component> = Vec::new();
    for item in s_expr.iter().skip(1) {
        let expr_list = item.as_list().ok_or(format!("Expected a list in the placement scope, found: {:?}", item))?;
        let first_item = expr_list.first().ok_or("Expected at least one item in the placement item")?;
        let first_item = first_item.as_atom().ok_or("Expected an atom as the first item in the placement item")?;
        if first_item != "component" {
            return Err(format!("Expected 'component' as the first item in the placement item, found: {}", first_item));
        }
        
    }

    

    todo!("Placement parsing not implemented yet");
}

pub fn parse_s_expr_to_struct(s_expr: &SExpr)->Result<DsnStruct, String>{
    let mut resolution: Option<Resolution> = None;
    let mut structure: Option<Structure> = None;
    let expr_list = s_expr.as_list().ok_or("Expected a list at the top level")?;
    for expr in expr_list {
        let expr_list2 = match expr {
            SExpr::List(list) => list,
            _ => continue,
        };
        let first_item = expr_list2.first().ok_or(format!("Expected at least one item in the outermost scope"))?;
        let first_item = first_item.as_atom().ok_or(format!("Expected an atom as the first item in the outermost scope"))?;
        match first_item.as_str() {
            "parser" => {
                continue;
            },
            "resolution" => {
                let second_item = expr_list2.get(1).ok_or(format!("Expected a second item in the resolution scope"))?;
                let third_item = expr_list2.get(2).ok_or(format!("Expected a third item in the resolution scope"))?;
                let second_item = second_item.as_atom().ok_or(format!("Expected an atom as the second item in the resolution scope"))?;
                let third_item = third_item.as_atom().ok_or(format!("Expected an atom as the third item in the resolution scope"))?;
                let value = third_item.parse::<f64>().map_err(|e| format!("Failed to parse resolution value: {}", e))?;
                resolution = Some(Resolution{
                    unit: second_item.to_string(),
                    value,
                });
            },
            "structure" =>{
                structure = Some(parse_structure(expr_list2)?);
            }
            _ => {
                return Err(format!("Unknown S-expression type: {}", first_item));
            }
        }
    }

    todo!()
}