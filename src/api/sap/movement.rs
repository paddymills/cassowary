
/// SAP movement type
/// 
/// id: 3-digit movement type
/// project: if this is a Q-variant
/// 
/// example: 261Q -> MovementType { id: 261, project: true }
#[derive(Debug)]
pub struct MovementType {
    id: u16,
    project: bool
}


