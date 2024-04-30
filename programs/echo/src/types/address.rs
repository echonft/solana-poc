use anchor_lang::prelude::*;
use ethereum_types::H160;
use std::io;
use std::io::{Read, Write};
use wormhole_anchor_sdk::wormhole::constants::CHAIN_ID_SOLANA;

#[derive(Clone, Debug, PartialEq)]
pub struct Address {
    pub chain_id: u16,
    pub solana_address: Option<Pubkey>,
    pub eth_address: Option<ethereum_types::Address>,
}

pub const ADDRESS_SERIALIZED_SIZE: usize = 34;

impl AnchorSerialize for Address {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write(&self.chain_id.to_be_bytes())?;
        match self.chain_id {
            CHAIN_ID_SOLANA => {
                let address = self.solana_address.ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Solana chain id but address not present",
                ));
                address.unwrap().serialize(writer)?;
                Ok(())
            }
            2 => {
                let address = self.eth_address.ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Ethereum chain id but address not present",
                ));
                // write 12 padding bytes since Ethereum's addresses are 20 bytes
                writer.write(&[0; 12])?;
                writer.write(&address.unwrap().to_fixed_bytes())?;
                Ok(())
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unsupported chain id ({})", self.chain_id),
            )),
        }
    }
}

impl AnchorDeserialize for Address {
    fn deserialize_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut data = [0; ADDRESS_SERIALIZED_SIZE];
        let size = reader.read(&mut data)?;
        if size != ADDRESS_SERIALIZED_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("wrong size: expected 34 bytes but size is {}", size),
            ));
        }
        let mut buffer = [0; 2];
        buffer.copy_from_slice(&data[0..2]);
        let chain_id = u16::from_be_bytes(buffer);
        match chain_id {
            CHAIN_ID_SOLANA => {
                let address = Pubkey::try_from_slice(&data[2..])?;
                Ok(Address {
                    chain_id,
                    solana_address: Some(address),
                    eth_address: None,
                })
            }
            2 => {
                let mut buffer = [0; 20];
                buffer.copy_from_slice(&data[14..]);
                let address = H160::try_from(&buffer).unwrap();
                Ok(Address {
                    chain_id,
                    solana_address: None,
                    eth_address: Some(address),
                })
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unsupported chain id ({})", chain_id),
            )),
        }
    }
}
