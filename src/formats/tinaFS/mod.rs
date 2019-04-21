pub mod user;
pub use super::
{
  super::
  {
    crypto::
    {
      Encryption,
    },
  },
  mbr::
  {
    MasterBootRecord,
    Partition,
    PartitionType,
    RawMasterBootRecord,
    RawPartition,
  },
  uf4::
  {
    Yapter,
  },
  tinaFS::
  {
    user::
    {
      User,
      UserID,
    },
  },
};

use std::
{
  io::
  {
    SeekFrom,
    prelude::*,
  },
  fs::
  {
    File,
  },
};

pub struct TinaFS
{
  chunkSize:                            usize,
  countChunks:                          usize,
  emptyChunks:                          usize,
  lstUsers:                             Vec<User>,
}

pub fn TinaFS
(
  chunkSize:                            usize,
  countChunks:                          usize,
) -> TinaFS
{
  let emptyChunks                       =   countChunks                                                       //  number of chunks for
                                        -   0                                                                 //  * master boot record, stage0, stage1, …
                                        -   0                                                                 //  * user table
                                        -   ( countChunks + 8 * chunkSize - 1 ) / ( 8 * chunkSize )           //  * public bit map
                                        +   0;
  TinaFS
  {
    chunkSize:                          chunkSize,
    countChunks:                        countChunks,
    emptyChunks:                        emptyChunks,
    lstUsers:                           vec!(),
  }
}

impl TinaFS
{
  pub fn addUser
  (
    &mut self,
    user:                               User,
  ) -> Result<UserID, &str>
  {
    self.lstUsers.push( user );
    Ok( UserID::from( self.lstUsers.len() ) )
  }
  pub fn store
  (
    &mut self,
    fileName:                           &str,
    masterBootRecord:                   MasterBootRecord,
  )
  {
    //  create public bit map
    let mut pubBitMap: Vec<u8>          =   Vec::with_capacity( ( self.countChunks + 7 ) / 8 );
    pubBitMap.resize( ( self.countChunks + 7 ) / 8 , 0 );
    let mut pubBitMap: Box<[u8]>        =   pubBitMap.into_boxed_slice();

    //  master boot record
    

    //  create output file
    let mut file                        =   File::create    ( fileName ).unwrap();
    file.set_len  ( ( self.chunkSize * self.countChunks ) as u64 ).unwrap();
    file.seek     ( SeekFrom::Start ( ( 0 * self.chunkSize ) as u64 ) ).unwrap();
    

  }
}
