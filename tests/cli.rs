
use assert_cmd::Command;

#[test]
fn encrypt_small_str() -> Result<(), Box<dyn std::error::Error>> {
	let plaintext: String = String::from("hello world");
	
	//=========================================================================
	// Perform encryption
    let mut cmd_encrypt = Command::cargo_bin("snailcrypt-cli")
							.unwrap_or_else(|error| {
								panic!("Error: {:?}", error);
							});

    let encrypted = cmd_encrypt
    						   .arg("-e")
							   .arg("2022-11-19T17:00:00+0100")			   
							   .arg("-f")
							   .write_stdin(plaintext.as_str())
							   .assert()
							   .success();
	let ciphertext: Vec<u8> = encrypted.get_output().stdout.to_owned();
	
	//=========================================================================
	// Perform decryption		   
	let mut cmd_decrypt = Command::cargo_bin("snailcrypt-cli")
							.unwrap_or_else(|error| {
								panic!("Error: {:?}", error);
							});
	
	
	cmd_decrypt.arg("-d")
			   .write_stdin(ciphertext)
			   .assert()
			   .stdout(plaintext);

    Ok(())
}

#[test]
fn encrypt_large_str() -> Result<(), Box<dyn std::error::Error>> {
	let plaintext: String = 
		String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo, 
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.  
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.  
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada 
massa, quis varius mi purus non odio.  Pellentesque condimentum, 
magna ut suscipit hendrerit, ipsum augue ornare nulla, non 
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum 
lorem.  Fusce sagittis, libero non molestie mollis, magna orci 
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");
	
	//=========================================================================
	// Perform encryption
    let mut cmd_encrypt = Command::cargo_bin("snailcrypt-cli")
							.unwrap_or_else(|error| {
								panic!("Error: {:?}", error);
							});

    let encrypted = cmd_encrypt
    						   .arg("-e")
							   .arg("2022-11-19T17:00:00+0100")			   
							   .arg("-f")
							   .write_stdin(plaintext.as_str())
							   .assert()
							   .success();
	let ciphertext: Vec<u8> = encrypted.get_output().stdout.to_owned();
	
	//=========================================================================
	// Perform decryption		   
	let mut cmd_decrypt = Command::cargo_bin("snailcrypt-cli")
							.unwrap_or_else(|error| {
								panic!("Error: {:?}", error);
							});
	
	
	cmd_decrypt.arg("-d")
			   .write_stdin(ciphertext)
			   .assert()
			   .stdout(plaintext);

    Ok(())
}

#[test]
fn encrypt_fail_lockdate() -> Result<(), Box<dyn std::error::Error>> {
	let plaintext: String = String::from("hello world");
	
	//=========================================================================
	// Perform encryption
    let mut cmd_encrypt = Command::cargo_bin("snailcrypt-cli")
							.unwrap_or_else(|error| {
								panic!("Error: {:?}", error);
							});

    cmd_encrypt
    		.arg("-e")
			.arg("2022-11-19T17:00:00+0100")			   
			.write_stdin(plaintext.as_str())
			.assert()
			.failure()
			.stderr("Error: lock date \"2022-11-19T17:00:00+0100\" is in the past.\n");

    Ok(())
}