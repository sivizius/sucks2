#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate sucks2;
use sucks2::
{
  assembly::
  {
    InstructionSet,
    x86::
    {
      X86,
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

  let     myCode
  = X86 ()
    .label("start")
    .add(X86::ax, 42)
    .cmp(X86::ax, -42);

  let mut myAssembly
  = myCode.compile
    (
      InstructionSet::i8086,
      16,
      16,
    ).unwrap();
  
  let bytesPerLine                      =   32;
  let lines                             =   myAssembly.len() / bytesPerLine;
  let rest                              =   myAssembly.len() % bytesPerLine;
  for line in 0..lines
  {
    for offs                            in  0 .. bytesPerLine
    {
      print! ( "{:02x} ", myAssembly[ bytesPerLine * line + offs ] );
    }
    print! ( "| " );
    for offs                            in  0 .. bytesPerLine
    {
      let char                          =   myAssembly[ bytesPerLine * line + offs ];
      if  ( char >= 0x20)
      &&  ( char <= 0x7e )
      {
        print! ( "{}", char as char );
      }
      else
      {
        print! ( "." );
      }
    }
    println! ( "" );
  }
  for offs                              in 0 .. rest
  {
    print! ( "{:02x} ", myAssembly[ bytesPerLine * lines + offs ] );
  }
  for offs                              in 0 .. bytesPerLine - rest
  {
    print! ( "   " );
  }
  print! ( "| " );
  for offs in 0 .. rest
  {
    let char                            =   myAssembly[ bytesPerLine * lines + offs ];
    if  ( char >= 0x20)
    &&  ( char <= 0x7e )
    {
      print! ( "{}", char as char );
    }
    else
    {
      print! ( "." );
    }
  }
  println! ( "" );

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

//Hello World