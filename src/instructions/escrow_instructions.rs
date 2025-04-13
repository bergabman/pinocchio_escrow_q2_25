use pinocchio::program_error::ProgramError;

#[repr(u8)]
pub enum EscrowInstruction {
    Make,
    Take,
    Refund,
}

impl TryFrom<&u8> for EscrowInstruction {
    type Error = ProgramError;

    fn try_from(byte: &u8) -> Result<Self, Self::Error> {
        match byte {
            0 => Ok(Self::Make),
            1 => Ok(Self::Take),
            2 => Ok(Self::Refund),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
