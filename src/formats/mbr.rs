#[repr(C, packed)]
pub struct RawPartition
{
  bootable:                             u8,
  chsFirstSector:                       [u8; 3],
  partitionType:                        u8,
  chsFinalSector:                       [u8; 3],
  lbaAddress:                           u32,
  lbaLength:                            u32,
}

pub enum PartitionType
{
  Empty                                 =   0x00,
  FileAllocationTable12                 =   0x01,
  XenixRoot                             =   0x02,
  XenixUser                             =   0x03,
  FileAllocationTable16                 =   0x04,
  ExtendedBootRecordCHS                 =   0x05,
  FileAllocationTable16b                =   0x06,
  NewTechnologyFileSystem               =   0x07,
  ThisIsNotAFileSystem                  =   0x23,
}

pub struct Partition
{
  bootable:                             bool,
  partitionType:                        PartitionType,
  
}

#[repr(C, packed)]
pub struct RawMasterBootRecord
{
  content:                              [u8; 512 - 18],
  partitions:                           [RawPartition; 4],
  magic:                                u16,
}

pub struct MasterBootRecord
{
  ctrPartition:                         usize,
  hazBootable:                          bool,
  tblPartition:                         Vec<Partition>
}

pub fn MasterBootRecord
(
) -> MasterBootRecord
{
  MasterBootRecord
  {
    ctrPartition:                       0,
    hazBootable:                        false,
    tblPartition:                       vec!(),
  }
}

impl MasterBootRecord
{
  pub fn bootPartition
  (
    &mut self,
    partitionType:                      PartitionType,
  ) -> Result<usize, &str>
  {
    if      self.hazBootable
    {
      Err ( "there is already a bootable partition in this master boot record" )
    }
    else if self.ctrPartition >= 4
    {
      Err ( "there are already four partiotions in this master boot record" )
    }
    else
    {
      self.tblPartition.push
      (
        Partition
        {
          bootable:                     true,
          partitionType:                partitionType,
        }
      );
      self.ctrPartition                 +=  1;
      Ok ( self.ctrPartition - 1 )
    }
  }  
}