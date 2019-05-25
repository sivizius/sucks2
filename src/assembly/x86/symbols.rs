pub use super::
{
  instructions::
  {
    InstructionAddress,
  },
  operands::
  {
    Operand,
    OperandType,
  },
};

use std::
{
  collections::
  {
    HashMap,
  },
};

pub struct Symbol                       ( pub SymbolIdentifier  );

impl  Operand                           for Symbol
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::Symbol  ( self.0 ), 0 ) }
}

pub struct  SymbolEntry
{
  title:                                SymbolIdentifier,
  first:                                Option<OperandType>,
  value:                                Option<OperandType>,
  round:                                usize,
}

pub type    SymbolIdentifier            =   &'static  str;

pub struct  SymbolList
{
  hashMap:                              HashMap<SymbolIdentifier, SymbolReference>,
  symbols:                              Vec<SymbolEntry>,
}

impl        SymbolList
{
  pub fn assign
  (
    &mut self,
    title:                              SymbolIdentifier,
    value:                              Option<OperandType>,
    round:                              usize,
  ) ->  SymbolReference
  {
    if let Some ( refer ) = self.hashMap.get  ( title )
    {
      if self.symbols [ *refer  ].first.is_none ( )
      {
        self.symbols  [ *refer  ].first =   value.clone ( );
      }
      self.symbols  [ *refer  ].value   =   value;
      self.symbols  [ *refer  ].round   =   round;
      *refer
    }
    else
    {
      let refer                         =   self.symbols.len  ( );
      self.symbols.push
      (
        SymbolEntry
        {
          title:                        title,
          first:                        value.clone ( ),
          value:                        value,
          round:                        round,
        }
      );
      self.hashMap.insert
      (
        title,
        refer,
      );
      refer
    }
  }
  pub fn define
  (
    &mut self,
    title:                              SymbolIdentifier,
    value:                              Option<OperandType>,
    round:                              usize,
  ) ->  Result<SymbolReference, &'static str>
  {
    if let Some ( refer ) = self.hashMap.get  ( title )
    {
      if self.symbols [ *refer  ].first.is_none ( )
      {
        self.symbols  [ *refer  ].first =   value.clone ( );
        self.symbols  [ *refer  ].value =   value;
        self.symbols  [ *refer  ].round =   round;
        Ok  ( *refer  )
      }
      else
      {
        Err ( "Symbol already defined" )
      }
    }
    else
    {
      let refer                         =   self.symbols.len  ( );
      self.symbols.push
      (
        SymbolEntry
        {
          title:                        title,
          first:                        value.clone ( ),
          value:                        value,
          round:                        round,
        }
      );
      self.hashMap.insert
      (
        title,
        refer,
      );
      Ok  ( refer )
    }
  }
  pub fn expect
  (
    &mut self,
    title:                              SymbolIdentifier,
  ) ->  SymbolReference
  {
    if let Some ( refer ) = self.hashMap.get  ( title )
    {
      *refer
    }
    else
    {
      let refer                         =   self.symbols.len  ( );
      self.symbols.push
      (
        SymbolEntry
        {
          title:                        title,
          first:                        None,
          value:                        None,
          round:                        0,
        }
      );
      self.hashMap.insert
      (
        title,
        refer,
      );
      refer
    }
  }
  pub fn modify
  (
    &mut self,
    refer:                              SymbolReference,
    value:                              Option<OperandType>,
    round:                              usize,
  ) -> Option<&'static str>
  {
    if refer  < self.symbols.len  ( )
    {
      if self.symbols [ refer ].first.is_none ( )
      {
        self.symbols  [ refer ].first   =   value.clone ( );
      }
      self.symbols  [ refer ].value     =   value;
      self.symbols  [ refer ].round     =   round;
      None
    }
    else
    {
      Some  ( "Invalid Reference" )
    }
  }
  pub fn myName
  (
    &mut self,
    refer:                              SymbolReference,
  ) ->  Result<String, &'static str>
  {
    if refer  < self.symbols.len  ( )
    {
      Ok  ( self.symbols  [ refer ].title.to_string ( ) )
    }
    else
    {
      Err  ( "Invalid Reference" )
    }
  }
  pub fn obtain
  (
    &self,
    refer:                              SymbolReference,
    round:                              usize,
  ) ->  Result<Option<OperandType>, &'static str>
  {
    if refer  < self.symbols.len  ( )
    {
      if  self.symbols  [ refer ].round ==  round
      {
        Ok  ( self.symbols  [ refer ].value.clone ( ) )
      }
      else
      {
        Ok  ( self.symbols  [ refer ].first.clone ( ) )
      }
    }
    else
    {
      Err  ( "Invalid Reference" )
    }
  }
}

pub fn      SymbolList
(
) ->  SymbolList
{
  SymbolList
  {
    hashMap:                            HashMap::new  ( ),
    symbols:                            vec!  ( ),
  }
}

pub type    SymbolReference             =   usize;
