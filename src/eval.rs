
fn eval(node: NodeType) {
    match node {
        NodeType::LetStatement(lt) => { 
            if let ExpressionType::Int(i) = lt.value {
                println!("{} = {}", lt.name.0, i.0);
            }
        },
        _ => {}
    }
}

