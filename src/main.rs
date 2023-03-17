extern crate getopts;
extern crate chrono;
extern crate snailcrypt;

use core::cmp::Ordering;
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
    Local,
};	
use url::form_urlencoded;

const URL_MAX_LEN: usize = 8000;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version(program: &str) {
    println!("{} is version 0.2.0
Copyright by Richard Bäck 2023

If you have not received a copy of the EULA you can access it by downloading 
snailcrypt-cli at https://www.snailcrypt.com#download", program);
}

fn encrypt(lockdate_str: &str, 
		   hint: &str,
		   generate_url: bool,
		   force_lockdate: bool,
		   force_url_length: bool,		   
		   mut in_descriptor: Box<dyn Read>,
		   mut out_descriptor: Box<dyn Write>) 
	-> i32 {
	//=========================================================================
	// Setup version to use
	let mut client_version = client::ClientVersion::V1;
	if hint.len() > 0 {
		client_version = client::ClientVersion::V2;
	}
	
	//=========================================================================
	// Setup client object
	let analyzer_factory: factory::AnalyzerFactory = 
		factory::AnalyzerFactory::new(); 		
	let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
	let client_factory: factory::ClientFactory = 
		factory::ClientFactory::new(analyzer_factory, 
									config_factory);
    let client: Box<dyn client::Client> = client_factory
    									  .create(client_version);
       
    //=========================================================================
    // Parse lock date
    let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str(&lockdate_str,
	                                                               client.get_datetime_format())
	    .unwrap_or_else(|_error| {
	    eprintln!("Error: unable to parse the lock date \"{}\"", lockdate_str);
	    exit(1);
	});
	
	//=========================================================================
	// Exit on lockdate in the past
	let date_now: DateTime<FixedOffset> = Local::now()
									.with_timezone(
										&FixedOffset::east_opt(0)
										.unwrap_or_else(|| {
											panic!("Error: unexpected error during conversion of current date time.");									
										}));
	if force_lockdate == false && date_now.cmp(&lockdate) != Ordering::Less {
		eprintln!("Error: lock date \"{}\" is in the past.",		
				  lockdate.format(client.get_datetime_format()).to_string());
		return 1;
	}
	
	//=========================================================================
	// Retrieve plaintext
	let mut plaintext: String = String::new();
	in_descriptor.read_to_string(&mut plaintext).unwrap_or_else(|error| {
		panic!("Error: {:?}", error);
	});
	 
 	//=========================================================================
 	// Encrypt plaintext
 	let ciphertext_result: Result<String, String> =
 		client.encrypt(&client::ClientEncryptArg { 
 			plaintext,
 			lockdate,
 			hint: String::from(hint) 
 		});
 	if ciphertext_result.is_err() {
 		eprintln!("{}", ciphertext_result.unwrap_err());
 		return 1;
 	}
 	let mut ciphertext = ciphertext_result.unwrap();
 	
	//=========================================================================
	// Generate URL
	if generate_url == true {
		ciphertext = form_urlencoded::Serializer::new(String::new())
                            .append_pair("c",
                                         ciphertext.as_str())
                            .finish();
		ciphertext.insert_str(0, "https://webapp.snailcrypt.com/timer.php?");
		if ciphertext.len() > URL_MAX_LEN
			&& force_url_length == false {
			eprintln!("Error: the generated URL is longer than {} characters", URL_MAX_LEN);
			return 1;
		}
	}
	
	//=========================================================================
	// Write ciphertext
	out_descriptor.write_all(ciphertext.as_bytes())
				  .unwrap_or_else(|error| {
		panic!("Error: {:?}", error);
	});
	
	return 0;
}

fn decrypt(extract_hint: bool,
	       mut in_descriptor: Box<dyn Read>,
		   mut out_descriptor: Box<dyn Write>)
	-> i32 {
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
	// Decrypt ciphertext but do not retrieve anything from the result   
    let decryption_result = client
   		.decrypt(ciphertext.as_str());

    let plaintext: String;
    if extract_hint == true {
    	//=====================================================================
    	// Retrieve hint
    	if decryption_result.is_err() {
    		plaintext = decryption_result.unwrap_err().hint;
    	} else {
    		plaintext = decryption_result.unwrap().hint;
    	}
    } else {
		//=====================================================================
		// Retrieve ciphertext    
	    plaintext = decryption_result
			.unwrap_or_else(|error| {
			    panic!("Error: {:?}", error.error_message);
			})
			.plaintext;
	}
	
	//=========================================================================
	// Write plaintext
	out_descriptor.write_all(plaintext.as_bytes()).unwrap_or_else(|error| {
		panic!("Error: {:?}", error);
	});
	
	return 0;
}

fn main() {
    //=========================================================================
    // Setup argument parsing and the available options
	let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("d", "decrypt",       "Decrypts a string");
    opts.optopt( "e", "encrypt",       "Encrypts a string using the given lock date (e.g. \"2023-01-31T23:00:00+0000\"", "LOCK_DATE");
    opts.optopt( "t", "hint",          "Use string as hint for the encrypted string. This option is only used for -e.", "HINT");
    opts.optflag( "T", "extract-hint", "Extracts hint from encrypted string. This option is only used for -d.");
    opts.optopt( "i", "input",         "Use input file instead of stdin", "INPUT_FILE");    
    opts.optopt( "o", "stdout",        "Use input file instead of stdout", "OUTPUT_FILE");    
    opts.optflag("f", "force",         "Use the force and ignore any warnings. Those include:
- Ignore using a lock date in the past (option -e). This might still fail if the server rejects the request.
- Ignore the URL limit on URL generation (option -u)");
    opts.optflag("u", "url",           "Generate a URL pointing to a timer containing the message on https://webapp.snailcrypt.com. This is an option for -e.");
    opts.optflag("h", "help",          "Print this help");
    opts.optflag("V", "version",       "Print version");
    
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
	
	//=========================================================================
	// Set hint
	let hint = match matches.opt_str("t") {
		Some(hint) => hint,
		None => String::from(""),
	};
	
	//=========================================================================
	// Set extract hint flag
	let mut extract_hint: bool = false;
	if matches.opt_present("T") {
		extract_hint = true;
	}
	
	//=========================================================================
	// Set URL generation flag
	let mut generate_url: bool = false;
	if matches.opt_present("u") {
		generate_url = true;
	}
	
	//=========================================================================
	// Set force flag
	let mut force_lockdate: bool = false;
	let mut force_url_length: bool = false;
	if matches.opt_present("f") {
		force_lockdate = true;
		force_url_length = true;
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
		
		exit(encrypt(&lockdate_str,
						  hint.as_str(),
						  generate_url, 
					      force_lockdate,				
					      force_url_length,	  
					      in_descriptor, 
					      out_descriptor));
	} else if matches.opt_present("d") {
		//=====================================================================
		// Perform decryption
		exit(decrypt(extract_hint,
		   in_descriptor, 
						  out_descriptor));
	} else {
		//=====================================================================
		// Error: neither option is present
		eprintln!("Neither option 'e' nor option 'd' is present. Select one.");
		println!("");
		print_usage(&program, opts);
        exit(1);		
	}
}
