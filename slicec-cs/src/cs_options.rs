// Copyright (c) ZeroC, Inc. All rights reserved.

use slice::options::SliceOptions;
use structopt::StructOpt;

// Note: StructOpt uses the doc-comments of fields to populate the '--help' output of slicec-cs.
//       boolean flags automatically default to false, and strings automatically default to empty.

/// This struct is responsible for parsing the command line options specific to 'slicec-cs'.
/// The option parsing methods are automatically generated for the struct by the `StructOpt` crate.
#[derive(StructOpt, Debug)]
#[structopt(name = "slicec-cs", version = "0.1.0", rename_all = "kebab-case", about = ABOUT_STRING)]
pub struct CsOptions {
    // Import the options common to all slice compilers.
    #[structopt(flatten)]
    pub slice_options: SliceOptions,
}

/// Short description of slicec-cs that is displayed in it's help dialogue.
const ABOUT_STRING: &str = "The slice compiler for C#.
                            Generates C# code from slice files for use with icerpc.";
