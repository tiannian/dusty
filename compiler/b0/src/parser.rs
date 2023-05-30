use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../../pest/b0-ir.pest"]
struct DusyIRParser;
