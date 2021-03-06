#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate sucks2;
use sucks2::
{
  formats::
  {
    mbr::
    {
      MasterBootRecord,
    },
    tinaFS::
    {
      TinaFS,
      User,
    },
  },
};

fn _hexDump
(
  buffer:                               Box<[u8]>,
  width:                                usize,
  offset:                               usize,
  mut length:                           usize,
) -> Result<usize, &'static str>
{
  let   size                            =   buffer.len();
  if offset < size
  {
    if length == 0
    {
      length                            =   size - offset;
    }
    if length <= ( size - offset )
    {
      let   lines                       =   length / width;
      for line                          in  0 .. lines
      {
        for pos                         in  0 .. width
        {
          print!  ( "{:02x} ", buffer [ offset + width * line + pos ] );
        }
        print!  ( "| " );
        for pos                         in  0 .. width
        {
          let char                      =   buffer [ offset + width * line + pos ];
          if  ( char >= 0x20 && char <= 0x7e )
          ||  ( char >= 0xa0 )
          {
            print!  ( "{}", char as char );
          }
          else
          {
            print!  ( "." );
          }
        }
        println!  ( "" );
      }
      let   remainder                   =   length % width;
      if remainder > 0
      {
        for pos                         in  0 .. remainder
        {
          print!  ( "{:02x} ", buffer [ offset + width * lines + pos ] );
        }
        for _                           in  remainder .. width
        {
          print!  ( "   " );
        }
        print!  ( "| " );
        for pos                         in  0 .. remainder
        {
          let char                      =   buffer [ offset + width * lines + pos ];
          if  ( char >= 0x20 && char <= 0x7e )
          ||  ( char >= 0xa0 )
          {
            print!  ( "{}", char as char );
          }
          else
          {
            print!  ( "." );
          }
        }
        println!  ( "" );
      }
      Ok ( length )
    }
    else
    {
      Err ( "Length Out Of Bonds" )
    }
  }
  else
  {
    Err ( "Offset Out Of Bonds" )
  }
}


#[test]
fn main () -> Result<(), &'static str>
{
  let mut myTinaFS
  = TinaFS
    (
      4096,
      8
    );

  let     sivizius
  = User
    (
      "sivizius",
      "Hello World!"
    );

  myTinaFS.addUser ( sivizius ).unwrap();

  let     myMasterBootRecord
  = MasterBootRecord
  (
  );

  myTinaFS.store
  (
    "build/alpha.bin",
    myMasterBootRecord
  );
  Ok(())
}
