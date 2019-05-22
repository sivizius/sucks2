#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[macro_use]
extern crate sucks2;
use sucks2::
{
  assembly::
  {
    InstructionSet,
    x86::
    {
      X86,
      expressions::
      {
        Expression,
        ExpressionToken,
      },
      memory::
      {
        Memory16,
        Memory16Registers,
      },
      registers::
      {
        SegmentRegisterNumber,
      },
    },
  },
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

use std::
{
  fs::
  {
    File,
  },
  io::
  {
    Write,
  },
  process::
  {
    Command,
  },
};

fn hexDump
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
  let     myCode
  = X86 ()
    .label( "simple math instruction 8 bit"                                                         ) 
    .add  ( X86::cl,                                X86::dl                                         ) //  Register  to  Register
    .add  ( x86Mem16! ( byte [ bp si 0x80 - + ] ),  X86::dl                                         ) //  Register  to  Memory
    .add  ( X86::dl,                                x86Mem16! ( byte [ bp si 0x80 - + ] )           ) //  Memory    to  Register
    .add  ( X86::cl,                                0x90                                            ) //  Immediate to  Register
    .add  ( x86Mem16! ( byte [ bp si 0x80 - + ] ),  0x42                                            ) //  Immediate to  Memory
    .add  ( X86::al,                                0x23                                            ) //  Immediate to  Accumulator
    .label( "simple math instruction 16 bit"                                                        ) 
    .add  ( X86::cx,                                X86::dx                                         ) //  Register  to  Register
    .add  ( x86Mem16! ( word [ bp si 0x80 - + ] ),  X86::dx                                         ) //  Register  to  Memory
    .add  ( X86::dx,                                x86Mem16! ( word [ bp si 0x80 - + ] )           ) //  Memory    to  Register
    .add  ( X86::cx,                                0x32                                            ) //  Immediate to  Register  Sign Extended
    .add  ( X86::cx,                                0x9000                                          ) //  Immediate to  Register
    .add  ( x86Mem16! ( word [ bp si 0x80 - + ] ),  0x42                                            ) //  Immediate to  Memory    Sign Extended
    .add  ( x86Mem16! ( word [ bp si 0x80 - + ] ),  0x1337                                          ) //  Immediate to  Memory
    .add  ( X86::ax,                                0x1337                                          ) //  Immediate to  Accumulator
    .label( "one byte instructions"                                                                 ) 
    .iret (                                                                                         )
    ;

  let mut myAssembly
  = myCode.compile
    (
      InstructionSet::i8086,
      16,
      16,
    ).unwrap();

  let mut file                          =   File::create  ( "build/8086.bin"  ).unwrap();
  file.write_all  ( &myAssembly ).unwrap();
  file.sync_all   (             ).unwrap();

  let mut cmdDisasm                     =   Command::new("objdump")
                                            .arg("--disassemble-all")
                                            .arg("--disassembler-options=intel")
                                            .arg("--target=binary")
                                            .arg("--architecture=i8086")
                                            .arg("build/8086.bin")
                                            .status()
                                            .unwrap();
  Ok(())
}
