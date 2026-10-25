#[allow(unused_imports)]
use super::*;

#[test]
fn test_instruction_parsing() {
    assert_eq!(InstructionKind::parse("write code for rust function"), InstructionKind::Code);
    assert_eq!(InstructionKind::parse("explain ownership in rust"), InstructionKind::Explain);
    assert_eq!(InstructionKind::parse("write a story about golf"), InstructionKind::Creative);
    assert_eq!(InstructionKind::Code.to_persona_name(), "Coder");
}
