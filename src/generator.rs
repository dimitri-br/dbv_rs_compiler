use super::parser::ASTNode;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate(nodes: Vec<ASTNode>) -> Vec<u8> {
        nodes.iter().flat_map(|node| {
            let mut bytecode = vec![node.op_code];
            bytecode.extend(&node.args);
            bytecode
        }).collect()
    }
}