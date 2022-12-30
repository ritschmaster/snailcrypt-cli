extern crate getopts;
extern crate chrono;
extern crate snailcrypt;

use std::{
	env,
	fs::File,
	io::{
		Read,
		stdin,
		stdout,
		Write,
	},
	process::exit,
};

use getopts::Options;
use snailcrypt::{
	util,
	client,
	factory,
};
use chrono::{
        DateTime,
        FixedOffset,
};	

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version(program: &str) {
    println!("{} is version 0.1.0", program);
}

fn encrypt(lockdate_str: &str, 
		   mut in_descriptor: Box<dyn Read>,
		   mut out_descriptor: Box<dyn Write>) {
	//=========================================================================
	// Setup client object
	let analyzer_factory: factory::AnalyzerFactory = 
		factory::AnalyzerFactory::new(); 		
	let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
	let client_factory: factory::ClientFactory = 
		factory::ClientFactory::new(analyzer_factory, 
									config_factory);
    let client: Box<dyn client::Client> = client_factory
    									  .create(client::ClientVersion::V1);
    
    //=========================================================================
    // Parse lock date
    let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str(&lockdate_str,
	                                                               client.get_datetime_format())
	    .unwrap_or_else(|error| {
	    panic!("Error: {:?}", error);
	});
	
	//=========================================================================
	// Retrieve plaintext
	let mut plaintext: String = String::new();
	in_descriptor.read_to_string(&mut plaintext).unwrap_or_else(|error| {
		panic!("Error: {:?}", error);
	});
	 
 	//=========================================================================
 	// Encrypt plaintext
    let ciphertext: String = client
    						 .encrypt(plaintext.as_str(), 
	    							  lockdate)
    						 .unwrap_or_else(|error| {
	    panic!("Error: {:?}", error);
	}); 	   
	
	//=========================================================================
	// Write ciphertext
	out_descriptor.write_all(ciphertext.as_bytes())
				  .unwrap_or_else(|error| {
		panic!("Error: {:?}", error);
	});
}

fn decrypt(mut in_descriptor: Box<dyn Read>,
		   mut out_descriptor: Box<dyn Write>) {
	//=========================================================================
	// Retrieve ciphertext
	let mut ciphertext: String = String::new();
	in_descriptor.read_to_string(&mut ciphertext).unwrap_or_else(|error| {
		panic!("Error: {:?}", error);
	});			
			
	//=========================================================================
	// Setup analyzer object
	let analyzer_factory: factory::AnalyzerFactory = 
		factory::AnalyzerFactory::new();
	let analyzer: Box<dyn util::Analyzer> = analyzer_factory.create();
	
	//=========================================================================
	// Setup client object 		
	let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
	let client_factory: factory::ClientFactory = 
		factory::ClientFactory::new(analyzer_factory, 
									config_factory);
	let client_version: client::ClientVersion = 
		analyzer.get_version(ciphertext.as_str())								
				.unwrap_or_else(|error| {
					panic!("Error: {:?}", error);			
				});
	let client: Box<dyn client::Client> = client_factory
    									  .create(client_version);
    
	//=========================================================================
	// Decrypt ciphertext    
    let plaintext: String = client
    						.decrypt(ciphertext.as_str())
    						.unwrap_or_else(|error| {
	    panic!("Error: {:?}", error);
	});
	
	//=========================================================================
	// Write plaintext
	out_descriptor.write_all(plaintext.as_bytes()).unwrap_or_else(|error| {
		panic!("Error: {:?}", error);
	});
}

fn main() {
    //=========================================================================
    // Setup argument parsing and the available options
	let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("d", "decrypt", "Decrypts a string");
    opts.optopt( "e", "encrypt", "Encrypts a string using the given lock date", "LOCK_DATE");    
    opts.optopt( "i", "input",   "Use input file instead of stdin", "INPUT_FILE");    
    opts.optopt( "o", "stdout",  "Use input file instead of stdout", "OUTPUT_FILE");    
//    opts.optflag("f", "force",   "Force using a lock date in the past");
    opts.optflag("h", "help",    "Print this help");
    opts.optflag("V", "version", "Print version");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { 
			eprintln!("{}", f.to_string());
			println!("");
			print_usage(&program, opts);
			return;
		},
    };	
    
    if matches.opt_present("h") {
	    //=====================================================================
	    // Print the help text
        print_usage(&program, opts);
        return;
    }    
    
    if matches.opt_present("V") {
	    //=====================================================================
	    // Print the version
        print_version(&program);
        return;
    }    
    
    //=========================================================================
    // Setup where to get the plaintext or ciphertext from
    let mut in_descriptor: Box<dyn Read> = Box::new(stdin());
    if matches.opt_present("i") {
		let input_filename = match matches.opt_str("i") {
			Some(input_filename) => input_filename,
			None => {
				print_usage(&program, opts);
				return;
			},		
		};
	
		in_descriptor = Box::new(File::open(input_filename)
								 .unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		}))
	}
	
	//=========================================================================
	// Setup where to write the ciphertext or plaintext to
	let mut out_descriptor: Box<dyn Write> = Box::new(stdout());
	if matches.opt_present("o") {
		let output_filename = match matches.opt_str("o") {
			Some(output_filename) => output_filename,
			None => {
				print_usage(&program, opts);
				return;
			},		
		};
	
		out_descriptor = Box::new(File::create(output_filename)
								 .unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		}))	
	}
	
	if matches.opt_present("e") && matches.opt_present("d") {
		//=====================================================================
		// Error: both options are present
		eprintln!("Option 'e' and option 'd' are present. Select one.");
		println!("");
		print_usage(&program, opts);
		exit(1);	
	} else if matches.opt_present("e") {
		//=====================================================================
		// Perform encryption
		let lockdate_str = match matches.opt_str("e") {
			Some(lockdate_str) => lockdate_str,
			None => {
				print_usage(&program, opts);
				return;
			},		
		};    
		
		encrypt(&lockdate_str, in_descriptor, out_descriptor);
	} else if matches.opt_present("d") {
		//=====================================================================
		// Perform decryption
		decrypt(in_descriptor, out_descriptor);
	} else {
		//=====================================================================
		// Error: neither option is present
		eprintln!("Neither option 'e' nor option 'd' is present. Select one.");
		println!("");
		print_usage(&program, opts);
        exit(1);		
	}
}
