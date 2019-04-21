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

  let     myAssembly
  = myCode.compile
    (
      InstructionSet::i8086,
      16,
      16,
    ).unwrap();

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